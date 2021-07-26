use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphColor {
    pub background: Color,
    pub foreground: Color,
}

impl Default for GlyphColor {
    fn default() -> Self {
        Self::new(Color::BLACK, Color::WHITE)
    }
}

impl GlyphColor {
    pub fn new(background: Color, foreground: Color) -> Self {
        Self {
            background,
            foreground,
        }
    }
}
