use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Start a new DrawBatch in each system that writes to the terminal
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // Query for all entities that have a Point and Render component
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            // Render the entity with render data at position
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // z-index is high because we render entities on top of everything
    draw_batch.submit(5000).expect("Batch error");
}
