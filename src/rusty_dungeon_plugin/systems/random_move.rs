use bevy::prelude::*;

use crate::rusty_dungeon_plugin::components::{MovingRandomly, WantsToMove};

pub fn random_move(mut commands: Commands, query: Query<(Entity, &UVec2), With<MovingRandomly>>) {
    puffin::profile_function!();
    query.for_each_mut(|(entity, pos)| {
        let rng = fastrand::Rng::new();
        let destination = match rng.u8(0..4) {
            0 => Vec2::new(-1., 0.),
            1 => Vec2::new(1., 0.),
            2 => Vec2::new(0., -1.),
            _ => Vec2::new(0., 1.),
        } + pos.as_f32();
        commands.spawn().insert(WantsToMove {
            entity,
            destination: destination.as_u32(),
        });
    });
}
