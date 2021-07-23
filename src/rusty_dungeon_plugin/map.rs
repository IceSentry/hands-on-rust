use crate::{
    ascii_tilemap_plugin::{self, DrawContext},
    HEIGHT, WIDTH,
};
use bevy::prelude::*;
use fastrand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Default for Map {
    fn default() -> Self {
        Self::new(WIDTH as usize, HEIGHT as usize)
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![TileType::Floor; width * height],
        }
    }

    pub fn render(&self, ctx: &mut DrawContext) {
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                let index = Map::index(x, y);
                match self.tiles[index] {
                    TileType::Floor => ctx.set(x, y, Color::BLACK, Color::YELLOW, '.'),
                    TileType::Wall => ctx.set(x, y, Color::BLACK, Color::GREEN, '#'),
                }
            }
        }
    }

    pub fn in_bounds(&self, point: UVec2) -> bool {
        (point.x as usize) < WIDTH as usize && (point.y as usize) < HEIGHT as usize
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
        (y * WIDTH as usize) + x
    }
}

pub struct MapBuilder {
    rooms: Vec<ascii_tilemap_plugin::geometry::Rect>,
}

impl MapBuilder {
    pub fn new(room_count: usize) -> Self {
        Self {
            rooms: Vec::with_capacity(room_count),
        }
    }

    fn build_random_rooms(&mut self, map: &mut Map, rng: &mut Rng) {
        while self.rooms.len() < self.rooms.capacity() {
            let room = ascii_tilemap_plugin::geometry::Rect::with_dimension(
                rng.usize(1..WIDTH as usize - 10),
                rng.usize(1..HEIGHT as usize - 10),
                rng.usize(2..10),
                rng.usize(2..10),
            );

            if !self.rooms.iter().any(|r| r.intersect(&room)) {
                for point in room.points() {
                    if let Some(index) = map.try_index(point) {
                        map.tiles[index] = TileType::Floor
                    } else {
                        println!("tile not found at {}", point);
                    }
                }
                self.rooms.push(room);
            }
        }
    }

    fn build_vertical_tunnels(&mut self, map: &mut Map, y1: usize, y2: usize, x: usize) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = map.try_index(UVec2::new(x as u32, y as u32)) {
                map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn build_horizontal_tunnels(&mut self, map: &mut Map, x1: usize, x2: usize, y: usize) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = map.try_index(UVec2::new(x as u32, y as u32)) {
                map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn build_tunnels(&mut self, map: &mut Map, rng: &mut Rng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.usize(0..2) == 1 {
                self.build_horizontal_tunnels(
                    map,
                    prev.x as usize,
                    new.x as usize,
                    prev.y as usize,
                );
                self.build_vertical_tunnels(map, prev.y as usize, new.y as usize, new.x as usize);
            } else {
                self.build_vertical_tunnels(map, prev.y as usize, new.y as usize, prev.x as usize);
                self.build_horizontal_tunnels(map, prev.x as usize, new.x as usize, new.y as usize);
            }
        }
    }

    pub fn build(&mut self, width: usize, height: usize, rng: &mut Rng) -> (Map, UVec2) {
        let mut map = Map::new(width, height);
        map.tiles.fill(TileType::Wall);
        self.build_random_rooms(&mut map, rng);
        self.build_tunnels(&mut map, rng);
        let player_start = self.rooms[0].center();
        (map, player_start)
    }
}
