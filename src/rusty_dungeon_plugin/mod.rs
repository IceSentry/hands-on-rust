use crate::{
    ascii_tilemap_plugin::{DrawContext, Drawing},
    HEIGHT, WIDTH,
};
use bevy::{
    diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use map::{Map, MapBuilder};
use player::Player;

mod map;
mod player;

use map::{Map, MapBuilder};
use player::Player;

pub struct RustyDungeonPlugin;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct UpdateSystem;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(update.system().label(UpdateSystem).before(Drawing))
            .add_system(diagnostic.system().after(UpdateSystem).before(Drawing));
    }
}

const NUM_ROOMS: usize = 20;

fn startup(mut commands: Commands) {
    let mut rng = fastrand::Rng::new();
    rng.seed(42);
    let (map, player_start) = MapBuilder::new(NUM_ROOMS, WIDTH, HEIGHT, &mut rng).build();
        MapBuilder::new(NUM_ROOMS).build(WIDTH as usize, HEIGHT as usize, &mut rng);
    commands.insert_resource(map);
    commands.insert_resource(Player::new(player_start.x, player_start.y));
        player_start.x as usize,
        player_start.y as usize,
    ));
}

fn update(
    mut ctx: DrawContext,
    map: Res<Map>,
    keyboard_input: Res<Input<KeyCode>>,
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
