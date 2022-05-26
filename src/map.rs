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
    pub revealed_tiles: Vec<bool>,
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
            revealed_tiles: vec![false; NUM_TILES],
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

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        // Check if destination is a walkable tile on our map
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

// Traits in rust are defined as the following
/*
A trait tells the Rust compiler about functionality a particular type has and can share with other types.
You can use traits to define shared behavior in an abstract way.
You can use trait bounds to specify that a generic can be any type that has certain behavior.
*/

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        // SmallVec is a bracket-lib optimization for small lists
        // Since Rust's Vec type can be resource intensive this is ideal for small cases such as this
        let mut exits = SmallVec::new(); // Rust is smart enough to deduce the type since we specified it in the return
        let location = self.index_to_point2d(idx);

        // Returns a list of available exits from a point idx with the second argument being weight
        // Check all 4 directions, enemies can only move orthogonal
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    // Is the tile a wall? (opaque)
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize = bounds
            .x
            .try_into()
            .expect("Not a valid usize. Did something go negative?");
        Point::new(idx % w, idx / w)
    }

    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}
