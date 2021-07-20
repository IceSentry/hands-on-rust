use crate::ascii_tilemap_plugin::{DrawContext, HEIGHT, WIDTH};
use bevy::prelude::*;

pub const NUM_TILES: usize = WIDTH * HEIGHT;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut DrawContext) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = Map::index(x, y);
                match self.tiles[index] {
                    TileType::Floor => ctx.set(x, y, Color::BLACK, Color::YELLOW, '.'),
                    TileType::Wall => ctx.set(x, y, Color::BLACK, Color::GREEN, '#'),
                }
            }
        }
    }

    pub fn in_bounds(&self, point: UVec2) -> bool {
        (point.x as usize) < WIDTH && (point.y as usize) < HEIGHT
    }

    pub fn can_enter_tile(&self, point: UVec2) -> bool {
        self.in_bounds(point)
            && self.tiles[Map::index(point.x as usize, point.y as usize)] == TileType::Floor
    }

    pub fn try_index(&self, point: UVec2) -> Option<usize> {
        if self.in_bounds(point) {
            Some(Map::index(point.x as usize, point.y as usize))
        } else {
            None
        }
    }

    fn index(x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }
}
