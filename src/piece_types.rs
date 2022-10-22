use enumset::EnumSetType;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}

// TODO consider using this crate https://crates.io/crates/strum
impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Size::Small => {
                write!(f, "small")
            }
            Size::Medium => {
                write!(f, "medium")
            }
            Size::Large => {
                write!(f, "large")
            }
        }
    }
}

#[derive(EnumSetType, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug)]
pub struct PieceType {
    color: Color,
    size: Size,
}

impl PieceType {
    pub fn new(color: Color, size: Size) -> Self {
        PieceType { color, size }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

#[derive(Debug)]
pub struct Piece {
    type_: PieceType,
    id: u8, // will be 0, 1, or 2 -- may want to change to an `enum`
}

impl Piece {
    pub fn type_(&self) -> &PieceType {
        &self.type_
    }
}

impl Piece {
    pub fn new(type_: PieceType, id: u8) -> Self {
        Piece { type_, id }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.color {
            Color::Red => {
                write!(f, "{} red", self.size)
            }
            Color::Green => {
                write!(f, "{} green", self.size)
            }
            Color::Blue => {
                write!(f, "{} blue", self.size)
            }
            Color::Yellow => {
                write!(f, "{} yellow", self.size)
            }
        }
    }
}
