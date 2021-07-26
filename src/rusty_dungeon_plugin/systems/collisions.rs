use crate::rusty_dungeon_plugin::components::{Enemy, Player};
use bevy::prelude::*;

pub fn collisions(
    mut commands: Commands,
    player_query: Query<&UVec2, With<Player>>,
    enemy_query: Query<(Entity, &UVec2), With<Enemy>>,
) {
    let player_pos = player_query.single().expect("player not found");
    enemy_query.for_each(|(entity, enemy_pos)| {
        if *enemy_pos == *player_pos {
            commands.entity(entity).despawn();
        }
    });
}
