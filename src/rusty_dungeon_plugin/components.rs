use crate::ascii_tilemap_plugin::color::GlyphColor;

#[derive(Debug, Clone, Copy)]
pub struct Render {
    pub color: GlyphColor,
    pub glyph: char,
}

#[derive(Debug, Clone, Copy)]
pub struct Player;

#[derive(Debug, Clone, Copy)]
pub struct Enemy;
