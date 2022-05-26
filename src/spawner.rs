// Handles spawning entities

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    // Pushes a new player with Player, Point and Render components
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 100,
            max: 100,
        },
        FieldOfView::new(8),
    ));
}

// Some helpers for enemy types
fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    // Get some data for our monster by rolling a dice
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6),
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
