use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, PartialOrd)]
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

pub enum PieceType {
    Red(Size),
    Green(Size),
    Blue(Size),
    Yellow(Size),
}

pub struct Piece {
    type_: PieceType,
    id: u8, // will be 0, 1, or 2 -- may want to change to an `enum`
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PieceType::Red(size) => {
                write!(f, "{} red", size)
            }
            PieceType::Green(size) => {
                write!(f, "{} green", size)
            }
            PieceType::Blue(size) => {
                write!(f, "{} blue", size)
            }
            PieceType::Yellow(size) => {
                write!(f, "{} yellow", size)
            }
        }
    }
}
