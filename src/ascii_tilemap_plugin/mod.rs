use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tilemap::{Map, MapQuery, TileParent};

use self::{
    draw_context::ActiveLayer,
    render::{RenderLayers, TileRenderData},
};

pub use builder::{LayerDataBuilder, TilemapBuilder};
pub use draw_context::DrawContext;

mod builder;
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
    tilemap_builder: Res<TilemapBuilder>,
) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera_bundle.orthographic_projection.scale = 400.;
    commands.spawn_bundle(camera_bundle);

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

        let render_layer = vec![
            TileRenderData::default();
            (layer_settings.chunk_size.x * layer_settings.chunk_size.y) as usize
        ];
        info!("layer_id: {} len: {}", layer_id, render_layer.len());
        render_layers.push(render_layer);
    };

    for layer_builder_data in &tilemap_builder.layers {
        let layer_data = layer_builder_data.build_layer();

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
        if layer_data.is_background_transparent {
            let mut layer_settings = layer_settings;
            // this should help iteration speed since we don't need to iterate as many tiles
            layer_settings.chunk_size = UVec2::ZERO;
            build_layer(
                layer_data.background_id,
                layer_settings,
                material_handle.clone(),
            );
        } else {
            build_layer(
                layer_data.background_id,
                layer_settings,
                material_handle.clone(),
            );
        }
        build_layer(layer_data.foreground_id, layer_settings, material_handle);
    }
    commands.insert_resource(render_layers as RenderLayers);

    let mut layer_entities = vec![];
    for layer_builder_data in &tilemap_builder.layers {
        let layer_data = layer_builder_data.build_layer();
        info!("layer_data {:?}", layer_data);
        // because of borrow checker can't do this in the other loop
        // can't borrow commands
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
    info!("TileData added to tiles");
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
                            0 // foreground and transparent backgrounds should be invisible after clear
                        } else {
                            219 // ASCII code 219 = █ ( Block, graphic character )
                        };
                        tile.color = color;
                    }
                    for mut tile in &mut render_layers[layer.foreground_id as usize] {
                        tile.glyph = 0_u16;
                        tile.color = color;
                    }
                }
            }
        }
        layer.command_buffer.clear();
    });
}

// This assumes a single map with a single chunk per layer
fn _get_tile_entity(
    map_query: &Query<&bevy_ecs_tilemap::Map>,
    layer_query: &Query<&bevy_ecs_tilemap::Layer>,
    chunk_query: &Query<&bevy_ecs_tilemap::Chunk>,
    tile_pos: UVec2,
    layer_id: u16,
) -> Option<Entity> {
    let map = map_query.single().expect("map not found");
    map.get_layer_entity(layer_id)
        .and_then(|layer_entity| layer_query.get(*layer_entity).ok())
        .and_then(|layer| layer.get_chunk(UVec2::ZERO))
        .and_then(|chunk_entity| chunk_query.get(chunk_entity).ok())
        .and_then(|chunk| chunk.get_tile_entity(chunk.to_chunk_pos(tile_pos)))
}
