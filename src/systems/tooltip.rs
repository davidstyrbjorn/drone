use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // Grab entities that have both point and name component
    let mut positions = <(Entity, &Point, &Name)>::query();

    // Grab mouse position according to camera offset
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_mouse_pos = *mouse_pos + offset;

    // As with every render system we create a new DrawBatch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_mouse_pos) // Grab entity that has same position as mouse
        .for_each(|(entity, _, name)| {
            // Get that entities name
            let screen_pos = *mouse_pos * 4;
            // Check if entity has health component, otherwise just display name
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} / {} hp", &name.0, health.current, health.max)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
