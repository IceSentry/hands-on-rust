#![allow(unused)]

use self::settings::AsciiTilemapSettings;
use bevy::{
    asset::AssetPath,
    ecs::system::SystemParam,
    prelude::*,
    utils::{HashMap, HashSet, Instant},
};
use bevy_ecs_tilemap::{
    Chunk, LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TileParent, TilemapPlugin,
};

pub mod color;
pub mod geometry;
pub mod settings;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub struct Drawing;

pub struct ActiveLayer(u32);

// TODO support per layer tilesheet
#[derive(Clone, Copy)]
pub struct Layer {
    background_id: u16,
    foreground_id: u16,
    is_transparent: bool,
    is_glyph_background_transparent: bool,
}

impl Layer {
    fn new(layer_id: u8, is_transparent: bool, is_glyph_background_transparent: bool) -> Self {
        let real_layer = u16::from(layer_id * 2);
        Self {
            background_id: real_layer,
            foreground_id: real_layer + 1,
            is_transparent,
            is_glyph_background_transparent,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileInfo {
    color: Color,
    glyph: char,
}

#[derive(Debug)]
pub enum DrawCommand {
    DrawTile {
        layer_id: u16,
        position: UVec2,
        tile_info: TileInfo,
    },
    ClearLayer {
        layer_id: u16,
        color: Color,
    },
    ClearAllLayers {
        color: Color,
    },
}
type CommandBuffer = Vec<DrawCommand>;
type Layers = Vec<Vec<TileInfo>>;

pub struct AsciiTilemapPlugin;

impl Plugin for AsciiTilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TilemapPlugin)
            .add_system_set(
                SystemSet::new()
                    .label(Drawing)
                    .with_system(process_command_buffer.system().before("draw"))
                    .with_system(draw.system().label("draw")),
            )
            .add_startup_system(setup.system())
            .insert_resource(ActiveLayer(0))
            .insert_resource(vec![] as CommandBuffer);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<AsciiTilemapSettings>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    info!("initializing ascii_tilemap_plugin...");
    let start = Instant::now();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load(settings.tilesheet_asset_path.as_str());
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0_u16, map_entity);

    let layer_settings = LayerSettings::new(
        UVec2::new(settings.horizontal_chunks, settings.vertical_chunks),
        UVec2::new(
            settings.width / settings.horizontal_chunks,
            settings.height / settings.vertical_chunks,
        ),
        Vec2::new(settings.tile_width as f32, settings.tile_height as f32),
        Vec2::new(
            (settings.tilesheet_width * settings.tile_width) as f32,
            (settings.tilesheet_height * settings.tile_height) as f32,
        ),
    );

    let mut build_layer = |layer_id| {
        let (mut layer_builder, layer_entity) =
            LayerBuilder::new(&mut commands, layer_settings, 0_u16, layer_id, None);
        layer_builder.set_all(TileBundle::default());
        map_query.build_layer(&mut commands, layer_builder, material_handle.clone());
        map.add_layer(&mut commands, layer_id, layer_entity);
    };

    for layer in &settings.layers {
        build_layer(layer.background_id);
        build_layer(layer.foreground_id);
    }

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(
            -(settings.window_width() / 2.),
            -(settings.window_height() / 2.),
            0.0,
        ))
        .insert(GlobalTransform::default());

    // setup internal map representation
    let mut layers = vec![
        vec![
            TileInfo {
                color: Color::BLACK,
                glyph: ' '
            };
            (settings.width * settings.height) as usize
        ];
        settings.layers.len() * 2
    ];
    commands.insert_resource(layers as Layers);

    info!(
        "initializing ascii_tilemap_plugin...done {:?}",
        start.elapsed()
    );
}

fn process_command_buffer(
    mut command_buffer: ResMut<CommandBuffer>,
    settings: Res<AsciiTilemapSettings>,
    mut layers: ResMut<Layers>,
) {
    puffin::profile_function!();

    // Use an internal representation of the map to do all the operations.
    // Once it's done, send the end result to the tilemap
    for command in command_buffer.iter() {
        match *command {
            DrawCommand::DrawTile {
                layer_id,
                position,
                tile_info,
            } => {
                let index = (position.y * settings.width + position.x) as usize;
                let mut tile = &mut layers[layer_id as usize][index];
                *tile = tile_info;
            }
            DrawCommand::ClearLayer { layer_id, color } => {
                let layer_id = layer_id as usize;
                let layer_setting = settings.layers[get_layer_setting_index(layer_id)];
                clear_layer(&mut layers[layer_id], layer_id, layer_setting, color);
            }
            DrawCommand::ClearAllLayers { color } => {
                for (layer_id, layer) in layers.iter_mut().enumerate() {
                    let layer_setting = settings.layers[get_layer_setting_index(layer_id)];
                    clear_layer(layer, layer_id, layer_setting, color);
                }
            }
        }
    }
    command_buffer.clear();
}

fn get_layer_setting_index(layer_id: usize) -> usize {
    if layer_id % 2 == 0 {
        layer_id / 2
    } else {
        (layer_id - 1) / 2
    }
}

