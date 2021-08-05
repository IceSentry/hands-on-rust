use bevy::prelude::*;

use crate::rusty_dungeon_plugin::{
    camera::Camera,
    components::{Player, WantsToMove},
    map::Map,
};

pub fn movement(
    mut commands: Commands,
    query: Query<(Entity, &WantsToMove)>,
    player_query: Query<(), With<Player>>,
    map: Res<Map>,
    mut camera: ResMut<Camera>,
) {
    puffin::profile_function!();
    query.for_each_mut(|(entity, wants_to_move)| {
        if map.can_enter_tile(wants_to_move.destination) {
            commands
                .entity(wants_to_move.entity)
                .insert(wants_to_move.destination);
            if player_query.get(wants_to_move.entity).is_ok() {
                camera.on_player_move(wants_to_move.destination);
            }
        }
        commands.entity(entity).despawn();
    });
}
