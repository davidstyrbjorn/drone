use crate::prelude::*;

// The CommandBuffer is a way to insert instructions for Legion to perform after the system is finished running
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    // Grab all Position + Player combinations and get that position component
    let mut players = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);

    // Next, get all enemy positions through a similar query
    // We need Entity here because we remove entities using that field (u64)
    let mut enemies = <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
