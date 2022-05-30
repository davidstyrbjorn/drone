use crate::prelude::*;
use themes::*;

use self::{
    automata::CellularAutomataArchitect, drunkard::DrunkardsWalkArchitect, prefab::apply_prefab,
    rooms::RoomArchitect,
};

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

// A large part of how Rust enforced safe concurrency between threads is through these new types 'Sync' and 'Send'
// Sync lets us safe access this object from multipile threads
// Send lets us safe share this object between threads
pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub teleportation_crystal_start: Point,
    pub monster_spawns: Vec<Point>,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut theme_name: &str = "dungeon";
        // Build map and select appropiate theme
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => {
                // TODO: make this cave themed
                theme_name = "forest";
                Box::new(DrunkardsWalkArchitect {})
            }
            1 => {
                theme_name = "dungeon";
                Box::new(RoomArchitect {})
            }
            _ => {
                theme_name = "forest";
                Box::new(CellularAutomataArchitect {})
            }
        };

        let mut mb = architect.new(rng);
        apply_prefab(&mut mb, rng);

        match theme_name {
            "dungeon" => mb.theme = DungeonTheme::new(),
            "forest" => mb.theme = ForestTheme::new(),
            _ => mb.theme = DungeonTheme::new(),
        }

        mb
    }

    fn fill(&mut self, tile: TileType) {
        // Lambda function passed to for each which operatoes on the mutable itertor
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let djikstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;
        // TODO: I don't really get this, try to figure this out later
        self.map.index_to_point2d(
            djikstra_map
                .map
                .iter()
                .enumerate() // Returns (index, distance)
                .filter(|(_, dist)| *dist < UNREACHABLE)
                // we use max_by because we have tuples, but we want to compare e.1, and then grab e.0 (index)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // Check if this new randomly generated room intersects an existing
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // If the new room does not overlap an existing one we can continue
            if !overlap {
                // Loop through and change the actual map to floor TileType
                room.for_each(|p: Point| {
                    if self.map.in_bounds(p) {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // Get a mutable copy of the rooms array
        let mut rooms = self.rooms.clone();
        // Vectors include a sort_by to sort elements.
        // It takes a closure (lambda) which calls the cmp function
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // Iterate over the rooms, skip first
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // Randomly choose if we dig first horizontal then vertical or vice versa
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y)
            }
        }
    }

    // Carves a tunnel from y1 to y2 on the map
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        // Start on the minimum and move to to the maximum y coord
        for y in min(y1, y2)..=max(y1, y2) {
            // Probe map to see if x,y exists as index
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                // if so turn it into floor
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // Carve a tunnel from x1 to x2 on the map
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // Returns a vector of spawn points atleast 10 points away from the player
    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            if spawnable_tiles.is_empty() {
                break;
            }
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }

        spawns
    }
}
