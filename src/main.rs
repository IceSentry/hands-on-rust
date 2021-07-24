#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_pass_by_value,
    clippy::default_trait_access,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use ascii_tilemap_plugin::{AsciiTilemapPlugin, AsciiTilemapSettings};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod ascii_tilemap_plugin;
mod flappy_plugin;
mod rusty_dungeon_plugin;

// TODO
// * use layers for background and foreground
// * find a way to control the window dimension from the plugin or update the tilemap size on resize

pub const WIDTH: u32 = 80;
pub const HEIGHT: u32 = 50;

pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            // TODO find a way to control this by the plugin
            width: (WIDTH * TILE_WIDTH) as f32,
            height: (HEIGHT * TILE_HEIGHT) as f32,
            title: String::from("hands on dungeon crawler"),
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AsciiTilemapPlugin)
        .insert_resource(AsciiTilemapSettings {
            tilesheet_asset_path: "16x16-sb-ascii.png",
            width: WIDTH,
            height: HEIGHT,
            tile_width: TILE_WIDTH,
            tile_height: TILE_HEIGHT,
            ..Default::default()
        })
        // .add_plugin(flappy_plugin::FlappyPlugin)
        .add_plugin(rusty_dungeon_plugin::RustyDungeonPlugin)
        .run();
}
