use std::thread::spawn;

use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        // iter_mut lets us mutate the array
        map.tiles.iter_mut().for_each(|t| {
            // Bias towards Wall for a quicker iteration count
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn sprinkle_details(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let dirs: [Point; 4] = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ];
        // First sprinkle completely random points on the map
        map.tiles.iter_mut().for_each(|t| {
            if rng.range(0, 15) < 1 {
                *t = TileType::Floor2;
            }
        });

        let mut points_to_grow_grass: Vec<Point> = Vec::new();

        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let idx = map.point2d_to_index(Point::new(x, y));
                // Is it a grass tile?
                if map.tiles[idx] == TileType::Floor2 {
                    dirs.iter().for_each(|dir| {
                        let pos = Point::new(x, y) + *dir;
                        // Is neighbour in bound + a floor? turn it into grass
                        if map.in_bounds(pos) {
                            let neighbour_idx = map.point2d_to_index(pos);
                            if map.tiles[neighbour_idx] == TileType::Floor {
                                points_to_grow_grass.push(pos);
                            }
                        }
                    });
                }
            }
        }

        // Actually grow the grass
        points_to_grow_grass.iter().for_each(|pos| {
            let idx = map.point2d_to_index(*pos);
            map.tiles[idx] = TileType::Floor2;
        });

        // Sprinkle some random mushrooms on the map finally
        // First sprinkle completely random points on the map
        map.tiles.iter_mut().for_each(|t| {
            if rng.range(0, 15) < 1 {
                *t = TileType::Floor3;
            }
        });
    }

    // Very important help for the cellular automation algo.
    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                // Don't count current tile; only count its neighbors
                if !(dx == 0 && dy == 0)
                    && [TileType::Wall, TileType::Wall2]
                        .contains(&map.tiles[map_idx(x + dx, y + dy)])
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        // Clone the whole map
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                // The rules of Cellular Automation here
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance1)| distance.partial_cmp(&distance1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            monster_spawns: Vec::new(),
            guaranteed_monster_spawns: Vec::new(),
            player_start: Point::zero(),
            teleportation_crystal_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        // Do some functions calls, pass around mb to actually build the map
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.teleportation_crystal_start = start + Point::new(1, 0);
        // mb.teleportation_crystal_start = mb.find_most_distant();
        self.sprinkle_details(rng, &mut mb.map);

        mb
    }
}
