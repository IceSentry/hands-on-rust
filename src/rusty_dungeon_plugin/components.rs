use ascii_tilemap_plugin::color::GlyphColor;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Render {
    pub color: GlyphColor,
    pub glyph: char,
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Enemy;

#[derive(Debug)]
pub struct MovingRandomly;

#[derive(Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: UVec2,
}

pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug)]
pub struct Name(pub String);

pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
