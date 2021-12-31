use crate::ascii_tilemap_plugin::DrawContext;
use crate::{
    rusty_dungeon_plugin::{
        camera::Camera,
        map::{Map, TileType},
    },
    LayerId,
};
use bevy::prelude::*;

pub fn map_render(mut ctx: DrawContext, map: Res<Map>, camera: Res<Camera>) {
    puffin::profile_function!();
    ctx.set_active_layer(LayerId::Map as u8);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pos = IVec2::new(x, y);
            if let Some(tile_type) = map.get_tile(pos.as_u32()) {
                let glyph = match tile_type {
                    TileType::Wall => '#',
                    TileType::Floor => '.',
                };
                let pos_offset = (pos - IVec2::new(camera.left_x, camera.top_y)).as_u32();
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
