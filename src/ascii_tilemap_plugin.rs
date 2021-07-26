#![allow(unused)]

use bevy::{
    asset::AssetPath,
    ecs::system::SystemParam,
    prelude::*,
    utils::{HashMap, Instant},
};
use bevy_ecs_tilemap::{
    Chunk, LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TileParent, TilemapPlugin,
};

pub mod color;
pub mod geometry;

const BACKGROUND_LAYER_ID: u32 = 0;

#[derive(Clone)]
pub struct AsciiTilemapSettings {
    /// The asset path to the tilesheet texture
    tilesheet_asset_path: String,
    /// The amount of tiles displayed on the screen horizontally
    width: u32,
    /// The amount of tiles displayed on the screen horizontally
    height: u32,
    /// The amount of pixels horizontally for a single tile
    tile_width: u32,
    /// The amount of pixels vertically for a single tile
    tile_height: u32,
    /// The amount of tiles horizontally in the spritesheet
    tilesheet_width: u32,
    /// The amount of tiles vertically in the spritesheet
    tilesheet_height: u32,
    /// The amount of chunks horizontally
    horizontal_chunks: u32,
    /// The amount of chunks vertically
    vertical_chunks: u32,
    layers: Vec<Layer>,
}

impl Default for AsciiTilemapSettings {
    fn default() -> Self {
        Self {
            tilesheet_asset_path: "tilesheet.png".into(),
            width: 80,
            height: 50,
            tile_width: 16,
            tile_height: 16,
            tilesheet_height: 16,
            tilesheet_width: 16,
            horizontal_chunks: 1,
            vertical_chunks: 1,
            layers: vec![],
        }
    }
}

impl AsciiTilemapSettings {
    pub fn builder() -> AsciiTilemapSettingsBuilder {
        AsciiTilemapSettingsBuilder::default()
    }

    pub fn window_width(&self) -> f32 {
        (self.width * self.tile_width) as f32
    }

    pub fn window_height(&self) -> f32 {
        (self.height * self.tile_height) as f32
    }
}

pub struct AsciiTilemapSettingsBuilder {
    settings: AsciiTilemapSettings,
}

impl Default for AsciiTilemapSettingsBuilder {
    fn default() -> Self {
        Self {
            settings: AsciiTilemapSettings::default(),
        }
    }
}

impl AsciiTilemapSettingsBuilder {
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    pub fn with_tile_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.tile_width = width;
        self.settings.tile_height = height;
        self
    }

    pub fn with_tilesheet_path<S: ToString>(&mut self, path: S) -> &mut Self {
        self.settings.tilesheet_asset_path = path.to_string();
        self
    }

    /// The dimension of the tilesheet
    /// WARN in tiles not in pixels
    pub fn with_tilesheet_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.tilesheet_width = width;
        self.settings.tile_height = height;
        self
    }

    pub fn with_layer(
        &mut self,
        layer_id: u8,
        is_transparent: bool,
        is_glyph_background_transparent: bool,
    ) -> &mut Self {
        self.settings.layers.push(Layer::new(
            layer_id,
            is_transparent,
            is_glyph_background_transparent,
        ));
        self
    }

    pub fn build(&self) -> AsciiTilemapSettings {
        self.settings.clone()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stage {
    AfterUpdate,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub struct Drawing;

struct TileToDraw {
    texture_index: char,
    color: Color,
}
pub struct TilesToDraw(HashMap<UVec3, TileToDraw>);

pub struct ActiveLayer(u32);

// TODO support per layer tilesheet
#[derive(Clone, Copy)]
struct Layer {
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

pub struct AsciiTilemapPlugin;

impl Plugin for AsciiTilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TilemapPlugin)
            .add_system(draw.system().label(Drawing))
            .add_startup_system(setup.system())
            .insert_resource(TilesToDraw(HashMap::default()))
            .insert_resource(ActiveLayer(0));
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

    info!(
        "initializing ascii_tilemap_plugin...done {:?}",
        start.elapsed()
    );
}

fn draw(
    mut chunk_query: Query<&mut Chunk>,
    mut tiles_to_draw: ResMut<TilesToDraw>,
    mut tile_query: Query<(&mut Tile, &TileParent, &UVec2)>,
) {
    if tiles_to_draw.0.is_empty() {
        return;
    }

    tile_query.for_each_mut(|(mut tile, tile_parent, tile_pos)| {
        if let Some(tile_to_draw) = tiles_to_draw
            .0
            .get(&tile_pos.extend(tile_parent.layer_id.into()))
        {
            tile.texture_index = tile_to_draw.texture_index as u16;
            tile.color = tile_to_draw.color;
        }
    });
    tiles_to_draw.0.clear();

    // always update all the chunks because we always clear the screen
    chunk_query.for_each_mut(|mut chunk| {
        chunk.needs_remesh = true;
    });
}

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    pub map_query: MapQuery<'a>,
    pub tile_query: Query<'a, (&'static mut Tile, &'static TileParent)>,
    settings: Res<'a, AsciiTilemapSettings>,
    tiles_to_draw: ResMut<'a, TilesToDraw>,
    active_layer: ResMut<'a, ActiveLayer>,
}

