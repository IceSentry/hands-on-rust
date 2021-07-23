use crate::ascii_tilemap_plugin::{DrawContext, HEIGHT, WIDTH};
use bevy::prelude::*;

mod map;
mod player;

use map::Map;
use player::Player;

use self::map::MapBuilder;

pub struct RustyDungeonPlugin;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(update.system());
    }
}

const NUM_ROOMS: usize = 20;

fn startup(mut commands: Commands) {
    let mut rng = fastrand::Rng::new();
    rng.seed(42);
    let (map, player_start) = MapBuilder::new(NUM_ROOMS).build(WIDTH, HEIGHT, &mut rng);
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
    player.render(&mut ctx)
}
