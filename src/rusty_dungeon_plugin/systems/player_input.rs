use crate::rusty_dungeon_plugin::{
    components::{Enemy, Player, WantsToAttack, WantsToMove},
    TurnState,
};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn player_input(
    mut commands: Commands,
    player_query: Query<(Entity, &UVec2), With<Player>>,
    enemy_query: Query<(Entity, &UVec2), With<Enemy>>,
    mut turn_state: ResMut<State<TurnState>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    puffin::profile_function!();
    debug!("player_input");
    // Only process the first event
    if let Some(event) = keyboard_input_events.iter().find(|x| x.state.is_pressed()) {
        let delta = match event.key_code {
            Some(KeyCode::Left | KeyCode::A) => Vec2::new(-1., 0.),
            Some(KeyCode::Right | KeyCode::D) => Vec2::new(1., 0.),
            Some(KeyCode::Up | KeyCode::W) => Vec2::new(0., -1.),
            Some(KeyCode::Down | KeyCode::S) => Vec2::new(0., 1.),
            _ => Vec2::ZERO,
        };

        player_query.for_each_mut(|(player, position)| {
            let destination = (position.as_f32() + delta).as_u32();
            if let Some((enemy, _)) = enemy_query.iter().find(|(_, pos)| **pos == destination) {
                commands.spawn().insert(WantsToAttack {
                    attacker: player,
                    victim: enemy,
                });
            } else {
                commands.spawn().insert(WantsToMove {
                    entity: player,
                    destination,
                });
            }
        });

        if let Err(e) = turn_state.set(TurnState::PlayerTurn) {
            warn!("Failed to set state {}", e);
        }
    }
}
