#![allow(unused)]

use bevy::{asset::AssetPath, ecs::system::SystemParam, prelude::*};
use bevy_ecs_tilemap::{
    Chunk, LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TilemapPlugin,
};

pub mod geometry;

const BACKGROUND_LAYER_ID: u16 = 0;
const FOREGROUND_LAYER_ID: u16 = 1;

pub struct AsciiTilemapPlugin;

pub struct AsciiTilemapSettings {
    /// The asset path to the tilesheet texture
    /// default = "tilesheet.png"
    pub tilesheet_asset_path: &'static str,
    /// The amount of tiles displayed on the screen horizontally
    /// default = 80
    pub width: u32,
    /// The amount of tiles displayed on the screen horizontally
    /// default = 50
    pub height: u32,
    /// The amount of pixels horizontally for a single tile
    /// default = 16
    pub tile_width: u32,
    /// The amount of pixels vertically for a single tile
    /// default = 16
    pub tile_height: u32,
    /// The amount of tiles horizontally in the spritesheet
    /// default = 16
    pub tilesheet_width: u32,
    /// The amount of tiles vertically in the spritesheet
    /// default = 16
    pub tilesheet_height: u32,
    /// The amount of chunks horizontally
    /// default = 2
    pub horizontal_chunks: u32,
    /// The amount of chunks vertically
    /// default = 2
    pub vertical_chunks: u32,
}

impl Default for AsciiTilemapSettings {
    fn default() -> Self {
        Self {
            tilesheet_asset_path: "tilesheet.png",
            width: 80,
            height: 50,
            tile_width: 16,
            tile_height: 16,
            tilesheet_height: 16,
            tilesheet_width: 16,
            horizontal_chunks: 2,
            vertical_chunks: 2,
        }
    }
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

    let texture_handle = asset_server.load(settings.tilesheet_asset_path);
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

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
            LayerBuilder::new(&mut commands, layer_settings, 0u16, layer_id, None);
        layer_builder.set_all(TileBundle::default());
        map_query.build_layer(&mut commands, layer_builder, material_handle.clone());
        map.add_layer(&mut commands, layer_id, layer_entity);
    };

    build_layer(BACKGROUND_LAYER_ID);
    build_layer(FOREGROUND_LAYER_ID);

    let window_width = settings.width * settings.tile_width;
    let window_height = settings.height * settings.tile_height;

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(
            -((window_width / 2) as f32),
            -((window_height / 2) as f32),
            0.0,
        ))
        .insert(GlobalTransform::default());
}

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    pub map_query: MapQuery<'a>,
    pub tile_query: Query<'a, &'static mut Tile>,
    settings: Res<'a, AsciiTilemapSettings>,
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
        self.print(
            (self.settings.width as usize / 2) - (text.to_string().len() / 2),
            y,
            text,
        );
    }

    /// prints a string centered on the x axis with foreground and background color
    pub fn print_color_centered(&mut self, y: usize, text: &str) {
        self.print(
            (self.settings.width as usize / 2) - (text.to_string().len() / 2),
            y,
            text,
        );
    }

    /// sets a tile to a specific character
    pub fn set(&mut self, x: usize, y: usize, background: Color, foreground: Color, char: char) {
        if x >= self.settings.width as usize || y >= self.settings.height as usize {
            return;
        }

        // This makes sure the origin is at the top left of the tilemap
        let position = UVec2::new(x as u32, self.settings.height as u32 - 1 - y as u32);

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
