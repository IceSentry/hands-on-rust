#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use ascii_tilemap_plugin::{PIXEL_HEIGHT, PIXEL_WIDTH};
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
            title: String::from("hands on flappy"),
            resizable: false,
            width: PIXEL_WIDTH as f32,
            height: PIXEL_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ascii_tilemap_plugin::AsciiTilemapPlugin)
        // .add_plugin(flappy_plugin::FlappyPlugin)
        .add_plugin(rusty_dungeon_plugin::RustyDungeonPlugin)
        .run();
}
