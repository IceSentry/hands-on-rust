#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::default_trait_access,
    clippy::module_name_repetitions
)]

use self::{
    draw_context::ActiveLayer,
    render::{RenderLayers, TileRenderData},
};
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, render_resource::TextureUsages},
};
use bevy_ecs_tilemap::{
    ChunkPos, ChunkSize, Map, MapQuery, MapSize, TextureSize, TileParent, TilePos, TileSize,
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
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin)
            .add_system_set(
                SystemSet::new()
                    .label(TilemapDrawing)
                    .with_system(process_command_buffer.before("render"))
                    .with_system(render::render.label("render")),
            )
            .add_startup_system(setup.label("setup"))
            .add_startup_stage("tile_setup", SystemStage::parallel())
            .add_startup_system_to_stage("tile_setup", setup_tiles)
            .add_system(set_texture_filters_to_nearest)
            .insert_resource(ActiveLayer(0));
    }
}

pub type LayerEntities = Vec<Entity>;

#[derive(Debug, Clone, Component)]
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
    mut map_query: MapQuery,
    tilemap_builder: Res<TilemapBuilder>,
) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera_bundle.orthographic_projection.scale = 400.;
    commands.spawn_bundle(camera_bundle);

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0_u16, map_entity);

    let mut render_layers = Vec::with_capacity(tilemap_builder.layers.len() * 2);

    let mut build_layer = |layer_id, layer_settings, material_handle: Handle<Image>| {
        let (mut layer_builder, layer_entity) =
            bevy_ecs_tilemap::LayerBuilder::new(&mut commands, layer_settings, 0_u16, layer_id);
        layer_builder.set_all(bevy_ecs_tilemap::TileBundle::default());
        map_query.build_layer(&mut commands, layer_builder, material_handle);
        map.add_layer(&mut commands, layer_id, layer_entity);

        let render_layer = vec![
            TileRenderData::default();
            (layer_settings.chunk_size.0 * layer_settings.chunk_size.1) as usize
        ];
        info!("layer_id: {} len: {}", layer_id, render_layer.len());
        render_layers.push(render_layer);
    };

    // always just use 1 chunk per layer since this is always going to be on screen anyway
    let map_size = MapSize(1, 1);

    for layer_builder_data in &tilemap_builder.layers {
        let layer_data = layer_builder_data.build_layer();

        let tile_size = layer_builder_data.tile_size.expect("tile_size not set");
        let tilesheet_size = layer_builder_data
            .tilesheet_size
            .expect("tilesheet_size not set");
        let texture_size = tilesheet_size * tile_size;

        let layer_settings = bevy_ecs_tilemap::LayerSettings::new(
            map_size,
            ChunkSize(layer_data.size.x, layer_data.size.y),
            TileSize(tile_size.x, tile_size.y),
            TextureSize(texture_size.x, texture_size.y),
        );
        let path = layer_builder_data
            .texture_path
            .clone()
            .expect("texture_path not set");

        let texture_handle = asset_server.load(path.as_str());
        if layer_data.is_background_transparent {
            let mut layer_settings = layer_settings;
            // this should help iteration speed since we don't need to iterate as many tiles
            layer_settings.chunk_size = ChunkSize(0, 0);
            build_layer(
                layer_data.background_id,
                layer_settings,
                texture_handle.clone(),
            );
        } else {
            build_layer(
                layer_data.background_id,
                layer_settings,
                texture_handle.clone(),
            );
        }
        build_layer(layer_data.foreground_id, layer_settings, texture_handle);
    }
    commands.insert_resource(render_layers as RenderLayers);

    let mut layer_entities = vec![];
    for layer_builder_data in &tilemap_builder.layers {
        let layer_data = layer_builder_data.build_layer();
        // info!("layer_data {:?}", layer_data);
        // because of borrow checker can't do this in the other loop
        // can't borrow commands
        let entity = commands.spawn().insert(layer_data).id();
        layer_entities.push(entity);
    }
    commands.insert_resource(layer_entities as LayerEntities);

    let size = tilemap_builder.layers[0]
        .size
        .expect("size not set on first layer")
        .as_vec2();
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

#[derive(Component)]
pub struct TileData {
    pub index: usize,
    pub layer_id: usize,
    pub chunk: Entity,
}

fn setup_tiles(
    mut commands: Commands,
    tile_query: Query<(Entity, &TileParent, &TilePos)>,
    layers: Query<&Layer>,
) {
    let mut i = 0;
    tile_query.for_each(|(entity, tile_parent, pos)| {
        i += 1;
        let layer = layers
            .iter()
            .find(|l| {
                l.background_id == tile_parent.layer_id || l.foreground_id == tile_parent.layer_id
            })
            .expect("layer not found");
        let index = (pos.1 * layer.size.x + pos.0) as usize;
        commands.entity(entity).insert(TileData {
            index,
            layer_id: tile_parent.layer_id as usize,
            chunk: tile_parent.chunk,
        });
    });
    info!("TileData added to tiles {}", i);
}

fn process_command_buffer(mut layers: Query<&mut Layer>, mut render_layers: ResMut<RenderLayers>) {
    // puffin::profile_function!();
    layers.for_each_mut(|mut layer| {
        // info!("buffer len: {}", layer.command_buffer.len());
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
                        let background_tile = TileRenderData::new(background, 219 as char); // ASCII code 219 = ??? ( Block, graphic character )
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
                            219 // ASCII code 219 = ??? ( Block, graphic character )
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
    let map = map_query.single();
    map.get_layer_entity(layer_id)
        .and_then(|layer_entity| layer_query.get(*layer_entity).ok())
        .and_then(|layer| layer.get_chunk(ChunkPos(0, 0)))
        .and_then(|chunk_entity| chunk_query.get(chunk_entity).ok())
        .and_then(|chunk| {
            chunk.get_tile_entity(chunk.to_chunk_pos(TilePos(tile_pos.x, tile_pos.y)))
        })
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::COPY_DST;
            }
        }
    }
}
