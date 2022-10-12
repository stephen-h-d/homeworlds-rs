use super::piece_types::Piece;

use std::collections::HashMap;

enum PieceLoc {
    Bank,
    First_Player_HW,
    Second_Player_HW,
    Colony(Piece),
}

enum Player {
    FIRST,
    SECOND,
}

struct OwnedPiece {
    piece: Piece,
    owner: Player,
}

struct Colony {
    star: Piece,
    ships: Vec<OwnedPiece>,
}

struct Homeworld {
    // if there are no stars at a homeworld after the first two moves, the game is over, but we need
    // to be able to represent the final game state and the first two game states
    stars: (Option<Piece>, Option<Piece>),
    ships: Vec<OwnedPiece>,
}

pub struct GameState {
    homeworlds: [Homeworld; 2],
    colonies: Vec<Colony>, // TODO change to ArrayVec
    turn: Player,
    // we use this to check whether it's one of the first two moves, but it will also be useful
    // information generally
    move_count: u64,
}

impl GameState {
    fn add_valid_move_moves(&self, moves: &mut Vec<GameState>) {
        // for (ref piece, ref state) in &self.piece_locs {}
        for homeworld in &self.homeworlds {}
    }

    fn add_valid_capture_moves(&self, moves: &mut Vec<GameState>) {
        // for (ref piece, ref state) in &self.piece_locs {}
    }

    fn add_valid_trade_moves(&self, moves: &mut Vec<GameState>) {
        // for (ref piece, ref state) in &self.piece_locs {}
    }

    fn add_valid_build_moves(&self, moves: &mut Vec<GameState>) {
        // for (ref piece, ref state) in &self.piece_locs {}
    }

    pub fn valid_moves(&self) -> Vec<GameState> {
        let mut result = Vec::new();
        self.add_valid_move_moves(&mut result);
        self.add_valid_capture_moves(&mut result);
        self.add_valid_trade_moves(&mut result);
        self.add_valid_build_moves(&mut result);
        result
    }
}
