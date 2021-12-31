use bevy::prelude::*;
use fastrand::Rng;

use super::components::{Enemy, Health, MovingRandomly, Name, Player, Position, Render};
use crate::ascii_tilemap_plugin::color::GlyphColor;

pub fn spawn_player(commands: &mut Commands, position: Position) {
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

pub fn spawn_monster(commands: &mut Commands, rng: &mut Rng, position: Position) {
    let (hp, name, glyph) = match rng.u32(1..10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    commands
        .spawn()
        .insert(Enemy)
        .insert(position)
        .insert(MovingRandomly)
        .insert(Name(name))
        .insert(Health {
            current: hp,
            max: hp,
        })
        .insert(Render {
            color: GlyphColor::foreground(Color::WHITE),
            glyph,
        });
}

fn goblin() -> (i32, String, char) {
    (1, "Goblin".to_string(), 'g')
}

fn orc() -> (i32, String, char) {
    (2, "Orc".to_string(), 'o')
}
