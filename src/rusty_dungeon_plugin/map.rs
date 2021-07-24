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
    width: u32,
    height: u32,
}

impl Default for Map {
    fn default() -> Self {
        Self::new(WIDTH, HEIGHT)
    }
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Floor; (width * height) as usize],
        }
    }

    pub fn render(&self, ctx: &mut DrawContext) {
        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
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
        (point.x) < WIDTH && (point.y) < HEIGHT
    }

    pub fn can_enter_tile(&self, point: UVec2) -> bool {
        self.in_bounds(point) && self.tiles[Map::index(point.x, point.y)] == TileType::Floor
    }

    pub fn try_index(&self, point: UVec2) -> Option<usize> {
        if self.in_bounds(point) {
            Some(Map::index(point.x, point.y))
        } else {
            None
        }
    }

    fn index(x: u32, y: u32) -> usize {
        ((y * WIDTH) + x) as usize
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct MapBuilder<'a> {
    rooms: Vec<ascii_tilemap_plugin::geometry::Rect>,
    width: u32,
    height: u32,
    rng: &'a mut Rng,
}

impl<'a> MapBuilder<'a> {
    pub fn new(room_count: usize, width: u32, height: u32, rng: &'a mut Rng) -> Self {
        Self {
            rooms: Vec::with_capacity(room_count),
            width,
            height,
            rng,
        }
    }

    fn build_random_rooms(&mut self, map: &mut Map) {
        while self.rooms.len() < self.rooms.capacity() {
            let room = ascii_tilemap_plugin::geometry::Rect::with_dimension(
                self.rng.u32(1..WIDTH - 10),
                self.rng.u32(1..HEIGHT - 10),
                self.rng.u32(2..10),
                self.rng.u32(2..10),
            );

            if !self.rooms.iter().any(|r| r.intersect(&room)) {
                for point in room.points() {
                    if let Some(index) = map.try_index(point) {
                        map.tiles[index] = TileType::Floor;
                    } else {
                        println!("tile not found at {}", point);
                    }
                }
                self.rooms.push(room);
            }
        }
    }

    fn build_vertical_tunnels(&self, map: &mut Map, y1: u32, y2: u32, x: u32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = map.try_index(UVec2::new(x, y)) {
                map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn build_horizontal_tunnels(&self, map: &mut Map, x1: u32, x2: u32, y: u32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = map.try_index(UVec2::new(x, y)) {
                map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn build_tunnels(&mut self, map: &mut Map) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if self.rng.u32(0..2) == 1 {
                self.build_horizontal_tunnels(map, prev.x, new.x, prev.y);
                self.build_vertical_tunnels(map, prev.y, new.y, new.x);
            } else {
                self.build_vertical_tunnels(map, prev.y, new.y, prev.x);
                self.build_horizontal_tunnels(map, prev.x, new.x, new.y);
            }
        }
    }

    pub fn build(&mut self) -> (Map, UVec2) {
        let mut map = Map::new(self.width, self.height);
        map.tiles.fill(TileType::Wall);

        self.build_random_rooms(&mut map);
        self.build_tunnels(&mut map);

        let player_start = self.rooms[0].center();
        (map, player_start)
    }
}
