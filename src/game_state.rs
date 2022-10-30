use super::pieces::Piece;
use enumset::EnumSet;
use std::collections::HashMap;

use crate::game_state::Action::Move;
use crate::pieces::{Color, PieceBank, PieceType, Size};
use itertools::Itertools;
use std::slice::Iter;

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

    fn ships(&self) -> &Vec<OwnedPiece>;

    fn id(&self) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OwnedPiece {
    piece: Piece,
    owner: Player,
}

fn owned_by<'a>(
    ships: &'a Vec<OwnedPiece>,
    player: &'a Player,
) -> impl Iterator<Item = &'a OwnedPiece> + 'a {
    ships.iter().filter(|ship| ship.owner == *player)
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
    id: u64,
}

impl Location for Colony {
    fn sizes(&self) -> EnumSet<Size> {
        EnumSet::from(*self.star.type_().size())
    }

    fn ships(&self) -> &Vec<OwnedPiece> {
        &self.ships
    }

    fn id(&self) -> u64 {
        self.id
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
    id: u64,
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

    fn ships(&self) -> &Vec<OwnedPiece> {
        &self.ships
    }

    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    bank: PieceBank,
    homeworlds: [Homeworld; 2],
    colonies: HashMap<u64, Colony>, // TODO change to array of Options
    current_player: Player,
    // we use this to check whether it's one of the first two turns, but it will also be useful
    // information generally
    turn_count: u64,
    colony_id_counter: u64,
}

trait IsAction {}

#[derive(Debug, PartialEq, Eq)]
struct MoveAction {
    src_id: u64,
    dest_id: u64,
    piece_type: PieceType,
}

impl IsAction for MoveAction {}

#[derive(Debug, PartialEq, Eq)]
struct BuildAction {}

impl IsAction for BuildAction {}

#[derive(Debug, PartialEq, Eq)]
struct TradeAction {}

impl IsAction for TradeAction {}

#[derive(Debug, PartialEq, Eq)]
struct CaptureAction {}

impl IsAction for CaptureAction {}

#[derive(Debug, PartialEq, Eq)]
enum SacrificeAction {
    SmallRed(Piece, Option<CaptureAction>),
    MediumRed(Piece, [Option<CaptureAction>; 2]),
    LargeRed(Piece, [Option<CaptureAction>; 3]),
    SmallBlue(Piece, Option<TradeAction>),
    MediumBlue(Piece, [Option<TradeAction>; 2]),
    LargeBlue(Piece, [Option<TradeAction>; 3]),
    SmallGreen(Piece, Option<BuildAction>),
    MediumGreen(Piece, [Option<BuildAction>; 2]),
    LargeGreen(Piece, [Option<BuildAction>; 3]),
    SmallYellow(Piece, Option<MoveAction>),
    MediumYellow(Piece, [Option<MoveAction>; 2]),
    LargeYellow(Piece, [Option<MoveAction>; 3]),
}

impl IsAction for SacrificeAction {}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Move(MoveAction),
    Build(BuildAction),
    Trade(TradeAction),
    Capture(CaptureAction),
    Sacrifice(SacrificeAction),
}

impl GameState {
    fn add_move_actions(&self, actions: &mut Vec<Action>) {
        let homeworld_iter = self.homeworlds.iter().map(|h| h as &dyn Location);
        let colony_iter = self.colonies.values().map(|c| c as &dyn Location);
        let location_iter = homeworld_iter.chain(colony_iter).into_iter();
        let location_iter_2 = location_iter.clone();
        let location_pairs = location_iter.cartesian_product(location_iter_2);
        for (src_location, dest_location) in location_pairs {
            let can_move = src_location
                .colors(&self.current_player)
                .contains(Color::Yellow);
            if can_move && src_location.reachable(dest_location) {
                let ships_to_move = owned_by(src_location.ships(), &self.current_player);
                for ship in ships_to_move {
                    let action = MoveAction {
                        src_id: src_location.id(),
                        dest_id: dest_location.id(),
                        piece_type: *ship.piece.type_(),
                    };
                    // TODO this will cause some redundancy if two pieces of the same type are in
                    // the same location and owned by the same player.  determine what to do about
                    // this.
                    actions.push(Action::Move(action));
                }
            }
        }
    }

    fn add_capture_actions(&self, actions: &mut Vec<Action>) {
        todo!()
    }

    fn add_trade_actions(&self, actions: &mut Vec<Action>) {
        todo!()
    }

    fn add_build_actions(&self, actions: &mut Vec<Action>) {
        todo!()
    }

    pub fn valid_moves(&self) -> Vec<GameState> {
        let mut result = Vec::new();
        self.add_move_actions(&mut result);
        self.add_capture_actions(&mut result);
        self.add_trade_actions(&mut result);
        self.add_build_actions(&mut result);
        let mut result = vec![];
        result
    }
}

mod tests {
    use crate::game_state::{Action, GameState, Homeworld, MoveAction, OwnedPiece, Player};
    use crate::pieces::*;
    use std::collections::HashMap;

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
                    id: 0,
                },
                Homeworld {
                    stars: hw2_stars,
                    ships: vec![hw2_ship],
                    id: 1,
                },
            ],
            colonies: HashMap::new(),
            current_player: Player::First,
            turn_count: 2,
            colony_id_counter: 2,
        };
        let mut expected_move_state = game_state.clone();
        let ship = expected_move_state.homeworlds[0].ships.pop().unwrap();
        expected_move_state.homeworlds[1].ships.push(ship);

        let mut move_actions = vec![];
        game_state.add_move_actions(&mut move_actions);
        let expected_actions = vec![Action::Move(MoveAction {
            src_id: 0,
            dest_id: 1,
            piece_type: LARGE_GREEN,
        })];
        assert_eq!(expected_actions, move_actions);
    }
}
