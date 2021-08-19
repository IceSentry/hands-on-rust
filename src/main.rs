#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_pass_by_value,
    clippy::default_trait_access,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use ascii_tilemap_plugin::AsciiTilemapPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;

use crate::ascii_tilemap_plugin::{LayerDataBuilder, TilemapBuilder};

mod ascii_tilemap_plugin;
mod flappy_plugin;
mod profiler_plugin;
mod rusty_dungeon_plugin;

pub const WIDTH: u32 = 80;
pub const HEIGHT: u32 = 50;

pub const DISPLAY_WIDTH: u32 = WIDTH / 2;
pub const DISPLAY_HEIGHT: u32 = HEIGHT / 2;

pub const TILE_WIDTH: u32 = 32;
pub const TILE_HEIGHT: u32 = 32;

pub const WINDOW_WIDTH: f32 = DISPLAY_WIDTH as f32 * TILE_WIDTH as f32;
pub const WINDOW_HEIGHT: f32 = DISPLAY_HEIGHT as f32 * TILE_HEIGHT as f32;

pub enum LayerId {
    Map = 0,
    Entities = 1,
    Hud = 2,
    Diagnostic = 3,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            // TODO find a way to control this by the plugin
            // if they don't match the map will not be aligned properly
            // or update the tilemap size on resize
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: String::from("hands on dungeon crawler"),
            // vsync: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::PINK))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(profiler_plugin::ProfilerPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(AsciiTilemapPlugin)
        .insert_resource(
            TilemapBuilder::new()
                .with_layer(
                    LayerDataBuilder::new(LayerId::Map as u16)
                        .texture_path("dungeonfont.png")
                        .size(DISPLAY_WIDTH, DISPLAY_HEIGHT)
                        .tile_size(TILE_WIDTH as f32, TILE_HEIGHT as f32),
                )
                .with_layer(
                    LayerDataBuilder::new(LayerId::Entities as u16)
                        .texture_path("dungeonfont.png")
                        .size(DISPLAY_WIDTH, DISPLAY_HEIGHT)
                        .tile_size(TILE_WIDTH as f32, TILE_HEIGHT as f32)
                        .is_transparent(true)
                        .is_background_transparent(true),
                )
                .with_layer(
                    LayerDataBuilder::new(LayerId::Hud as u16)
                        .texture_path("16x16-sb-ascii.png")
                        .size(WIDTH * 2, HEIGHT * 2)
                        .tile_size(8., 8.)
                        .is_transparent(true)
                        .is_background_transparent(true),
                )
                .with_layer(
                    LayerDataBuilder::new(LayerId::Diagnostic as u16)
                        .texture_path("16x16-sb-ascii.png")
                        .size(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2)
                        .tile_size(16., 16.)
                        .is_transparent(true)
                        .is_background_transparent(true),
                )
                .build(),
        )
        // .add_plugin(flappy_plugin::FlappyPlugin)
        .add_plugin(rusty_dungeon_plugin::RustyDungeonPlugin)
        .run();
}
