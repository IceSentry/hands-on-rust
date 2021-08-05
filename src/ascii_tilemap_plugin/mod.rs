use bevy::prelude::*;
use bevy_ecs_tilemap::{Map, MapQuery, TileParent};

use self::{
    draw_context::ActiveLayer,
    render::{RenderLayers, TileRenderData},
};

pub use draw_context::DrawContext;

pub mod color;
pub mod draw_context;
pub mod geometry;
mod render;

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
            .add_startup_stage("tile_setup", SystemStage::parallel())
            .add_startup_system_to_stage("tile_setup", setup_tiles.system())
            .insert_resource(ActiveLayer(0));
    }
}

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
        let layer_data = layer_builder_data.build();

        let chunk_size = layer_data.size;
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
            layer_data.background_id,
            layer_settings,
            material_handle.clone(),
        );
        build_layer(layer_data.foreground_id, layer_settings, material_handle);
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

pub struct TileData {
    pub index: usize,
    pub layer_id: usize,
    pub chunk: Entity,
}

fn setup_tiles(
    mut commands: Commands,
    tile_query: Query<(Entity, &TileParent, &UVec2)>,
    layers: Query<&Layer>,
) {
    tile_query.for_each(|(entity, tile_parent, pos)| {
        let layer = layers
            .iter()
            .find(|l| {
                l.background_id == tile_parent.layer_id || l.foreground_id == tile_parent.layer_id
            })
            .expect("layer not found");
        let index = (pos.y * layer.size.x + pos.x) as usize;
        commands.entity(entity).insert(TileData {
            index,
            layer_id: tile_parent.layer_id as usize,
            chunk: tile_parent.chunk,
        });
    });
    info!("LayerData added to tiles");
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
