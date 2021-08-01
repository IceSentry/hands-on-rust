use super::components::{Enemy, Health, MovingRandomly, Player, Render};
use crate::ascii_tilemap_plugin::color::GlyphColor;
use bevy::prelude::*;
use fastrand::Rng;

pub fn spawn_player(commands: &mut Commands, position: UVec2) {
    commands
        .spawn()
        .insert(Player)
        .insert(position)
        .insert(Health {
            current: 20,
            max: 20,
        })
        .insert(Render {
            color: GlyphColor::default(),
            glyph: '@',
        });
}

pub fn spawn_monster(commands: &mut Commands, rng: &mut Rng, position: UVec2) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(position)
        .insert(MovingRandomly)
        .insert(Render {
            color: GlyphColor::foreground(Color::WHITE),
            glyph: match rng.u8(0..4) {
                0 => 'E',
                1 => 'O',
                2 => 'o',
                _ => 'g',
            },
        });
}
