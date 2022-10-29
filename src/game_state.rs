use super::pieces::Piece;
use enumset::EnumSet;

use crate::pieces::{Color, PieceBank, Size};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PieceLoc {
    Bank,
    FirstPlayerHW,
    SecondPlayerHW,
    Colony(Piece),
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Player {
    First,
    Second,
}

trait HasColors {
    fn colors(&self, player: &Player) -> EnumSet<Color>;
}

trait Location: HasColors {
    fn sizes(&self) -> EnumSet<Size>;

    fn reachable(&self, other: &dyn Location) -> bool {
        self.sizes() & other.sizes() == EnumSet::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OwnedPiece {
    piece: Piece,
    owner: Player,
}

impl OwnedPiece {
    fn first(piece: Piece) -> Self {
        OwnedPiece {
            piece,
            owner: Player::First,
        }
    }

    fn second(piece: Piece) -> Self {
        OwnedPiece {
            piece,
            owner: Player::Second,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Colony {
    star: Piece,
    ships: Vec<OwnedPiece>,
}

impl Location for Colony {
    fn sizes(&self) -> EnumSet<Size> {
        EnumSet::from(*self.star.type_().size())
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Location for Homeworld {
    fn sizes(&self) -> EnumSet<Size> {
        match &self.stars {
            [Some(first), Some(second)] => *first.type_().size() | *second.type_().size(),
            [Some(first), Option::None] => EnumSet::from(*first.type_().size()),
            [Option::None, Some(second)] => EnumSet::from(*second.type_().size()),
            [Option::None, Option::None] => EnumSet::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    bank: PieceBank,
    homeworlds: [Homeworld; 2],
    colonies: Vec<Colony>, // TODO change to ArrayVec
    turn: Player,
    // we use this to check whether it's one of the first two moves, but it will also be useful
    // information generally
    move_count: u64,
}

impl GameState {
    fn add_valid_move_moves(&self, moves: &mut Vec<GameState>) {
        let homeworld_iter = self.homeworlds.iter().map(|h| h as &dyn Location);
        let colony_iter = self.colonies.iter().map(|c| c as &dyn Location);
        let location_iter = homeworld_iter.chain(colony_iter).into_iter();
        let location_iter_2 = location_iter.clone();
        let location_pairs = location_iter.cartesian_product(location_iter_2);
        for (location_a, location_b) in location_pairs {
            if location_a.reachable(location_b)
                && location_a.colors(&self.turn).contains(Color::Yellow)
            {
                todo!()
            }
        }
    }

    fn add_valid_capture_moves(&self, moves: &mut Vec<GameState>) {
        todo!()
    }

    fn add_valid_trade_moves(&self, moves: &mut Vec<GameState>) {
        todo!()
    }

    fn add_valid_build_moves(&self, moves: &mut Vec<GameState>) {
        todo!()
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

mod tests {
    use crate::game_state::{GameState, Homeworld, OwnedPiece, Player};
    use crate::pieces::*;

    fn pop_option(bank: &mut PieceBank, type_: &PieceType) -> Option<Piece> {
        Some(bank.pop_piece(type_).unwrap())
    }

    fn pop(bank: &mut PieceBank, type_: &PieceType) -> Piece {
        bank.pop_piece(type_).unwrap()
    }

    #[test]
    fn do_a_thing() {
        let mut bank = PieceBank::new();

        let hw1_stars = [
            pop_option(&mut bank, &SMALL_RED),
            pop_option(&mut bank, &SMALL_YELLOW),
        ];
        let hw1_ship = OwnedPiece::first(pop(&mut bank, &LARGE_GREEN));
        let hw2_stars = [
            pop_option(&mut bank, &MEDIUM_BLUE),
            pop_option(&mut bank, &LARGE_RED),
        ];
        let hw2_ship = OwnedPiece::second(pop(&mut bank, &LARGE_GREEN));

        let game_state = GameState {
            bank,
            homeworlds: [
                Homeworld {
                    stars: hw1_stars,
                    ships: vec![hw1_ship],
                },
                Homeworld {
                    stars: hw2_stars,
                    ships: vec![hw2_ship],
                },
            ],
            colonies: vec![],
            turn: Player::First,
            move_count: 0,
        };
        let mut expected_move_state = game_state.clone();
        let ship = expected_move_state.homeworlds[0].ships.pop().unwrap();
        expected_move_state.homeworlds[1].ships.push(ship);

        let mut move_states = vec![];
        game_state.add_valid_move_moves(&mut move_states);
        let expected_states = vec![expected_move_state];
        assert_eq!(expected_states, move_states);
    }
}
