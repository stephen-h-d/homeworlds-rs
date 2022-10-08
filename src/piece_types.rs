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

pub enum Piece {
    Red(Size),
    Green(Size),
    Blue(Size),
    Yellow(Size),
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Piece::Red(size) => {
                write!(f, "{} red", size)
            }
            Piece::Green(size) => {
                write!(f, "{} green", size)
            }
            Piece::Blue(size) => {
                write!(f, "{} blue", size)
            }
            Piece::Yellow(size) => {
                write!(f, "{} yellow", size)
            }
        }
    }
}