impl<'a> DrawContext<'a> {
    /// sets a tile to a specific character
    pub fn set(&mut self, x: u32, y: u32, background: Color, foreground: Color, glyph: char) {
        if x >= self.settings.width || y >= self.settings.height {
            return;
        }

        // This makes sure the origin is at the top left of the tilemap
        let position = UVec2::new(x, self.settings.height as u32 - 1 - y);
        let active_layer = self.get_active_layer();
        if !active_layer.is_glyph_background_transparent {
            self.tiles_to_draw.0.insert(
                position.extend(u32::from(active_layer.background_id)),
                TileToDraw {
                    color: background,
                    texture_index: 219 as char, // ASCII code 219 = █ ( Block, graphic character )
                },
            );
        }
        self.tiles_to_draw.0.insert(
            position.extend(u32::from(active_layer.foreground_id)),
            TileToDraw {
                color: foreground,
                texture_index: glyph,
            },
        );
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
        self.print_color(x, y, Color::BLACK, Color::WHITE, text);
    }

    /// prints a string centered on the x axis
    pub fn print_centered(&mut self, y: u32, text: &str) {
        self.print_color_centered(y, Color::BLACK, Color::WHITE, text);
    }

    /// Clears the `active_layer`
    pub fn cls(&mut self) {
        self.cls_color(Color::BLACK);
    }

    /// Clears the `active_layer` with a specific color
    pub fn cls_color(&mut self, color: Color) {
        self.tile_query.for_each_mut(|(mut tile, tile_parent)| {
            let active_layer = self.get_active_layer();
            if active_layer.background_id == tile_parent.layer_id
                || active_layer.foreground_id == tile_parent.layer_id
            {
                Self::clear_tile(&mut tile, tile_parent.layer_id, active_layer, color);
            }
        });
    }

    pub fn cls_all_layers(&mut self) {
        self.cls_color_all_layers(Color::BLACK);
    }

    pub fn cls_color_all_layers(&mut self, color: Color) {
        self.tile_query.for_each_mut(|(mut tile, tile_parent)| {
            let layer_index = if tile_parent.layer_id % 2 == 0 {
                tile_parent.layer_id / 2
            } else {
                (tile_parent.layer_id - 1) / 2
            };
            let layer = self.settings.layers[layer_index as usize];
            Self::clear_tile(&mut tile, tile_parent.layer_id, layer, color);
        });
    }

    fn clear_tile(tile: &mut Tile, layer_id: u16, layer: Layer, color: Color) {
        tile.texture_index = if layer_id == layer.background_id && !layer.is_transparent {
            219 // ASCII code 219 = █ ( Block, graphic character )
        } else {
            0 // foreground and transparent backgrounds should be invisible after clear
        };
        tile.color = color;
    }

    /// sets the active layer used by the `DrawContext`
    pub fn set_active_layer(&mut self, layer: u8) {
        self.active_layer.0 = u32::from(layer);
    }

    fn get_active_layer(&self) -> Layer {
        self.settings.layers[self.active_layer.0 as usize]
    }
}
