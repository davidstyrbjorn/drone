use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
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

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    // Gangsters-unite (Drawing map level HUD)
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    // Items HUD
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    // Draw the name for each item in the player's inventory
    item_query
        .iter(ecs)
        // Only grab the Carried components that are carried by the Player entity
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });
    // Draw text only if we have an item
    if y > 3 {
        draw_batch.print_color(Point::new(3, 2), "Inventory", ColorPair::new(YELLOW, BLACK));
    }

    draw_batch.submit(10000).expect("Batch error");
}
