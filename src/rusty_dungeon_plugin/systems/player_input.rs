use crate::rusty_dungeon_plugin::{camera::Camera, components::Player, map::Map};
use bevy::prelude::*;

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut camera: ResMut<Camera>,
    player_query: Query<&mut UVec2, With<Player>>,
) {
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

    player_query.for_each_mut(|mut position| {
        let new_position = (position.as_f32() + delta).as_u32();
        if map.can_enter_tile(new_position) {
            *position = new_position;
            camera.on_player_move(new_position);
        }
    });
}
