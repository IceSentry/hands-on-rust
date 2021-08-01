use crate::{
    ascii_tilemap_plugin::{DrawContext, Drawing},
    rusty_dungeon_plugin::spawner::spawn_monster,
    DISPLAY_HEIGHT, DISPLAY_WIDTH, HEIGHT, WIDTH,
};
use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Instant,
};
use camera::Camera;
use map::MapBuilder;

use self::{
    spawner::spawn_player,
    systems::{
        collisions::collisions, end_turn::end_turn, entity_render::entity_render,
        map_render::map_render, movement::movement, player_input::player_input,
        random_move::random_move,
    },
};

mod camera;
mod components;
mod map;
mod spawner;
mod systems;

pub struct RustyDungeonPlugin;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct RenderSystem;

const NUM_ROOMS: u32 = 20;
const MIN_ROOM_SIZE: u32 = 2;
const MAX_ROOM_SIZE: u32 = 10;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonserTurn,
}

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_state(TurnState::AwaitingInput)
            .add_system_set(
                SystemSet::on_update(TurnState::AwaitingInput)
                    .before(RenderSystem)
                    .with_system(player_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(TurnState::PlayerTurn)
                    .before(RenderSystem)
                    .with_system(movement.system().label("movement"))
                    .with_system(collisions.system().label("collisions").after("movement"))
                    .with_system(end_turn.system().after("collisions")),
            )
            .add_system_set(
                SystemSet::on_update(TurnState::MonserTurn)
                    .before(RenderSystem)
                    .with_system(random_move.system().label("random_move"))
                    .with_system(movement.system().label("movement").after("random_move"))
                    .with_system(end_turn.system().after("movement")),
            )
            .add_system_set(
                SystemSet::new()
                    .label(RenderSystem)
                    .before(Drawing)
                    .with_system(map_render.system())
                    .with_system(entity_render.system())
                    .with_system(diagnostic.system()),
            )
            .add_system(clear_screen.system().before(RenderSystem).before(Drawing));
    }
}

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
    puffin::profile_function!();

    ctx.cls_all_layers();
}

fn diagnostic(mut ctx: DrawContext, diagnostics: ResMut<Diagnostics>) {
    puffin::profile_function!();

    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::value);
    if let Some(fps) = fps {
        ctx.set_active_layer(2);
        ctx.print(0, 0, &format!("FPS {:.0}", fps));
    }
}
