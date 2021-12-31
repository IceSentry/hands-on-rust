use crate::ascii_tilemap_plugin::DrawContext;
use crate::{
    rusty_dungeon_plugin::{
        camera::Camera,
        components::{Health, Name},
        CursorPos,
    },
    LayerId,
};
use bevy::prelude::*;

pub fn tooltips(
    mut ctx: DrawContext,
    query: Query<(Entity, &UVec2, &Name)>,
    health_query: Query<&Health>,
    cursor_pos: Res<CursorPos>,
    camera: Res<Camera>,
) {
    let cursor_position = match cursor_pos.0 {
        Some(cursor_pos) => cursor_pos,
        _ => return,
    };
    puffin::profile_function!();
    ctx.set_active_layer(LayerId::Hud as u8);

    let offset = IVec2::new(camera.left_x, camera.top_y);
    let map_pos = cursor_position.as_i32() + offset;
    for (entity, _, name) in query.iter().filter(|(_, pos, _)| **pos == map_pos.as_u32()) {
        let screen_pos = cursor_position * 2;
        let display = if let Ok(health) = health_query.get(entity) {
            format!("{} : {} hp", name.0, health.current)
        } else {
            name.0.clone()
        };
        ctx.print(screen_pos.x, screen_pos.y, &display);
    }
}
