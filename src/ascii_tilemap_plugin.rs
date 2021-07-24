#![allow(unused)]

use bevy::{asset::AssetPath, ecs::system::SystemParam, prelude::*, utils::HashMap};
use bevy_ecs_tilemap::{
    Chunk, LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TileParent, TilemapPlugin,
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
    /// default = 1
    pub horizontal_chunks: u32,
    /// The amount of chunks vertically
    /// default = 1
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
            horizontal_chunks: 1,
            vertical_chunks: 1,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stage {
    AfterUpdate,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub struct Drawing;

impl Plugin for AsciiTilemapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TilemapPlugin)
            .add_system(draw.system().label(Drawing))
            .add_startup_system(setup.system())
            .insert_resource(TilesToDraw(HashMap::default()));
    }
}

struct TileToDraw {
    texture_index: char,
    color: Color,
}

pub struct TilesToDraw(HashMap<UVec3, TileToDraw>);

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
    tiles_to_draw: ResMut<'a, TilesToDraw>,
}

impl<'a> DrawContext<'a> {
    /// sets a tile to a specific character
    pub fn set(&mut self, x: u32, y: u32, background: Color, foreground: Color, char: char) {
        if x >= self.settings.width || y >= self.settings.height {
            return;
        }

        // This makes sure the origin is at the top left of the tilemap
        let position = UVec2::new(x, self.settings.height as u32 - 1 - y);
        self.tiles_to_draw.0.insert(
            position.extend(BACKGROUND_LAYER_ID.into()),
            TileToDraw {
                color: background,
                texture_index: 219 as char, // ASCII code 219 = █ ( Block, graphic character )
            },
        );
        self.tiles_to_draw.0.insert(
            position.extend(FOREGROUND_LAYER_ID.into()),
            TileToDraw {
                color: foreground,
                texture_index: char,
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

    /// Clears the screen
    pub fn cls(&mut self) {
        self.cls_color(Color::BLACK);
    }

    /// Clears the screen with a specific color
    pub fn cls_color(&mut self, color: Color) {
        self.tile_query.for_each_mut(|mut tile| {
            tile.texture_index = 219; // ASCII code 219 = █ ( Block, graphic character )
            tile.color = color;
        });
    }
}
