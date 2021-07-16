use bevy::{app::AppExit, prelude::*};

use crate::ascii_tilemap_plugin::{DrawContext, HEIGHT};

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
            .add_system_set(SystemSet::on_exit(GameState::End).with_system(clear_input.system()))
            .add_event::<RestartEvent>()
            .add_system(restart.system())
            .insert_resource(Player::new(5, 25))
            .insert_resource(FrameTime(0.0));
    }
}

struct RestartEvent;

const FRAME_DURATION: f32 = 0.075;

struct FrameTime(f32);

#[derive(Debug)]
struct Player {
    x: usize,
    y: usize,
    velocity: f32,
}

impl Player {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.set(self.x, self.y, '@');
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2. {
            self.velocity += 0.3;
        }
        self.y = (self.y as f32 + self.velocity) as usize;
        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.;
    }
}

fn clear_input(mut keyboard_input: ResMut<Input<KeyCode>>) {
    keyboard_input.update();
}

fn restart(
    mut state: ResMut<State<GameState>>,
    mut events: EventReader<RestartEvent>,
    mut player: ResMut<Player>,
) {
    if events.iter().count() == 0 {
        return;
    }
    info!("Restarting...");
    state.set(GameState::Playing).expect("failed to set state");
    *player = Player::new(5, 25);
}

fn menu(
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut restart_events: EventWriter<RestartEvent>,
) {
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Dragon");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    if keyboard_input.just_pressed(KeyCode::P) {
        restart_events.send(RestartEvent);
    } else if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}

fn play(
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut ctx: DrawContext,
    mut frame_time: ResMut<FrameTime>,
    mut player: ResMut<Player>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    ctx.cls();
    frame_time.0 += time.delta_seconds();
    if frame_time.0 > FRAME_DURATION {
        frame_time.0 = 0.0;
        player.gravity_and_move();
    }

    if keyboard_input.pressed(KeyCode::Space) {
        player.flap();
    }

    player.render(&mut ctx);
    ctx.print(0, 0, "Press SPACE to flap.");

    if player.y > HEIGHT {
        state.set(GameState::End).expect("failed to set state");
    }
}

fn end(
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut restart_events: EventWriter<RestartEvent>,
) {
    ctx.cls();
    ctx.print(0, 0, "end");
    ctx.print_centered(5, "You are dead");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    if keyboard_input.just_pressed(KeyCode::P) {
        restart_events.send(RestartEvent);
    } else if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}
