use crate::prelude::*;

pub struct DungeonTheme {}
pub struct ForestTheme {}
pub struct CaveTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl CaveTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

// Note here:
// tile_to_render is a pure function
// pure functions only operates on its inputs and doesn't store any state
// pure functions are always safe to use in a multi-threaded context, since there is nothing to sync between threads
// if you do need to store state, investigate 'syncronization primitives' in particular 'Mutex' and 'Atomic'

/* MAP THEME TRAIT IMPLEMENTATIONS STARTS HERE */

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        // TODO: Map Floor 2 + Floor 3 to decoration
        match tile_type {
            TileType::Floor | TileType::Floor2 | TileType::Floor3 => to_cp437('M'),
            TileType::Wall | TileType::Wall2 => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('='),
            TileType::Floor2 => to_cp437(';'),
            TileType::Floor3 => to_cp437('<'),
            TileType::Wall | TileType::Wall2 => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl MapTheme for CaveTheme {
    // TODO: Map Floor 2 + Floor 3 to decoration
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('-'),
            TileType::Floor2 | TileType::Floor3 => to_cp437(','),
            TileType::Wall => to_cp437('\\'),
            TileType::Wall2 => to_cp437('$'),
            TileType::Exit => to_cp437('>'),
        }
    }
}
