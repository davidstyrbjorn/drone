use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Stunned)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // Player fov
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
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
        // Grab entity that has same position as mouse and is visible
        .filter(|(_, pos, _)| **pos == map_mouse_pos && player_fov.visible_tiles.contains(&pos))
        .for_each(|(entity, _, name)| {
            // Get that entities name
            let screen_pos = *mouse_pos * 4;
            // Check if entity has health component, otherwise just display name
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} / {} hp\n", &name.0, health.current, health.max)
                } else {
                    name.0.clone()
                };
            let stunned =
                if let Ok(stunned) = ecs.entry_ref(*entity).unwrap().get_component::<Stunned>() {
                    format!("Stunned ({})", stunned.0)
                } else {
                    "".to_string()
                };
            let res = display + &stunned;
            draw_batch.print(screen_pos, res);
        });

    draw_batch.submit(10100).expect("Batch error");
}
