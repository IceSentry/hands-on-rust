use bevy::prelude::*;

use crate::ascii_tilemap_plugin::DrawContext;

use super::{camera::Camera, map::Map};

pub struct Player {
    position: UVec2,
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: UVec2::new(x, y),
        }
    }

    pub fn render(&self, ctx: &mut DrawContext, camera: &Camera) {
        ctx.set_active_layer(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            Color::BLACK,
            Color::WHITE,
            '@',
        );
    }

    pub fn update(&mut self, map: &Map, keyboard_input: &Input<KeyCode>, camera: &mut Camera) {
        let delta = if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            Vec2::new(-1., 0.)
        } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            Vec2::new(1., 0.)
        } else if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            Vec2::new(0., -1.)
        } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            Vec2::new(0., 1.)
        } else {
            Vec2::ZERO
        };

        let new_position = (self.position.as_f32() + delta).as_u32();
        if map.can_enter_tile(new_position) {
            self.position = new_position;
            camera.on_player_move(new_position);
        }
    }
}
