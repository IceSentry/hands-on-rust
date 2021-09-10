use crate::DrawCommand;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::{Layer, LayerEntities};

#[derive(Debug, Clone)]
pub struct ActiveLayer(pub u16);

#[derive(SystemParam)]
pub struct DrawContext<'a> {
    layers: Query<'a, &'static mut Layer>,
    active_layer: ResMut<'a, ActiveLayer>,
    layer_entities: Res<'a, LayerEntities>,
}

impl<'a> DrawContext<'a> {
    pub fn set(&mut self, x: u32, y: u32, background: Color, foreground: Color, glyph: char) {
        let entity = self.layer_entities[self.active_layer.0 as usize];
        if let Ok(mut layer) = self.layers.get_mut(entity) {
            if x >= layer.size.x || y >= layer.size.y {
                // ignores anything out of bounds
                return;
            }

            layer.command_buffer.push(DrawCommand::DrawTile {
                x,
                y,
                background,
                foreground,
                glyph,
            });
        }
    }

    /// Prints a string at the given position with foreground and background color
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    #[allow(clippy::cast_possible_truncation)]
    pub fn print_color(
        &mut self,
        x: u32,
        y: u32,
        background: Color,
        foreground: Color,
        text: &str,
    ) {
        for (i, char) in text.chars().enumerate() {
            self.set(x + i as u32, y, background, foreground, char);
        }
    }

    /// prints a string centered on the x axis with foreground and background color
    #[allow(clippy::cast_possible_truncation)]
    pub fn print_color_centered(
        &mut self,
        y: u32,
        background: Color,
        foreground: Color,
        text: &str,
    ) {
        let size = self.get_active_layer_size();
        self.print_color(
            (size.x / 2) - (text.to_string().len() as u32 / 2),
            y,
            background,
            foreground,
            text,
        );
    }

    /// Prints a string at the given position
    /// if the string is longer than the viewport it will get truncated, wrapping is not handled
    pub fn print(&mut self, x: u32, y: u32, text: &str) {
        self.print_color(x, y, Color::BLACK, Color::WHITE, text);
    }

    /// prints a string centered on the x axis
    pub fn print_centered(&mut self, y: u32, text: &str) {
        self.print_color_centered(y, Color::BLACK, Color::WHITE, text);
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn bar_horizontal(
        &mut self,
        start_x: u32,
        start_y: u32,
        width: u32,
        filled_amount: u32,
        max: u32,
        background: Color,
        foreground: Color,
    ) {
        let percent = filled_amount as f32 / max as f32;
        let fill_width = (percent * width as f32) as u32;
        for x in 0..width {
            if x <= fill_width {
                self.set(start_x + x, start_y, background, foreground, 219 as char);
            } else {
                self.set(start_x + x, start_y, background, foreground, '░');
            }
        }
    }

    /// Clears the active layer
    pub fn cls(&mut self) {
        self.cls_color(Color::BLACK);
    }

    /// Clears the active layer with a specific color
    pub fn cls_color(&mut self, color: Color) {
        let entity = self.layer_entities[self.active_layer.0 as usize];
        if let Ok(mut layer) = self.layers.get_mut(entity) {
            layer.command_buffer.push(DrawCommand::ClearLayer { color });
        }
    }

    pub fn cls_all_layers(&mut self) {
        self.cls_color_all_layers(Color::BLACK);
    }

    pub fn cls_color_all_layers(&mut self, color: Color) {
        self.layers.for_each_mut(|mut layer| {
            layer.command_buffer.push(DrawCommand::ClearLayer { color });
        });
    }

    pub fn set_active_layer(&mut self, layer: u8) {
        self.active_layer.0 = u16::from(layer);
    }

    pub fn get_active_layer_size(&mut self) -> UVec2 {
        let entity = self.layer_entities[self.active_layer.0 as usize];
        let layer = self.layers.get_mut(entity).expect("layer not found");
        layer.size
    }
}
