use bevy::prelude::*;

use crate::rusty_dungeon_plugin::components::{Health, WantsToAttack};

pub fn combat(
    mut commands: Commands,
    attackers: Query<(Entity, &WantsToAttack)>,
    mut health: Query<&mut Health>,
) {
    puffin::profile_function!();
    debug!("combat");
    let victims = attackers
        .iter()
        .map(|(entity, attack)| (entity, attack.victim))
        .collect::<Vec<_>>();
    for (message, victim) in victims {
        if let Ok(mut health) = health.get_mut(victim) {
            info!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.entity(victim).despawn();
            }
            info!("Health after attack: {}", health.current);
        }
        commands.entity(message).despawn();
    }
}
