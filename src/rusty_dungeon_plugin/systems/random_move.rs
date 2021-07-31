use bevy::prelude::*;

use crate::rusty_dungeon_plugin::{components::MovingRandomly, map::Map};

pub fn random_move(map: Res<Map>, query: Query<&mut UVec2, With<MovingRandomly>>) {
    puffin::profile_function!();

    query.for_each_mut(|mut pos| {
        let rng = fastrand::Rng::new();
        let destination = match rng.u8(0..4) {
            0 => Vec2::new(-1., 0.),
            1 => Vec2::new(1., 0.),
            2 => Vec2::new(0., -1.),
            _ => Vec2::new(0., 1.),
        } + pos.as_f32();
        if map.can_enter_tile(destination.as_u32()) {
            *pos = destination.as_u32();
        }
    });
}
