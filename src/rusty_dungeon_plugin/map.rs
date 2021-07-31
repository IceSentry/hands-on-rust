use crate::{ascii_tilemap_plugin, HEIGHT, WIDTH};
use anyhow::{bail, Result};
use bevy::prelude::*;
use fastrand::Rng;
use std::ops::Range;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone)]
pub struct Map {
    tiles: Vec<TileType>,
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

    pub fn set_tile(&mut self, position: UVec2, tile: TileType) {
        puffin::profile_function!();

        if let Some(index) = self.try_index(position) {
            self.tiles[index] = tile;
        }
    }

    pub fn get_tile(&self, position: UVec2) -> Option<TileType> {
        puffin::profile_function!();

        self.try_index(position).map(|index| self.tiles[index])
    }

    pub fn in_bounds(&self, point: UVec2) -> bool {
        puffin::profile_function!();

        point.x < self.width && point.y < self.height
    }

    pub fn can_enter_tile(&self, point: UVec2) -> bool {
        puffin::profile_function!();

        self.in_bounds(point) && self.get_tile(point) == Some(TileType::Floor)
    }

    fn try_index(&self, point: UVec2) -> Option<usize> {
        if self.in_bounds(point) {
            Some(((point.y * self.width) + point.x) as usize)
        } else {
            None
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct MapBuilder<'a> {
    rooms: Vec<ascii_tilemap_plugin::geometry::Rect>,
    width: u32,
    height: u32,
    room_size: Range<u32>,
    rng: &'a mut Rng,
}

impl<'a> MapBuilder<'a> {
    pub fn new(
        room_count: u32,
        width: u32,
        height: u32,
        room_size: Range<u32>,
        rng: &'a mut Rng,
    ) -> Self {
        Self {
            rooms: Vec::with_capacity(room_count as usize),
            width,
            height,
            room_size,
            rng,
        }
    }

    fn build_random_rooms(&mut self, map: &mut Map) {
        let mut iteration = 0;
        let room_count = self.rooms.capacity();
        while self.rooms.len() < room_count && iteration < room_count * 2 {
            let room = ascii_tilemap_plugin::geometry::Rect::with_dimension(
                self.rng.u32(0..self.width - self.room_size.end),
                self.rng.u32(0..self.height - self.room_size.end),
                self.rng.u32(self.room_size.clone()),
                self.rng.u32(self.room_size.clone()),
            );

            if !self.rooms.iter().any(|r| r.intersect(&room)) {
                for point in room.points() {
                    map.set_tile(point, TileType::Floor);
                }
                self.rooms.push(room);
            }
            iteration += 1;
        }
    }

    fn build_vertical_tunnels(map: &mut Map, y1: u32, y2: u32, x: u32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            map.set_tile(UVec2::new(x, y), TileType::Floor);
        }
    }

    fn build_horizontal_tunnels(map: &mut Map, x1: u32, x2: u32, y: u32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            map.set_tile(UVec2::new(x, y), TileType::Floor);
        }
    }

    fn build_tunnels(&mut self, map: &mut Map) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if self.rng.bool() {
                Self::build_horizontal_tunnels(map, prev.x, new.x, prev.y);
                Self::build_vertical_tunnels(map, prev.y, new.y, new.x);
            } else {
                Self::build_vertical_tunnels(map, prev.y, new.y, prev.x);
                Self::build_horizontal_tunnels(map, prev.x, new.x, new.y);
            }
        }
    }

    pub fn build(&mut self) -> Result<(Map, UVec2, Vec<ascii_tilemap_plugin::geometry::Rect>)> {
        if self.width <= self.room_size.end || self.height <= self.room_size.end {
            bail!(
                "width and height must be higher than max room_size {}",
                self.room_size.end
            );
        }

        let mut map = Map::new(self.width, self.height);
        map.tiles.fill(TileType::Wall);

        self.build_random_rooms(&mut map);
        self.build_tunnels(&mut map);

        let player_start = self.rooms[0].center();
        Ok((map, player_start, self.rooms.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::timeout;

    #[test]
    #[timeout(50)]
    fn build() {
        let mut rng = fastrand::Rng::new();
        rng.seed(42);
        assert!(MapBuilder::new(20, 11, 11, 1..2, &mut rng).build().is_ok());
    }
}
