use super::piece_types::Piece;
use enumset::EnumSet;

use crate::piece_types::Color;
use std::collections::HashMap;

enum PieceLoc {
    Bank,
    First_Player_HW,
    Second_Player_HW,
    Colony(Piece),
}

#[derive(Eq, PartialEq)]
enum Player {
    First,
    Second,
}

trait HasColors {
    fn colors(&self, player: &Player) -> EnumSet<Color>;
}

struct OwnedPiece {
    piece: Piece,
    owner: Player,
}

struct Colony {
    star: Piece,
    ships: Vec<OwnedPiece>,
}

impl HasColors for Vec<OwnedPiece> {
    fn colors(&self, player: &Player) -> EnumSet<Color> {
        self.iter()
            .filter(|piece| piece.owner == *player)
            .fold(EnumSet::new(), |colors, piece| {
                colors | *piece.piece.type_().color()
            })
    }
}

impl HasColors for Colony {
    fn colors(&self, player: &Player) -> EnumSet<Color> {
        *self.star.type_().color() | self.ships.colors(player)
    }
}

struct Homeworld {
    // if there are no stars at a homeworld after the first two moves, the game is over, but we need
    // to be able to represent the final game state and the first two game states
    stars: [Option<Piece>; 2],
    ships: Vec<OwnedPiece>,
}

impl HasColors for Homeworld {
    fn colors(&self, player: &Player) -> EnumSet<Color> {
        self.stars
            .iter()
            .flatten()
            .fold(self.ships.colors(player), |colors, piece| {
                colors | *piece.type_().color()
            })
    }
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

        for colony in &self.colonies {}
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
