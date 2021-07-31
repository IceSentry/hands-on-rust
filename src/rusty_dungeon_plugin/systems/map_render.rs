use crate::{
    ascii_tilemap_plugin::DrawContext,
    rusty_dungeon_plugin::{
        camera::Camera,
        map::{Map, TileType},
    },
};
use bevy::prelude::*;

pub fn map_render(mut ctx: DrawContext, map: Res<Map>, camera: Res<Camera>) {
    puffin::profile_function!();

    ctx.set_active_layer(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            puffin::profile_scope!("inner loop");

            let pos = UVec2::new(x, y);
            if let Some(tile_type) = map.get_tile(pos) {
                let glyph = match tile_type {
                    TileType::Wall => '#',
                    TileType::Floor => '.',
                };
                let pos_offset = {
                    puffin::profile_scope!("pos offset");
                    pos - UVec2::new(camera.left_x, camera.top_y)
                };
                ctx.set(
                    pos_offset.x,
                    pos_offset.y,
                    Color::BLACK,
                    Color::WHITE,
                    glyph,
                );
            }
        }
    }
}
