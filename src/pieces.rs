use derive_new::new;
use enumset::EnumSetType;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, PartialOrd, Debug, Hash, Copy, Clone)]
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

#[derive(EnumSetType, Debug, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(new, Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct PieceType {
    color: Color,
    size: Size,
}

impl PieceType {
    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

#[derive(new, Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Piece {
    type_: PieceType,
    id: u8, // will be 0, 1, or 2 -- may want to change to an `enum`
}

impl Piece {
    pub fn type_(&self) -> &PieceType {
        &self.type_
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

pub struct PieceBank {
    pieces: HashMap<PieceType, HashSet<Piece>>,
}

impl PieceBank {
    pub fn new() -> Self {
        PieceBank {
            pieces: HashMap::new(),
        }
    }

    pub fn pop_piece(&mut self, piece_type: &PieceType) -> Option<Piece> {
        let piece_type_set: &mut _ = self.pieces.get_mut(piece_type)?;
        let piece = *piece_type_set.iter().next()?;
        piece_type_set.remove(&piece);
        Some(piece)
    }
}
