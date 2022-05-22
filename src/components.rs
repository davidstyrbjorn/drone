// Legion components are usually structs but can also be enums such as Option<T>
// They dont have to explicilty have any functionality

pub use crate::prelude::*; // Bring in our prelude stuff

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // Player does not have any fields, it become a tag component

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy; // Enemy tag component
