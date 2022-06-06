use crate::prelude::*;

#[rustfmt::skip]
pub const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#-XX-#---
---#-XX-#---
-###----###-
------------
-###----###-
---#-XX-#---
---#-XX-#---
---######---
------------
", 12, 11
);

#[rustfmt::skip]
pub const MONSTER_FORTRESS: (&str, i32, i32) = ("
######
M----M
#----#
#X--X#
#-XX-#
######
", 6, 6);

pub const PREFAB_LIST: [(&str, i32, i32); 4] = [FORTRESS, FORTRESS, FORTRESS, MONSTER_FORTRESS];

pub fn apply_prefab(
    mb: &mut MapBuilder,
    rng: &mut RandomNumberGenerator,
    prefab: (&str, i32, i32),
) {
    let mut placement: Option<Point> = None;

    // Dijkstra map with our player start as start
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    // Repeatedly try to place our vault until we can give up after x attempts
    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        // Size of the prefab
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - prefab.1),
            rng.range(0, SCREEN_HEIGHT - prefab.2),
            prefab.1,
            prefab.2,
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            // Check if the point is reachable & the prefab won't fuck with the teleportation crystal
            if distance < 2000.0 && distance > 20.0 && mb.teleportation_crystal_start != pt {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            // Remove monster spawns that were inside the prefab points
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }
        attempts += 1;
    }

    // Is the placement Option not None?
    if let Some(placement) = placement {
        // Filter out enters and new lines from our map string
        let string_vec: Vec<char> = prefab
            .0
            .chars()
            .filter(|a| *a != '\r' && *a != '\n' && !a.is_whitespace())
            .collect();

        let mut i = 0;
        for ty in placement.y..placement.y + prefab.2 {
            for tx in placement.x..placement.x + prefab.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    'X' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.guaranteed_monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("apply_prefab doesn't know what to do with token: [{}]", c),
                }
                i += 1;
            }
        }
    }
}
