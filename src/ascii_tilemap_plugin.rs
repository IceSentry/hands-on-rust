use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_ecs_tilemap::{Map, MapQuery};

use crate::ascii_tilemap_plugin::render::TileRenderData;

use self::render::RenderLayers;

pub mod color;
pub mod geometry;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub struct TilemapDrawing;

pub struct AsciiTilemapPlugin;

impl Plugin for AsciiTilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin)
            .add_system_set(
                SystemSet::new()
                    .label(TilemapDrawing)
                    .with_system(process_command_buffer.system().before("render"))
                    .with_system(render::render.system().label("render")),
            )
            .add_startup_system(setup.system())
            .insert_resource(ActiveLayer(0));
    }
}

#[derive(Debug, Clone)]
pub struct ActiveLayer(u16);

pub type LayerEntities = Vec<Entity>;

#[derive(Debug, Clone)]
pub struct Layer {
    background_id: u16,
    foreground_id: u16,
    is_transparent: bool,
    is_background_transparent: bool,
    size: UVec2,
    command_buffer: Vec<DrawCommand>,
}

#[derive(Debug, Clone)]
enum DrawCommand {
    DrawTile {
        x: u32,
        y: u32,
        background: Color,
        foreground: Color,
        glyph: char,
    },
    ClearLayer {
        color: Color,
    },
}

#[derive(Debug, Clone)]
pub struct TilemapBuilder {
    pub layers: Vec<LayerBuilderData>,
}

#[derive(Debug, Clone)]
pub struct LayerBuilderData {
    pub texture_path: Option<String>,
    pub size: Option<UVec2>,
    pub tile_size: Option<Vec2>,
    /// WARN dimension in tiles
    pub tilesheet_size: Option<Vec2>,
    pub id: u16,
    pub is_transparent: bool,
    pub is_background_transparent: bool,
}

impl LayerBuilderData {
    fn build(&self) -> Layer {
        Layer {
            background_id: self.id * 2,
            foreground_id: self.id * 2 + 1,
            command_buffer: vec![],
            size: self.size.expect("layer.size not set"),
            is_background_transparent: self.is_background_transparent,
            is_transparent: self.is_transparent,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
    tilemap_builder: Res<TilemapBuilder>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0_u16, map_entity);

    // always just use 1 chunk per layer since this is always going to be on screen anyway
    let map_size = UVec2::new(1, 1);
    let mut render_layers = Vec::with_capacity(tilemap_builder.layers.len() * 2);

    let mut build_layer = |layer_id, layer_settings, material_handle: Handle<ColorMaterial>| {
        let (mut layer_builder, layer_entity) = bevy_ecs_tilemap::LayerBuilder::new(
            &mut commands,
            layer_settings,
            0_u16,
            layer_id,
            None,
        );
        layer_builder.set_all(bevy_ecs_tilemap::TileBundle::default());
        map_query.build_layer(&mut commands, layer_builder, material_handle);
        map.add_layer(&mut commands, layer_id, layer_entity);

        render_layers.push(vec![
            TileRenderData::default();
            (layer_settings.chunk_size.x * layer_settings.chunk_size.y)
                as usize
        ]);
    };

    for layer_builder_data in &tilemap_builder.layers {
        let chunk_size = layer_builder_data.size.expect("size not set");
        let tile_size = layer_builder_data.tile_size.expect("tile_size not set");
        let tilesheet_size = layer_builder_data
            .tilesheet_size
            .expect("tilesheet_size not set");
        let layer_settings = bevy_ecs_tilemap::LayerSettings::new(
            map_size,
            chunk_size,
            tile_size,
            tilesheet_size * tile_size,
        );
        let path = layer_builder_data
            .texture_path
            .clone()
            .expect("texture_path not set");

        let texture_handle = asset_server.load(path.as_str());
        let material_handle = materials.add(ColorMaterial::texture(texture_handle));
        build_layer(
            layer_builder_data.id * 2,
            layer_settings,
            material_handle.clone(),
        );
        build_layer(
            layer_builder_data.id * 2 + 1,
            layer_settings,
            material_handle,
        );
    }
    for (id, l) in render_layers.iter().enumerate() {
        info!("layer_id: {} len: {}", id, l.len());
    }

    commands.insert_resource(render_layers as RenderLayers);

    let mut layer_entities = vec![];
    for layer_builder_data in &tilemap_builder.layers {
        // because of borrow checker can't do this in the other loop
        let layer_data = layer_builder_data.build();
        info!("layer_data {:?}", layer_data);
        let entity = commands.spawn().insert(layer_data).id();
        layer_entities.push(entity);
    }
    commands.insert_resource(layer_entities as LayerEntities);

    let size = tilemap_builder.layers[0]
        .size
        .expect("size not set on first layer")
        .as_f32();
    let tile_size = tilemap_builder.layers[0]
        .tile_size
        .expect("tile_size not set on first layer");
    let window_size = (size * tile_size) / 2.;
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-window_size.x, -window_size.y, 0.0))
        .insert(GlobalTransform::default());
}

