use derive_new::new;
use enumset::EnumSet;
use enumset::EnumSetType;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumSetType, PartialOrd, Debug, Hash, EnumIter)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn all_sizes() -> EnumSet<Size> {
        Size::Small | Size::Medium | Size::Large
    }
}

// TODO consider using strum for this -- this was really just for me getting used to implementing
// the Display trait
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

#[derive(EnumSetType, Debug, Hash, EnumIter)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(new, Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct PieceType {
    size: Size,
    color: Color,
}

pub const SMALL_RED: PieceType = PieceType {
    size: Size::Small,
    color: Color::Red,
};

pub const MEDIUM_RED: PieceType = PieceType {
    size: Size::Medium,
    color: Color::Red,
};

pub const LARGE_RED: PieceType = PieceType {
    size: Size::Large,
    color: Color::Red,
};

pub const SMALL_YELLOW: PieceType = PieceType {
    size: Size::Small,
    color: Color::Yellow,
};

pub const MEDIUM_YELLOW: PieceType = PieceType {
    size: Size::Medium,
    color: Color::Yellow,
};

pub const LARGE_YELLOW: PieceType = PieceType {
    size: Size::Large,
    color: Color::Yellow,
};

pub const SMALL_GREEN: PieceType = PieceType {
    size: Size::Small,
    color: Color::Green,
};

pub const MEDIUM_GREEN: PieceType = PieceType {
    size: Size::Medium,
    color: Color::Green,
};

pub const LARGE_GREEN: PieceType = PieceType {
    size: Size::Large,
    color: Color::Green,
};

pub const SMALL_BLUE: PieceType = PieceType {
    size: Size::Small,
    color: Color::Blue,
};

pub const MEDIUM_BLUE: PieceType = PieceType {
    size: Size::Medium,
    color: Color::Blue,
};

pub const LARGE_BLUE: PieceType = PieceType {
    size: Size::Large,
    color: Color::Blue,
};

impl PieceType {
    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

#[derive(new, Debug, Eq, Hash, PartialEq, Clone)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceBank {
    pieces: HashMap<PieceType, Vec<Piece>>,
}

impl PieceBank {
    pub fn new() -> Self {
        let mut pieces: HashMap<PieceType, Vec<Piece>> = HashMap::new();
        let all_combos = Size::iter().cartesian_product(Color::iter());
        let all_combos = all_combos.cartesian_product(0..=2);
        for ((size, color), id) in all_combos {
            let type_ = PieceType { color, size };
            let piece = Piece { type_, id };
            let entry = pieces.entry(piece.type_);
            entry.or_default().push(piece);
        }

        PieceBank { pieces }
    }

    pub fn pop_piece(&mut self, piece_type: PieceType) -> Option<Piece> {
        let mut piece_type_vec = self.pieces.get_mut(&piece_type);
        // TODO figure out whether to treat this as an unrecoverable error in some other way than
        // panicking.
        piece_type_vec.unwrap().pop()
    }

    pub fn contains(&self, piece_type: PieceType) -> bool {
        self.pieces.contains_key(&piece_type)
    }
}
