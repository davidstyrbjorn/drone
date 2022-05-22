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

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // Player does not have any fields, it become a tag component

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy; // Enemy tag component

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly; // Tag component as the others above

// Message component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
