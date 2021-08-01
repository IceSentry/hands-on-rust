use crate::ascii_tilemap_plugin::color::GlyphColor;
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
