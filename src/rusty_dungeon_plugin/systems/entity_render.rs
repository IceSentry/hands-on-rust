use crate::{
    ascii_tilemap_plugin::DrawContext,
    rusty_dungeon_plugin::{camera::Camera, components::Render},
};
use bevy::prelude::*;

pub fn entity_render(mut ctx: DrawContext, camera: Res<Camera>, query: Query<(&UVec2, &Render)>) {
    ctx.set_active_layer(1);
    let offset = UVec2::new(camera.left_x, camera.top_y);
    query.for_each(|(position, render)| {
        let draw_pos = *position - offset;
        ctx.set(
            draw_pos.x,
            draw_pos.y,
            render.color.background,
            render.color.foreground,
            render.glyph,
        );
    });
}
