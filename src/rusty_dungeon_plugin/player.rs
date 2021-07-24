use bevy::prelude::*;

use crate::ascii_tilemap_plugin::DrawContext;

use super::map::Map;

pub struct Player {
    position: UVec2,
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: UVec2::new(x, y),
        }
    }

    pub fn render(&self, ctx: &mut DrawContext) {
        ctx.set(
            self.position.x,
            self.position.y,
            Color::BLACK,
            Color::WHITE,
            '@',
        );
    }

    pub fn update(&mut self, map: &Map, keyboard_input: &Input<KeyCode>) {
        let delta = if keyboard_input.just_pressed(KeyCode::Left)
            || keyboard_input.just_pressed(KeyCode::A)
        {
            Vec2::new(-1., 0.)
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            Vec2::new(1., 0.)
        } else if keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W)
        {
            Vec2::new(0., -1.)
        } else if keyboard_input.just_pressed(KeyCode::Down)
            || keyboard_input.just_pressed(KeyCode::S)
        {
            Vec2::new(0., 1.)
        } else {
            Vec2::ZERO
        };

        let new_position = (self.position.as_f32() + delta).as_u32();
        if map.can_enter_tile(new_position) {
            self.position = new_position;
        }
    }
}
