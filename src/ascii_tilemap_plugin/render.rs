use super::TileData;
use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::{Chunk, Tile};

pub type RenderLayers = Vec<Vec<TileRenderData>>;

#[derive(Debug, Clone, Copy)]
pub struct TileRenderData {
    pub color: Color,
    pub glyph: char,
}
impl TileRenderData {
    pub fn new(color: Color, glyph: char) -> Self {
        Self { color, glyph }
    }
}
impl Default for TileRenderData {
    fn default() -> Self {
        Self::new(Color::BLACK, 0 as char)
    }
}

pub fn render(
    mut chunk_query: Query<&mut Chunk>,
    tile_query: Query<(&mut Tile, &TileData)>,
    render_layers: Res<RenderLayers>,
) {
    puffin::profile_function!();
    let mut chunks = HashSet::default();

    tile_query.for_each_mut(|(mut tile, tile_data)| {
        // puffin::profile_scope!("tile_query");
        let tile_render_data = &render_layers[tile_data.layer_id][tile_data.index];
        if tile.texture_index != tile_render_data.glyph as u16
            || tile.color != tile_render_data.color
        {
            // puffin::profile_scope!("update tile");
            tile.texture_index = tile_render_data.glyph as u16;
            tile.color = tile_render_data.color;
            chunks.insert(tile_data.chunk);
        }
    });

    for chunk_entity in chunks.drain() {
        if let Ok(mut chunk) = chunk_query.get_mut(chunk_entity) {
            puffin::profile_scope!("chunk need remesh");
            chunk.needs_remesh = true;
        }
    }
}
