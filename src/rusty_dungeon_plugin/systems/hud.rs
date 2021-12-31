use crate::ascii_tilemap_plugin::DrawContext;
use crate::{
    rusty_dungeon_plugin::components::{Health, Player},
    LayerId, WIDTH,
};
use bevy::prelude::*;

#[allow(clippy::cast_sign_loss)]
pub fn hud(mut ctx: DrawContext, query: Query<&Health, With<Player>>) {
    let health = query.single().expect("no health for player");
    ctx.set_active_layer(LayerId::Hud as u8);
    ctx.print_centered(1, "Explore the dungeon. WASD or arrow keys to move.");
    ctx.bar_horizontal(
        0,
        0,
        WIDTH,
        health.current as u32,
        health.max as u32,
        Color::BLACK,
        Color::RED,
    );
    ctx.print_color_centered(
        0,
        Color::RED,
        Color::WHITE,
        &format!("Health: {} / {}", health.current, health.max),
    );
}
