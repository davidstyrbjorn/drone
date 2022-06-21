// Nested module
// There will be some proc macros with alot of boilerplate code, ignore this for now
// as this is a very advanced Rust feature
use crate::prelude::*;

#[system]
// These decorators help Legion know which data we are planning on reading/writing to
#[read_component(Point)]
#[write_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut did_something = false;
    if let Some(key) = *key {
        // Get our movement vector
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            // Picking up an item?
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();

                // Check how many items we already got on us, used in the filter when checking for item we're standing on
                let number_of_carried_components = <(Entity, &Carried)>::query()
                    .iter(ecs)
                    .filter(|(_e, c)| c.0 == player)
                    .count();

                // Find the item we are on and and remove Point component
                // replace with Carried component
                let mut items = <(Entity, &Item, &Point)>::query();
                items
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| {
                        (item_pos == player_pos && number_of_carried_components <= 10)
                    })
                    .for_each(|(entity, _item, _item_pos)| {
                        // Ocotupus-preventing code for multiple weapons on player
                        if let Ok(e) = ecs.entry_ref(*entity) {
                            let res = e.get_component::<Weapon>();
                            if res.is_ok() {
                                // Query for other weapons and remove
                                <(Entity, &Carried, &Weapon)>::query()
                                    .iter(ecs)
                                    .filter(|(_, c, _)| c.0 == player)
                                    .for_each(|(entity, _c, _w)| {
                                        commands.remove(*entity);
                                    });
                            }
                        }

                        // Remove point and add carried to make it dissapear from map and be carried by player
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));
                    });

                Point::new(0, 0)
            }
            // Alot of ways to consume items
            VirtualKeyCode::Key1 => use_item(0, ecs, commands, &mut did_something),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands, &mut did_something),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands, &mut did_something),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands, &mut did_something),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands, &mut did_something),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands, &mut did_something),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands, &mut did_something),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands, &mut did_something),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands, &mut did_something),

            _ => Point::new(0, 0),
        };

        commands.push((
            (),
            WantsToLog {
                log_entry: {
                    LogEntry {
                        message: "STÅNGKÅT FTW".to_string(),
                        color: ColorPair::new(YELLOW, BLACK),
                    }
                },
            },
        ));

        // Grab player entity and destination
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    // We hit a enemy!
                    hit_something = true;
                    did_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            // If no filter matched in the above iterator we can move as usual
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
                did_something = true;
            }
        }

        *turn_state = TurnState::PlayerTurn;

        // TODO This seems a little buggy, look at later
        // if did_something {
        //     *turn_state = TurnState::PlayerTurn;
        // } else {
        //     // Check the wait counter on player
        //     let mut player = <&mut Player>::query().iter_mut(ecs).nth(0).unwrap();
        //     if player.wait_count > 0 {
        //         *turn_state = TurnState::PlayerTurn;
        //         player.wait_count -= 1;
        //     } else {
        //         *turn_state = TurnState::AwaitingInput;
        //     }
        // }
    }
}

fn use_item(
    n: usize,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    did_something: &mut bool,
) -> Point {
    // Find the player entity
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _)| Some(*entity))
        .unwrap();

    // Find the item at nth position in our inventory
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        // Make sure item_entity is not a weapon
        let mut is_weapon = false;
        *did_something = true;
        if let Ok(e) = ecs.entry_ref(item_entity) {
            let res = e.get_component::<Weapon>();
            if res.is_ok() {
                is_weapon = true;
            }
        }

        if !is_weapon {
            commands.push((
                (),
                ActivateItem {
                    used_by: player_entity,
                    item: item_entity,
                },
            ));
        }
    }

    Point::zero()
}
