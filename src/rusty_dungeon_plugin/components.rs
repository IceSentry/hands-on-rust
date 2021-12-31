use crate::ascii_tilemap_plugin::color::GlyphColor;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct Position(pub UVec2);

#[derive(Debug, Clone, Copy, Component)]
pub struct Render {
    pub color: GlyphColor,
    pub glyph: char,
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub struct MovingRandomly;

#[derive(Debug, Component)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