fn clear_layer(layer: &mut Vec<TileInfo>, layer_id: usize, layer_setting: Layer, color: Color) {
    for mut tile in layer {
        tile.glyph =
            if layer_id == layer_setting.background_id as usize && !layer_setting.is_transparent {
                219 as char // ASCII code 219 = █ ( Block, graphic character )
            } else {
                0 as char // foreground and transparent backgrounds should be invisible after clear
            };
        tile.color = color;
    }
}

fn draw(
    mut chunk_query: Query<&mut Chunk>,
    mut tile_query: Query<(&mut Tile, &TileParent, &UVec2)>,
    layers: Res<Layers>,
    settings: Res<AsciiTilemapSettings>,
) {
    puffin::profile_function!();

    let mut chunks = HashSet::default();
    tile_query.for_each_mut(|(mut tile, tile_parent, pos)| {
        let index = (pos.y * settings.width + pos.x) as usize;
        let tile_info = layers[tile_parent.layer_id as usize][index];

        if tile.texture_index != tile_info.glyph as u16 || tile.color != tile_info.color {
            tile.texture_index = tile_info.glyph as u16;
            tile.color = tile_info.color;
            chunks.insert(tile_parent.chunk);
        }
    });

    for chunk_entity in chunks.drain() {
        if let Ok(mut chunk) = chunk_query.get_mut(chunk_entity) {
            chunk.needs_remesh = true;
        }
    }
}

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    commands: Commands<'a>,
    settings: Res<'a, AsciiTilemapSettings>,
    active_layer: ResMut<'a, ActiveLayer>,
    command_buffer: ResMut<'a, CommandBuffer>,
}

impl<'a> DrawContext<'a> {
    /// sets a tile to a specific character
    pub fn set(&mut self, x: u32, y: u32, background: Color, foreground: Color, glyph: char) {
        puffin::profile_function!();

        if x >= self.settings.width || y >= self.settings.height {
            // ignores anything out of bounds
            return;
        }

        // This makes sure the origin is at the top left of the tilemap
        let position = UVec2::new(x, self.settings.height as u32 - 1 - y);
        let active_layer = self.get_active_layer();
        if !active_layer.is_glyph_background_transparent {
            self.command_buffer.push(DrawCommand::DrawTile {
                layer_id: active_layer.background_id,
                position,
                tile_info: TileInfo {
                    color: background,
                    // ASCII code 219 = █ ( Block, graphic character )
                    glyph: 219 as char,
                },
            });
        }
        self.command_buffer.push(DrawCommand::DrawTile {
            layer_id: active_layer.foreground_id,
            position,
            tile_info: TileInfo {
                color: foreground,
                glyph,
            },
        });
    }

    /// Prints a string at the given position with foreground and background color
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    pub fn print_color(
        &mut self,
        x: u32,
        y: u32,
        background: Color,
        foreground: Color,
        text: &str,
    ) {
        puffin::profile_function!();

        for (i, char) in text.chars().enumerate() {
            self.set(x + i as u32, y, background, foreground, char);
        }
    }

    /// prints a string centered on the x axis with foreground and background color
    pub fn print_color_centered(
        &mut self,
        y: u32,
        background: Color,
        foreground: Color,
        text: &str,
    ) {
        puffin::profile_function!();

        self.print_color(
            (self.settings.width / 2) - (text.to_string().len() as u32 / 2),
            y,
            background,
            foreground,
            text,
        );
    }

    /// Prints a string at the given position
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    pub fn print(&mut self, x: u32, y: u32, text: &str) {
        puffin::profile_function!();

        self.print_color(x, y, Color::BLACK, Color::WHITE, text);
    }

    /// prints a string centered on the x axis
    pub fn print_centered(&mut self, y: u32, text: &str) {
        puffin::profile_function!();

        self.print_color_centered(y, Color::BLACK, Color::WHITE, text);
    }

    /// Clears the active layer
    pub fn cls(&mut self) {
        puffin::profile_function!();

        self.cls_color(Color::BLACK);
    }

    /// Clears the active layer with a specific color
    pub fn cls_color(&mut self, color: Color) {
        puffin::profile_function!();

        let active_layer = self.get_active_layer();
        self.command_buffer.push(DrawCommand::ClearLayer {
            layer_id: active_layer.background_id,
            color,
        });
        self.command_buffer.push(DrawCommand::ClearLayer {
            layer_id: active_layer.foreground_id,
            color,
        });
    }

    pub fn cls_all_layers(&mut self) {
        puffin::profile_function!();

        self.cls_color_all_layers(Color::BLACK);
    }

    pub fn cls_color_all_layers(&mut self, color: Color) {
        puffin::profile_function!();

        self.command_buffer
            .push(DrawCommand::ClearAllLayers { color });
    }

    pub fn set_active_layer(&mut self, layer: u8) {
        puffin::profile_function!();

        self.active_layer.0 = u32::from(layer);
    }

    fn get_active_layer(&self) -> Layer {
        puffin::profile_function!();

        self.settings.layers[self.active_layer.0 as usize]
    }
}
