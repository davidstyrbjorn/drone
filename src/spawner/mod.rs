// Handles spawning entities

use crate::prelude::*;
use template::Templates;

mod template;

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
    guaranteed_monster_spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(
        ecs,
        rng,
        level,
        spawn_points,
        guaranteed_monster_spawn_points,
    );
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    // Pushes a new player with Player, Point and Render components
    ecs.push((
        Player {
            map_level: 0,
            wait_count: 8,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 1000,
            max: 1000,
        },
        FieldOfView::new(8),
        Damage(100),
    ));
}

pub fn spawn_telerportation_crystal(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        TelerportationCrystal,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('l'),
        },
        Name("Teleportation Crystal".to_string()),
    ));
}
