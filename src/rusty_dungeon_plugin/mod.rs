use crate::{ascii_tilemap_plugin::DrawContext, HEIGHT, WIDTH};
use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

mod map;
mod player;

use map::{Map, MapBuilder};
use player::Player;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stage {
    AfterUpdate,
}

pub struct RustyDungeonPlugin;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_stage_after(
                CoreStage::Update,
                Stage::AfterUpdate,
                SystemStage::parallel(),
            )
            .add_system(update.system())
            .add_system_to_stage(Stage::AfterUpdate, diagnostic.system());
    }
}

const NUM_ROOMS: usize = 20;

fn startup(mut commands: Commands) {
    let mut rng = fastrand::Rng::new();
    rng.seed(42);
    let (map, player_start) =
        MapBuilder::new(NUM_ROOMS).build(WIDTH as usize, HEIGHT as usize, &mut rng);
    commands.insert_resource(map);
    commands.insert_resource(Player::new(
        player_start.x as usize,
        player_start.y as usize,
    ));
}

fn update(
    map: Res<Map>,
    mut player: ResMut<Player>,
    mut ctx: DrawContext,
    keyboard_input: Res<Input<KeyCode>>,
) {
    ctx.cls();
    player.update(&map, &keyboard_input);
    map.render(&mut ctx);
    player.render(&mut ctx);
}

fn diagnostic(mut ctx: DrawContext, diagnostics: ResMut<Diagnostics>) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::average);
    if let Some(fps) = fps {
        ctx.print(0, 0, &format!("FPS {:.0}", fps));
    }
}