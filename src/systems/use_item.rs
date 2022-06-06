use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[read_component(ProvidesStun)]
#[read_component(Point)]
#[write_component(Stunned)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    // to-do list of healings
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    // Iterate activate item components etc etc
    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            // It is possible that the item does not exist so we do this
            if let Ok(item) = ecs.entry_ref(activate.item) {
                /* Decide what effect type it is */
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }
                if let Ok(_) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
                if let Ok(_) = item.get_component::<ProvidesStun>() {
                    // Look at neighbouring tiles and tell enemies that they are stunned

                    if let Ok(holder) = ecs.entry_ref(activate.used_by) {
                        if let Ok(pt) = holder.get_component::<Point>() {
                            let dirs = [
                                Point::new(1, 0),
                                Point::new(-1, 0),
                                Point::new(0, 1),
                                Point::new(0, -1),
                            ];
                            let attacked_positions = dirs.map(|dir| *pt + dir);
                            <(Entity, &Point)>::query()
                                .filter(component::<Enemy>())
                                .iter(ecs)
                                .filter(|(_, p)| attacked_positions.contains(*p))
                                .for_each(|(e, _)| {
                                    // Attach a Stunned component to the enemy
                                    commands.add_component(*e, Stunned(6));
                                });
                        }
                    }
                }
            }
            // Remove the message + the item entity
            commands.remove(activate.item);
            commands.remove(*entity);
        });

    // Go through the healing to-do list
    for heal in healing_to_apply.iter() {
        // Find the target (player for example) and make sure it exists
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            // Same thing really, make sure item has healing component
            if let Ok(health) = target.get_component_mut::<Health>() {
                // Heal for heal.1 (tuple where the second is amount, check struct)
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}
