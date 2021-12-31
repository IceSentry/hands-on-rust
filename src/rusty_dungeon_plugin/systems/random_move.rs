use bevy::prelude::*;

use crate::rusty_dungeon_plugin::components::{
    Health, MovingRandomly, Player, Position, WantsToAttack, WantsToMove,
};

pub fn random_move(
    mut commands: Commands,
    mut movers: Query<(Entity, &Position), With<MovingRandomly>>,
    positions: Query<(Entity, &Position, &Health)>,
    player_query: Query<&Player>,
) {
    // puffin::profile_function!();
    movers.for_each_mut(|(entity, pos)| {
        let rng = fastrand::Rng::new();
        let destination = match rng.u8(0..4) {
            0 => IVec2::new(-1, 0),
            1 => IVec2::new(1, 0),
            2 => IVec2::new(0, -1),
            _ => IVec2::new(0, 1),
        } + pos.0.as_ivec2();
        let mut attacked = false;
        positions
            .iter()
            .filter(|(_, target_pos, _)| target_pos.0 == destination.as_uvec2())
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
                destination: Position(destination.as_uvec2()),
            });
        }
    });
}
