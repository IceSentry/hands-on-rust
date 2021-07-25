use crate::{
    ascii_tilemap_plugin::{DrawContext, Drawing},
    DISPLAY_HEIGHT, DISPLAY_WIDTH, HEIGHT, WIDTH,
};
use bevy::{
    core::FixedTimestep,
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Instant,
};
use camera::Camera;
use map::{Map, MapBuilder};
use player::Player;

mod camera;
mod map;
mod player;

pub struct RustyDungeonPlugin;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct UpdateSystem;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(update.system().label(UpdateSystem).before(Drawing)),
            )
            // .add_system(update.system().label(UpdateSystem).before(Drawing))
            .add_system(diagnostic.system().after(UpdateSystem).before(Drawing));
    }
}

const NUM_ROOMS: u32 = 20;
const MIN_ROOM_SIZE: u32 = 2;
const MAX_ROOM_SIZE: u32 = 10;

fn startup(mut commands: Commands) {
    info!("initializing rusty_dungeon...");
    let start = Instant::now();

    info!("Generating map...");
    let start_gen = Instant::now();

    let mut rng = fastrand::Rng::new();
    rng.seed(42);
    let (map, player_start) = MapBuilder::new(
        NUM_ROOMS,
        WIDTH,
        HEIGHT,
        MIN_ROOM_SIZE..MAX_ROOM_SIZE,
        &mut rng,
    )
    .build()
    .expect("failed to build the map");

    info!("Generating map...done {:?}", start_gen.elapsed());

    commands.insert_resource(map);
    commands.insert_resource(Player::new(player_start.x, player_start.y));
    commands.insert_resource(Camera::new(player_start, DISPLAY_WIDTH, DISPLAY_HEIGHT));

    info!("initializing rusty_dungeon...done {:?}", start.elapsed());
}

fn update(
    mut ctx: DrawContext,
    map: Res<Map>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<Player>,
    mut camera: ResMut<Camera>,
) {
    ctx.set_active_layer(0);
    ctx.cls();
    ctx.set_active_layer(1);
    ctx.cls();
    player.update(&map, &keyboard_input, &mut camera);
    map.render(&mut ctx, &camera);
    player.render(&mut ctx, &camera);
}

fn diagnostic(mut ctx: DrawContext, diagnostics: ResMut<Diagnostics>) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::average);
    if let Some(fps) = fps {
        ctx.print(0, 0, &format!("FPS {:.0}", fps));
    }
}
