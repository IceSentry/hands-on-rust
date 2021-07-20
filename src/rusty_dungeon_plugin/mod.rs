use crate::ascii_tilemap_plugin::{DrawContext, HEIGHT, WIDTH};
use bevy::prelude::*;

mod map;
mod player;

use map::Map;
use player::Player;

pub struct RustyDungeonPlugin;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Map::new())
            .insert_resource(Player::new(WIDTH / 2, HEIGHT / 2))
            .add_system(update.system());
    }
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
