use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Item)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Get renderables and field the player field of view
    let mut renderables_items = <(&Point, &Render, &Item)>::query().filter(!component::<Player>());
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    // Start a new DrawBatch in each system that writes to the terminal
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    renderables_items
        .iter(ecs)
        .filter(|(pos, _, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render, _)| {
            // Render the entity with render data at position
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch
        .submit(5000)
        .expect("Batch error, entity_render.rs");

    // Time to render players & enemies
    let mut player_batch = DrawBatch::new();
    player_batch.target(3);
    // Grab player + enemies
    <(&Point, &Render)>::query()
        .filter(component::<Player>() | component::<Enemy>())
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            player_batch.set(*pos - offset, render.color, render.glyph);
        });

    player_batch
        .submit(100000)
        .expect("Batch error (player_batch), entity_render.rs");
}
