use bevy::prelude::*;

use crate::ascii_tilemap_plugin::DrawContext;

pub struct RustyDungeonPlugin;

impl Plugin for RustyDungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(hello.system());
    }
}

fn hello(mut ctx: DrawContext) {
    ctx.print(0, 0, "Hello World!");
}
