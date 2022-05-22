use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// Clone for deep copy through .clone
// Copy overrides the default = operator to copy over the values instead of moving
// PartialEq lets us use == operator to compare
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    // Will use row-first encoding
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    // Helper to determine if point is safe in both dimensionality and is on a valid tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // Helper to determine if a point is inside of our map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0); // Enable the background layer
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    // Go from x,y to tile index with this helper function
                    let idx = map_idx(x, y);
                    // Map to something tileable
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                YELLOW,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                GREEN,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
