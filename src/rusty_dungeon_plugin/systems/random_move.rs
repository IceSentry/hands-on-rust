use bevy::prelude::*;

use crate::rusty_dungeon_plugin::components::{
    Health, MovingRandomly, Player, WantsToAttack, WantsToMove,
};

pub fn random_move(
    mut commands: Commands,
    movers: Query<(Entity, &UVec2), With<MovingRandomly>>,
    positions: Query<(Entity, &UVec2, &Health)>,
    player_query: Query<&Player>,
) {
    puffin::profile_function!();
    movers.for_each_mut(|(entity, pos)| {
        let rng = fastrand::Rng::new();
        let destination = match rng.u8(0..4) {
            0 => IVec2::new(-1, 0),
            1 => IVec2::new(1, 0),
            2 => IVec2::new(0, -1),
            _ => IVec2::new(0, 1),
        } + pos.as_i32();
        let mut attacked = false;
        positions
            .iter()
            .filter(|(_, target_pos, _)| **target_pos == destination.as_u32())
            .for_each(|(victim, _, _)| {
                if player_query.get(victim).is_ok() {
                    commands.spawn().insert(WantsToAttack {
                        attacker: entity,
                        victim,
                    });
                }
                attacked = true;
            });
        if !attacked {
            commands.spawn().insert(WantsToMove {
                entity,
                destination: destination.as_u32(),
            });
        }
    });
}
