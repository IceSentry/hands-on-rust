use crate::{
    ascii_tilemap_plugin::{DrawContext, Drawing},
    rusty_dungeon_plugin::spawner::spawn_monster,
    DISPLAY_HEIGHT, DISPLAY_WIDTH, HEIGHT, WIDTH,
};
use bevy::{
    core::FixedTimestep,
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Instant,
};
use camera::Camera;
use map::MapBuilder;

use self::{
    spawner::spawn_player,
    systems::{
        collisions::collisions, entity_render::entity_render, map_render::map_render,
        player_input::player_input,
    },
};

mod camera;
mod components;
mod map;
mod spawner;
mod systems;

pub struct RustyDungeonPlugin;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct ClearScreenSystem;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(30.0))
                    .with_system(player_input.system())
                    .with_system(collisions.system()),
            )
            .add_system(map_render.system().before(ClearScreenSystem))
            .add_system(entity_render.system().before(ClearScreenSystem))
            .add_system(diagnostic.system().before(ClearScreenSystem))
            .add_system(
                clear_screen
                    .system()
                    .label(ClearScreenSystem)
                    .before(Drawing),
            );
    }
}

const NUM_ROOMS: u32 = 20;
const MIN_ROOM_SIZE: u32 = 2;
const MAX_ROOM_SIZE: u32 = 10;

fn startup(mut commands: Commands) {
    info!("initializing rusty_dungeon...");
    let start = Instant::now();

    let mut rng = fastrand::Rng::new();
    rng.seed(42);

    let (map, player_start, rooms) = MapBuilder::new(
        NUM_ROOMS,
        WIDTH,
        HEIGHT,
        MIN_ROOM_SIZE..MAX_ROOM_SIZE,
        &mut rng,
    )
    .build()
    .expect("failed to build the map");

    commands.insert_resource(map);
    commands.insert_resource(Camera::new(player_start, DISPLAY_WIDTH, DISPLAY_HEIGHT));

    spawn_player(&mut commands, player_start);
    for pos in rooms.iter().skip(1).map(|r| r.center()) {
        spawn_monster(&mut commands, &mut rng, pos);
    }

    info!("initializing rusty_dungeon...done {:?}", start.elapsed());
}

fn clear_screen(mut ctx: DrawContext) {
    ctx.cls_all_layers();
}

fn diagnostic(mut ctx: DrawContext, diagnostics: ResMut<Diagnostics>) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::average);
    if let Some(fps) = fps {
        ctx.set_active_layer(2);
        ctx.print(0, 0, &format!("FPS {:.0}", fps));
    }
}
