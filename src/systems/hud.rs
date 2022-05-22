use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    // Query time!
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    // As with every render system we make a new render batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Instructional text
    draw_batch.print_centered(1, "Fly around the Dungeon. Cursor keys to move.");
    // Draw health bar
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED3, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED3),
    );
    draw_batch.submit(10000).expect("Batch error");
}
