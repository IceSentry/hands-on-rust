use bevy::{app::AppExit, prelude::*};

use crate::ascii_tilemap_plugin::{DrawContext, TilemapDrawing};
use crate::{HEIGHT, WIDTH};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    Playing,
    End,
}

pub struct FlappyPlugin;

impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Menu)
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(menu.before(TilemapDrawing)),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(clear_input))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(play.before(TilemapDrawing)),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(clear_input))
            .add_system_set(
                SystemSet::on_update(GameState::End).with_system(end.before(TilemapDrawing)),
            )
            .add_system_set(SystemSet::on_exit(GameState::End).with_system(clear_input))
            .add_event::<RestartEvent>()
            .add_system(restart)
            .insert_resource(Player::new(5, 25))
            .insert_resource(FrameTime(0.0))
            .insert_resource(Score(0))
            .insert_resource(Obstacle::new(WIDTH, 0));
    }
}

struct RestartEvent;

const FRAME_DURATION: f32 = 0.075;

struct FrameTime(f32);
struct Score(u32);

#[derive(Debug)]
struct Player {
    x: u32,
    y: u32,
    velocity: f32,
}

impl Player {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.set(0, self.y, Color::RED, Color::YELLOW, 1 as char);
    }

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn gravity_and_move(&mut self) {
        if self.velocity < 2. {
            self.velocity += 0.3;
        }
        // since usize doesn't have negative numbers casting the velocity didn't work
        self.y = (self.y as f32 + self.velocity) as u32;
        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -2.;
    }
}

struct Obstacle {
    x: u32,
    gap_y: u32,
    size: u32,
}

impl Obstacle {
    fn new(x: u32, score: u32) -> Self {
        Self {
            x,
            gap_y: fastrand::u32(10..40),
            size: u32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut DrawContext, player_x: u32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        let char = 177 as char; // ASCII code 177 = ??? ( Graphic character, medium density dotted )

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, Color::BLACK, Color::GREEN, char);
        }

        for y in self.gap_y + half_size..HEIGHT {
            ctx.set(screen_x, y, Color::BLACK, Color::GREEN, char);
        }
    }

    fn hit(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

fn clear_input(mut keyboard_input: ResMut<Input<KeyCode>>) {
    // puffin::profile_function!();
    keyboard_input.clear();
}

fn restart(
    mut state: ResMut<State<GameState>>,
    mut events: EventReader<RestartEvent>,
    mut player: ResMut<Player>,
    mut obstacle: ResMut<Obstacle>,
    mut score: ResMut<Score>,
) {
    // puffin::profile_function!();
    if events.iter().count() == 0 {
        return;
    }
    info!("Restarting...");
    state.set(GameState::Playing).expect("failed to set state");
    *player = Player::new(5, 25);
    *obstacle = Obstacle::new(WIDTH, 0);
    score.0 = 0;
}

fn menu(
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut restart_events: EventWriter<RestartEvent>,
) {
    // puffin::profile_function!();
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

#[allow(clippy::too_many_arguments)]
fn play(
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut ctx: DrawContext,
    mut frame_time: ResMut<FrameTime>,
    mut player: ResMut<Player>,
    mut obstacle: ResMut<Obstacle>,
    mut score: ResMut<Score>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // puffin::profile_function!();
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
    ctx.print(0, 1, &format!("Score: {}", score.0));

    obstacle.render(&mut ctx, player.x);
    if player.x > obstacle.x {
        score.0 += 1;
        *obstacle = Obstacle::new(player.x + WIDTH, score.0);
    }

    if player.y > HEIGHT || obstacle.hit(&player) {
        state.set(GameState::End).expect("failed to set state");
    }
}

fn end(
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut restart_events: EventWriter<RestartEvent>,
    score: Res<Score>,
) {
    // puffin::profile_function!();
    ctx.cls();
    ctx.print_centered(5, "You are dead");
    ctx.print_centered(6, &format!("You earned {} points", score.0));
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    if keyboard_input.just_pressed(KeyCode::P) {
        restart_events.send(RestartEvent);
    } else if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
    }
}
