use crate::ascii_tilemap_plugin::DrawContext;

use crate::{
    rusty_dungeon_plugin::{
        camera::Camera,
        components::{Position, Render},
    },
    LayerId,
};
use bevy::prelude::*;

pub fn entity_render(
    mut ctx: DrawContext,
    camera: Res<Camera>,
    query: Query<(&Position, &Render)>,
) {
    // puffin::profile_function!();
    ctx.set_active_layer(LayerId::Entities as u8);
    let offset = IVec2::new(camera.left_x, camera.top_y);
    query.for_each(|(position, render)| {
        let draw_pos = ((*position).0.as_ivec2() - offset).as_uvec2();
        ctx.set(
            draw_pos.x,
            draw_pos.y,
            render.color.background,
            render.color.foreground,
            render.glyph,
        );
    });
}
