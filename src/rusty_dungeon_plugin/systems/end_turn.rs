use bevy::prelude::*;

use crate::rusty_dungeon_plugin::TurnState;

pub fn end_turn(mut turn_state: ResMut<State<TurnState>>) {
    // puffin::profile_function!();
    match turn_state.current() {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => turn_state.set(TurnState::MonserTurn),
        TurnState::MonserTurn => turn_state.set(TurnState::AwaitingInput),
    }
    .expect("Failed to set state");
}
