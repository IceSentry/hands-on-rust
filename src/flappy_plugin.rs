use bevy::{app::AppExit, prelude::*};

use crate::ascii_tilemap_plugin::DrawContext;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    Playing,
    End,
}

pub struct FlappyPlugin;

impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Menu)
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu.system()))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(clear_input.system()))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(play.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(clear_input.system()),
            )
            .add_system_set(SystemSet::on_update(GameState::End).with_system(end.system()))
            .add_system_set(SystemSet::on_exit(GameState::End).with_system(clear_input.system()));
    }
}

fn clear_input(mut keyboard_input: ResMut<Input<KeyCode>>) {
    keyboard_input.update();
}

fn restart(state: &mut State<GameState>) {
    info!("Changing state to Playing");
    state.set(GameState::Playing).expect("failed to set state");
}

fn menu(
    mut state: ResMut<State<GameState>>,
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Dragon");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    if keyboard_input.just_pressed(KeyCode::P) {
        restart(&mut state);
    } else if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}

fn play(mut state: ResMut<State<GameState>>) {
    info!("Changing state to End");
    state.set(GameState::End).expect("failed to set state");
}

fn end(
    mut state: ResMut<State<GameState>>,
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    ctx.cls();
    ctx.print(0, 0, "end");
    ctx.print_centered(5, "You are dead");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    if keyboard_input.just_pressed(KeyCode::P) {
        restart(&mut state);
    } else if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}
