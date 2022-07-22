use super::MapArchitect;
use crate::prelude::*;

const NUM_ROOMS: usize = 30;

pub struct RoomArchitect {
    pub rooms: Vec<Rect>,
}

impl RoomArchitect {
    fn carve_decorations(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        // Turn some floor tiles into decorated, complete 50/50 chance
        map.tiles
            .iter_mut()
            .filter(|t| (**t == TileType::Floor && rng.range(0, 11) > 5))
            .for_each(|t| *t = TileType::Floor2);

        // Turn some wall tiles into decorated, favor-non decorated wall tiles
        map.tiles
            .iter_mut()
            .filter(|t| (**t == TileType::Wall && rng.range(0, 11) > 6))
            .for_each(|t| *t = TileType::Wall2);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 6),
                rng.range(1, SCREEN_HEIGHT - 6),
                rng.range(2, 6),
                rng.range(2, 6),
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
                    if map.in_bounds(p) {
                        let idx = map_idx(p.x, p.y);
                        map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
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
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y, map);
                self.apply_vertical_tunnel(prev.y, new.y, new.x, map);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x, map);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y, map)
            }
        }
    }

    // Carves a tunnel from y1 to y2 on the map
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32, map: &mut Map) {
        use std::cmp::{max, min};
        // Start on the minimum and move to to the maximum y coord
        for y in min(y1, y2)..=max(y1, y2) {
            // Probe map to see if x,y exists as index
            if let Some(idx) = map.try_idx(Point::new(x, y)) {
                // if so turn it into floor
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // Carve a tunnel from x1 to x2 on the map
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32, map: &mut Map) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = map.try_idx(Point::new(x, y)) {
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

impl MapArchitect for RoomArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            guaranteed_monster_spawns: Vec::new(),
            teleportation_crystal_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        mb.fill(TileType::Wall);
        self.build_random_rooms(rng, &mut mb.map);
        self.build_corridors(rng, &mut mb.map);
        mb.player_start = self.rooms[0].center();
        mb.teleportation_crystal_start = mb.find_most_distant();
        for room in self.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        self.carve_decorations(rng, &mut mb.map);

        mb
    }
}