fn process_command_buffer(layers: Query<&mut Layer>, mut render_layers: ResMut<RenderLayers>) {
    puffin::profile_function!();
    layers.for_each_mut(|mut layer| {
        for command in &layer.command_buffer {
            match *command {
                DrawCommand::DrawTile {
                    x,
                    y,
                    background,
                    foreground,
                    glyph,
                } => {
                    let y = layer.size.y - y - 1;
                    let index = (y * layer.size.x + x) as usize;

                    if !layer.is_background_transparent {
                        let background_tile = TileRenderData::new(background, 219 as char); // ASCII code 219 = █ ( Block, graphic character )
                        render_layers[layer.background_id as usize][index] = background_tile;
                    }

                    let foreground_tile = TileRenderData::new(foreground, glyph);
                    render_layers[layer.foreground_id as usize][index] = foreground_tile;
                }
                DrawCommand::ClearLayer { color } => {
                    for mut tile in &mut render_layers[layer.background_id as usize] {
                        tile.glyph = if layer.is_transparent {
                            0 as char // foreground and transparent backgrounds should be invisible after clear
                        } else {
                            219 as char // ASCII code 219 = █ ( Block, graphic character )
                        };
                        tile.color = color;
                    }
                    for mut tile in &mut render_layers[layer.foreground_id as usize] {
                        tile.glyph = 0 as char;
                        tile.color = color;
                    }
                }
            }
        }
        layer.command_buffer.clear();
    });
}

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    layers: Query<'a, &'static mut Layer>,
    active_layer: ResMut<'a, ActiveLayer>,
    layer_entities: Res<'a, LayerEntities>,
}

impl<'a> DrawContext<'a> {
    pub fn set(&mut self, x: u32, y: u32, background: Color, foreground: Color, glyph: char) {
        puffin::profile_function!();
        let entity = self.layer_entities[self.active_layer.0 as usize];
        if let Ok(mut layer) = self.layers.get_mut(entity) {
            if x >= layer.size.x || y >= layer.size.y {
                // ignores anything out of bounds
                return;
            }

            layer.command_buffer.push(DrawCommand::DrawTile {
                x,
                y,
                background,
                foreground,
                glyph,
            });
        }
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
        let size = self.get_active_layer_size();
        self.print_color(
            (size.x / 2) - (text.to_string().len() as u32 / 2),
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
        self.cls_color(Color::BLACK);
    }

    /// Clears the active layer with a specific color
    pub fn cls_color(&mut self, color: Color) {
        puffin::profile_function!();
        let entity = self.layer_entities[self.active_layer.0 as usize];
        if let Ok(mut layer) = self.layers.get_mut(entity) {
            layer.command_buffer.push(DrawCommand::ClearLayer { color });
        }
    }

    pub fn cls_all_layers(&mut self) {
        self.cls_color_all_layers(Color::BLACK);
    }

    pub fn cls_color_all_layers(&mut self, color: Color) {
        puffin::profile_function!();
        self.layers.for_each_mut(|mut layer| {
            layer.command_buffer.push(DrawCommand::ClearLayer { color });
        });
    }

    pub fn set_active_layer(&mut self, layer: u8) {
        puffin::profile_function!();
        self.active_layer.0 = u16::from(layer);
    }

    pub fn get_active_layer_size(&mut self) -> UVec2 {
        puffin::profile_function!();
        let entity = self.layer_entities[self.active_layer.0 as usize];
        let layer = self.layers.get_mut(entity).expect("layer not found");
        layer.size
    }
}

mod render {
    use super::Layer;
    use bevy::{prelude::*, utils::HashSet};
    use bevy_ecs_tilemap::{Chunk, Tile, TileParent};

    pub type RenderLayers = Vec<Vec<TileRenderData>>;

    #[derive(Debug, Clone, Copy)]
    pub struct TileRenderData {
        pub color: Color,
        pub glyph: char,
    }
    impl TileRenderData {
        pub fn new(color: Color, glyph: char) -> Self {
            Self { color, glyph }
        }
    }
    impl Default for TileRenderData {
        fn default() -> Self {
            Self::new(Color::BLACK, 0 as char)
        }
    }

    pub fn render(
        mut chunk_query: Query<&mut Chunk>,
        tile_query: Query<(&mut Tile, &TileParent, &UVec2)>,
        render_layers: Res<RenderLayers>,
        layers_query: Query<&mut Layer>,
    ) {
        puffin::profile_function!();
        let mut chunks = HashSet::default();
        layers_query.for_each_mut(|layer| {
            puffin::profile_scope!("layers_query");
            let width = layer.size.x;
            let background_layer = &render_layers[layer.background_id as usize];
            let foreground_layer = &render_layers[layer.foreground_id as usize];

            tile_query.for_each_mut(|(mut tile, tile_parent, pos)| {
                puffin::profile_scope!("tile_query");
                let layer_id = tile_parent.layer_id;
                if layer_id == layer.background_id || layer_id == layer.foreground_id {
                    let index = (pos.y * width + pos.x) as usize;
                    let tile_data = if layer_id == layer.background_id {
                        background_layer[index]
                    } else {
                        *foreground_layer.get(index).unwrap_or_else(|| {
                            panic!(
                                "failed to get tile at layer_id {} {} {} {} {}",
                                layer_id, layer.foreground_id, index, width, pos
                            )
                        })
                    };

                    if tile.texture_index != tile_data.glyph as u16 || tile.color != tile_data.color
                    {
                        tile.texture_index = tile_data.glyph as u16;
                        tile.color = tile_data.color;
                        chunks.insert(tile_parent.chunk);
                    }
                }
            });
        });

        for chunk_entity in chunks.drain() {
            if let Ok(mut chunk) = chunk_query.get_mut(chunk_entity) {
                chunk.needs_remesh = true;
            }
        }
    }
}
