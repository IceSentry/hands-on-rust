use bevy::prelude::*;
use bevy::{ecs::system::SystemParam, tasks::ComputeTaskPool};
use bevy_ecs_tilemap::{
    LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TilemapPlugin,
};

// TODO
// * extract plugin
// * use layers for background and foreground

pub const CHUNK_WIDTH: u32 = 10;
pub const CHUNK_HEIGHT: u32 = 10;
pub const MAP_WIDTH: u32 = 8;
pub const MAP_HEIGHT: u32 = 5;
pub const WIDTH: usize = MAP_WIDTH as usize * CHUNK_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize * CHUNK_HEIGHT as usize;

pub const TILE_WIDTH: usize = 16;
pub const TILE_HEIGHT: usize = 16;

pub const PIXEL_WIDTH: usize = WIDTH * TILE_WIDTH;
pub const PIXEL_HEIGHT: usize = HEIGHT * TILE_HEIGHT;

pub const TEXTURE_WIDTH: usize = TILE_WIDTH * 16;
pub const TEXTURE_HEIGHT: usize = TILE_HEIGHT * 16;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("hands on flappy"),
            resizable: false,
            width: PIXEL_WIDTH as f32,
            height: PIXEL_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(tilemap_setup.system())
        .add_system(update.system())
        .run();
}

fn tilemap_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("16x16-sb-ascii.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let layer_settings = LayerSettings::new(
        UVec2::new(MAP_WIDTH, MAP_HEIGHT),
        UVec2::new(CHUNK_WIDTH, CHUNK_HEIGHT),
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        Vec2::new(TEXTURE_WIDTH as f32, TEXTURE_HEIGHT as f32),
    );

    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16, None);
    layer_builder.set_all(TileBundle::new(
        Tile {
            texture_index: '.' as u16,
            ..Default::default()
        },
        UVec2::default(),
    ));
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(
            -((PIXEL_WIDTH / 2) as f32),
            -((PIXEL_HEIGHT / 2) as f32),
            0.0,
        ))
        .insert(GlobalTransform::default());
}

fn update(mut ctx: DrawContext) {
    ctx.cls();
    ctx.print(0, 0, "Hello bevy_ecs_tilemap hands-on");
}

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    pub map_query: MapQuery<'a>,
    pub tile_query: Query<'a, &'static mut bevy_ecs_tilemap::Tile>,
    pub pool: Res<'a, ComputeTaskPool>,
}

impl<'a> DrawContext<'a> {
    /// Prints a string at the given position
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    fn print(&mut self, x: usize, y: usize, output: &str) {
        for (i, char) in output.chars().enumerate() {
            if x + i >= WIDTH {
                return;
            }

            let tile_entity = self
                .map_query
                .get_tile_entity(
                    // This makes sure the origin is at the top left of the tilemap
                    UVec2::new((x + i) as u32, HEIGHT as u32 - 1 - y as u32),
                    0u16,
                    0u16,
                )
                .expect("no tile found");
            if let Ok(mut tile) = self.tile_query.get_mut(tile_entity) {
                tile.texture_index = char as u16;
            }
        }
    }

    /// Clears the screen
    fn cls(&mut self) {
        self.tile_query
            .par_for_each_mut(&self.pool, 10, |mut tile| {
                tile.texture_index = 0;
            });
    }
}
