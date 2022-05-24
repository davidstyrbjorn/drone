use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    // Find all entities with both point and chasing component
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    // Get the player
    let mut player = <(&Point, &Player)>::query();
    // Get all entities with point and health component
    let mut positions = <(Entity, &Point, &Health)>::query();

    // Extract player position and player idx from the player query tuple result
    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    // We use djikstra algorithm for pathfinding
    let search_targets = vec![player_idx];
    let djikstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = map_idx(pos.x, pos.y);
        // Gets the lowest cost tile pointing towards the player
        if let Some(desination) = DijkstraMap::find_lowest_exit(&djikstra_map, idx, map) {
            // Calculate the distance to the player
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            // Either get the player position or get a new tile to move to depending on distance
            // Helper: sqrt(2) > 1.2 so... figure that out...
            let destination = if distance > 1.2 {
                map.index_to_point2d(desination)
            } else {
                *player_pos
            };

            // Same as random move component from here pretty much!
            let mut attacked = false;
            // Go through all "hittable" entities to check if it's a player or blocked
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    // Is player?
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });
            // We can move!
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
