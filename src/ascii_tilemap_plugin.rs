#![allow(unused)]

use bevy::{asset::AssetPath, ecs::system::SystemParam, prelude::*};
use bevy_ecs_tilemap::{
    Chunk, LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TilemapPlugin,
};

pub mod geometry;

const CHUNK_WIDTH: u32 = 10;
const CHUNK_HEIGHT: u32 = 10;

const MAP_WIDTH: u32 = 8;
const MAP_HEIGHT: u32 = 5;

const TILE_WIDTH: usize = 16;
const TILE_HEIGHT: usize = 16;

const TEXTURE_WIDTH: usize = TILE_WIDTH * 16;
const TEXTURE_HEIGHT: usize = TILE_HEIGHT * 16;

pub const WIDTH: usize = MAP_WIDTH as usize * CHUNK_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize * CHUNK_HEIGHT as usize;

pub const WINDOW_WIDTH: usize = WIDTH * TILE_WIDTH;
pub const WINDOW_HEIGHT: usize = HEIGHT * TILE_HEIGHT;

const BACKGROUND_LAYER_ID: u16 = 0;
const FOREGROUND_LAYER_ID: u16 = 1;

pub struct AsciiTilemapPlugin;

pub struct AsciiTilemapSettings {
    pub tilemap_asset_path: &'static str,
}

impl Plugin for AsciiTilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TilemapPlugin)
            .add_startup_system(setup.system())
            .add_system(update_chunks.system());
    }
}

/// Forces the chunks to rerender on each frame
fn update_chunks(mut chunk_query: Query<&mut Chunk>) {
    for mut chunk in chunk_query.iter_mut() {
        chunk.needs_remesh = true;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<AsciiTilemapSettings>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load(settings.tilemap_asset_path);
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let layer_settings = LayerSettings::new(
        UVec2::new(MAP_WIDTH, MAP_HEIGHT),
        UVec2::new(CHUNK_WIDTH, CHUNK_HEIGHT),
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        Vec2::new(TEXTURE_WIDTH as f32, TEXTURE_HEIGHT as f32),
    );

    let mut build_layer = |layer_id| {
        let (mut layer_builder, layer_entity) =
            LayerBuilder::new(&mut commands, layer_settings, 0u16, layer_id, None);
        layer_builder.set_all(TileBundle::default());
        map_query.build_layer(&mut commands, layer_builder, material_handle.clone());
        map.add_layer(&mut commands, layer_id, layer_entity);
    };

    build_layer(BACKGROUND_LAYER_ID);
    build_layer(FOREGROUND_LAYER_ID);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(
            -((WINDOW_WIDTH / 2) as f32),
            -((WINDOW_HEIGHT / 2) as f32),
            0.0,
        ))
        .insert(GlobalTransform::default());
}

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    pub map_query: MapQuery<'a>,
    pub tile_query: Query<'a, &'static mut Tile>,
}

impl<'a> DrawContext<'a> {
    /// Prints a string at the given position
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    pub fn print(&mut self, x: usize, y: usize, text: &str) {
        self.print_color(x, y, Color::BLACK, Color::WHITE, text);
    }

    /// Prints a string at the given position with foreground and background color
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    pub fn print_color(
        &mut self,
        x: usize,
        y: usize,
        background: Color,
        foreground: Color,
        text: &str,
    ) {
        for (i, char) in text.chars().enumerate() {
            self.set(x + i, y, background, foreground, char);
        }
    }

    /// prints a string centered on the x axis
    pub fn print_centered(&mut self, y: usize, text: &str) {
        self.print((WIDTH / 2) - (text.to_string().len() / 2), y, text);
    }

    /// prints a string centered on the x axis with foreground and background color
    pub fn print_color_centered(&mut self, y: usize, text: &str) {
        self.print((WIDTH / 2) - (text.to_string().len() / 2), y, text);
    }

    /// sets a tile to a specific character
    pub fn set(&mut self, x: usize, y: usize, background: Color, foreground: Color, char: char) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        // This makes sure the origin is at the top left of the tilemap
        let position = UVec2::new(x as u32, HEIGHT as u32 - 1 - y as u32);

        let background_tile_entity = self
            .map_query
            .get_tile_entity(position, 0u16, BACKGROUND_LAYER_ID)
            .unwrap_or_else(|_| panic!("tile not found at {} ", position));
        if let Ok(mut tile) = self.tile_query.get_mut(background_tile_entity) {
            tile.color = background;
        }

        let foreground_tile_entity = self
            .map_query
            .get_tile_entity(position, 0u16, FOREGROUND_LAYER_ID)
            .unwrap_or_else(|_| panic!("tile not found at {} ", position));
        if let Ok(mut tile) = self.tile_query.get_mut(foreground_tile_entity) {
            tile.texture_index = char as u16;
            tile.color = foreground;
        }
    }

    /// Clears the screen
    pub fn cls(&mut self) {
        self.cls_color(Color::BLACK);
    }

    /// Clears the screen with a specific color
    pub fn cls_color(&mut self, color: Color) {
        for mut tile in self.tile_query.iter_mut() {
            tile.texture_index = 219; // ASCII code 219 = █ ( Block, graphic character )
            tile.color = color;
        }
    }
}
