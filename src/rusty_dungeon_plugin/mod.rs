use crate::{
    ascii_tilemap_plugin::DrawContext, LayerId, DISPLAY_HEIGHT, DISPLAY_WIDTH, HEIGHT, TILE_HEIGHT,
    TILE_WIDTH, WIDTH,
};

use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Instant,
};

use camera::Camera;
use map::MapBuilder;
use spawner::{spawn_monster, spawn_player};
use systems::{
    combat::combat, end_turn::end_turn, entity_render::entity_render, hud::hud,
    map_render::map_render, movement::movement, player_input::player_input,
    random_move::random_move, tooltips::tooltips,
};

mod camera;
mod components;
mod map;
mod spawner;
mod systems;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct RenderSystem;

const NUM_ROOMS: u32 = 20;
const MIN_ROOM_SIZE: u32 = 2;
const MAX_ROOM_SIZE: u32 = 10;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonserTurn,
}
pub struct CursorPos(pub Option<UVec2>);

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stage {
    BeforeCombat,
    Combat,
    Movement,
    EndTurn,
}

pub struct RustyDungeonPlugin;
impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            // Setup stages
            .add_stage(Stage::Combat, SystemStage::parallel())
            .add_stage_before(Stage::Combat, Stage::BeforeCombat, SystemStage::parallel())
            .add_stage_after(Stage::Combat, Stage::Movement, SystemStage::parallel())
            .add_stage_after(Stage::Movement, Stage::EndTurn, SystemStage::parallel())
            // TurnState
            .insert_resource(State::new(TurnState::AwaitingInput))
            .add_system_set_to_stage(Stage::BeforeCombat, State::<TurnState>::get_driver())
            .add_system_set_to_stage(Stage::Combat, State::<TurnState>::get_driver())
            .add_system_set_to_stage(Stage::Movement, State::<TurnState>::get_driver())
            .add_system_set_to_stage(Stage::EndTurn, State::<TurnState>::get_driver())
            // AwaitingInput
            .add_system_set_to_stage(
                Stage::BeforeCombat,
                SystemSet::on_update(TurnState::AwaitingInput).with_system(player_input.system()),
            )
            // PlayerTurn
            .add_system_set_to_stage(
                Stage::Combat,
                SystemSet::on_update(TurnState::PlayerTurn).with_system(combat.system()),
            )
            .add_system_set_to_stage(
                Stage::Movement,
                SystemSet::on_update(TurnState::PlayerTurn).with_system(movement.system()),
            )
            // MonsterTurn
            .add_system_set_to_stage(
                Stage::BeforeCombat,
                SystemSet::on_update(TurnState::MonserTurn).with_system(random_move.system()),
            )
            .add_system_set_to_stage(
                Stage::Combat,
                SystemSet::on_update(TurnState::MonserTurn).with_system(combat.system()),
            )
            .add_system_set_to_stage(
                Stage::Movement,
                SystemSet::on_update(TurnState::MonserTurn).with_system(movement.system()),
            )
            // EndTurn
            .add_system_set_to_stage(
                Stage::EndTurn,
                SystemSet::new()
                    .label(RenderSystem)
                    .with_system(hud.system())
                    .with_system(map_render.system())
                    .with_system(entity_render.system())
                    .with_system(tooltips.system())
                    .with_system(diagnostic.system()),
            )
            .add_system_to_stage(Stage::EndTurn, clear_screen.system().before(RenderSystem))
            .add_system_to_stage(Stage::EndTurn, end_turn.system())
            .add_system(update_cursor.system());
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
    commands.insert_resource(CursorPos(None));

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

fn update_cursor(mut cursor_pos: ResMut<CursorPos>, windows: Res<Windows>) {
    puffin::profile_function!();
    cursor_pos.0 = windows
        .get_primary()
        .and_then(Window::cursor_position)
        .map(|cursor_position| {
            let mut pos = Vec2::new(
                (cursor_position.x / (TILE_WIDTH as f32)).floor(),
                (cursor_position.y / (TILE_HEIGHT as f32)).floor(),
            )
            .as_u32();

            if pos.y > DISPLAY_HEIGHT - 1 {
                pos.y = 0;
            } else {
                pos.y = DISPLAY_HEIGHT - 1 - pos.y;
            }
            pos
        });
}

fn diagnostic(mut ctx: DrawContext, diagnostics: Res<Diagnostics>) {
    puffin::profile_function!();
    if let Some(fps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::value)
    {
        ctx.set_active_layer(LayerId::Diagnostic as u8);
        ctx.print(0, 0, &format!("FPS {:.0}", fps));
    }
}
