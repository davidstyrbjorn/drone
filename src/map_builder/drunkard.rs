use super::MapArchitect;
use crate::prelude::*;

pub struct DrunkardsWalkArchitect {}

const STAGGER_DISTANCE: usize = 300;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

impl DrunkardsWalkArchitect {
    fn carve_decorations(
        &mut self,
        start: usize,
        targets: Vec<usize>,
        rng: &mut RandomNumberGenerator,
        map: &mut Map,
    ) {
        let djikstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &vec![start], map, 2048.0);
        // Carve paths in the map from each target to start
        targets.iter().for_each(|target| {
            let mut current = target.clone();
            while current != start {
                if let Some(desitnation) =
                    DijkstraMap::find_lowest_exit(&djikstra_map, current, map)
                {
                    // Check distance
                    let pos1 = map.index_to_point2d(current);
                    let pos2 = map.index_to_point2d(start);
                    if DistanceAlg::Pythagoras.distance2d(pos1, pos2) < 1.2 {
                        break;
                    }
                    map.tiles[desitnation] = TileType::Floor2;
                    current = desitnation;
                }
            }
        });

        // Turn some tiles into decorated
        map.tiles
            .iter_mut()
            .filter(|t| (**t == TileType::Wall && rng.range(0, 10) > 5))
            .for_each(|t| *t = TileType::Wall2);
    }

    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            // Move in a direction
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            };

            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            guaranteed_monster_spawns: Vec::new(),
            teleportation_crystal_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        // Call a bunch of function and build the map
        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);
        // Walk until we have desired a percentage coverage
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            // Start a new drunkard at a random point on the map
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            // Make sure we always have a way to get to the center, making all sections available to the player
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.teleportation_crystal_start = mb.find_most_distant();

        // Carve some path decorations
        self.carve_decorations(
            mb.map.point2d_to_index(center),
            (&mb.monster_spawns)
                .into_iter()
                .map(|p| mb.map.point2d_to_index(*p))
                .collect(),
            rng,
            &mut mb.map,
        );

        mb
    }
}
