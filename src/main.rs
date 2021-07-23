#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use ascii_tilemap_plugin::{AsciiTilemapPlugin, AsciiTilemapSettings, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

mod ascii_tilemap_plugin;
mod flappy_plugin;
mod rusty_dungeon_plugin;

// TODO
// * use layers for background and foreground
// * find a way to control the window dimension from the plugin or update the tilemap size on resize

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("hands on dungeon crawler"),
            resizable: false,
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AsciiTilemapPlugin)
        .insert_resource(AsciiTilemapSettings {
            tilemap_asset_path: "16x16-sb-ascii.png",
        })
        // .add_plugin(flappy_plugin::FlappyPlugin)
        .add_plugin(rusty_dungeon_plugin::RustyDungeonPlugin)
        .run();
}
