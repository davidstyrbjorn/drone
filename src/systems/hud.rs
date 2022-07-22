use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld, #[resource] event_log: &mut EventLog) {
    // Query time!
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    // As with every render system we make a new render batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Instructional text
    draw_batch.print_centered(
        2,
        "Cursor keys to move. G for pickup. 1-9 for item use", // .to_ascii_uppercase(),
    );
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
        format!(" HEALTH {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED3),
    );

    let (player, map_level, wait_count) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level, player.wait_count)))
        .unwrap();

    // Gangsters-unite (Drawing map level HUD)
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );
    let theme = match map_level {
        0 | 1 => "FOREST",
        2 | 3 => "CAVE",
        _ => "THE DUNGEON",
    };
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 2),
        format!("{}", theme),
        ColorPair::new(YELLOW, BLACK),
    );

    // Items HUD
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 4;
    // Draw the name for each item in the player's inventory
    item_query
        .iter(ecs)
        // Only grab the Carried components that are carried by the Player entity
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 3, &name.0));
            y += 1;
        });
    // Draw text only if we have an item
    if y > 4 {
        draw_batch.print_color(Point::new(3, 3), "Inventory", ColorPair::new(YELLOW, BLACK));
    }
    // Draw how many waits the player have left
    draw_batch.print_color(
        Point::new(3, 2),
        format!("Wait count: {}", wait_count),
        ColorPair::new(YELLOW, BLACK),
    );

    // Event log render
    let mut y = (SCREEN_HEIGHT * 2) - 14; // Shadowing the y variable from before
    draw_batch.print_color(Point::new(1, y), "EVENT LOG", ColorPair::new(YELLOW, BLACK));
    y += 1;
    event_log
        .messages
        .iter()
        .enumerate()
        .for_each(|(idx, msg)| {
            y += 1;
            let text = (idx + 1).to_string() + ". " + &msg.message.clone();
            draw_batch.print_color(Point::new(1, y), &text, msg.color);
        });

    draw_batch.submit(10000).expect("Batch error");
}
