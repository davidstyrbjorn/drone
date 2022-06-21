use std::collections::HashSet;

// Legion components are usually structs but can also be enums such as Option<T>
// They dont have to explicilty have any functionality

pub use crate::prelude::*; // Bring in our prelude stuff

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

// Items can inflict damage but they aren't monsters so we need a way to indentify that an item is a weapon
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub wait_count: u8,
    pub map_level: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy; // Enemy tag component

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly; // Tag component as the others above

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TelerportationCrystal;

// Message component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

// Message compponent
#[derive(Clone, Debug, PartialEq)]
pub struct WantsToLog {
    pub log_entry: LogEntry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

// Message of intent for this as well, potentially we can let monsters use items if they want...
// what is the monster is below certain health and has a potion close to it, perhaps go for the potion instead
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesStun;

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

// Lasting effect
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stunned(pub i32);
