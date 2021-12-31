use crate::rusty_dungeon_plugin::{
    components::{Enemy, Health, Player, Position, WantsToAttack, WantsToMove},
    TurnState,
};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn player_input(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Position), With<Player>>,
    enemy_query: Query<(Entity, &Position), With<Enemy>>,
    mut player_health_query: Query<&mut Health, With<Player>>,
    mut turn_state: ResMut<State<TurnState>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // puffin::profile_function!();
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
            let destination = Position((position.0.as_vec2() + delta).as_uvec2());
            let mut did_something = false;
            if delta.x != 0. || delta.y != 0. {
                let mut hit_something = false;
                for (enemy, _) in enemy_query.iter().filter(|(_, pos)| **pos == destination) {
                    hit_something = true;
                    did_something = true;
                    commands.spawn().insert(WantsToAttack {
                        attacker: player,
                        victim: enemy,
                    });
                }

                if !hit_something {
                    did_something = true;
                    commands.spawn().insert(WantsToMove {
                        entity: player,
                        destination,
                    });
                }
            }

            if !did_something {
                match player_health_query.get_mut(player) {
                    Ok(mut health) => health.current = i32::min(health.max, health.current + 1),
                    Err(e) => warn!("Failed to update health {}", e),
                };
            }
        });

        if let Err(e) = turn_state.set(TurnState::PlayerTurn) {
            warn!("Failed to set state {}", e);
        }
    }
}
