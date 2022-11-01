use super::pieces::Piece;
use enumset::EnumSet;
use std::collections::HashMap;
use std::iter::Flatten;

use crate::game_state::Action::Move;
use crate::pieces::{Color, PieceBank, PieceType, Size};
use itertools::Itertools;
use std::slice::Iter;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PieceLoc {
    Bank,
    FirstPlayerHW,
    SecondPlayerHW,
    Colony(Piece),
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Player {
    First = 0,
    Second = 1,
}

trait HasColors {
    fn colors(&self, player: &Player) -> EnumSet<Color>;
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
struct Homeworld {
    stars: [Option<Piece>; 2],
    ships: Vec<OwnedPiece>,
    player: Player,
}

impl Homeworld {
    fn remaining_stars(&self) -> Flatten<Iter<Option<Piece>>> {
        self.stars.iter().flatten()
    }

    fn sizes(&self) -> EnumSet<Size> {
        self.remaining_stars()
            .map(|star| star.type_().size())
            .fold(EnumSet::new(), |sizes, size| sizes | *size)
    }

    fn colors(&self) -> EnumSet<Color> {
        self.remaining_stars()
            .map(|star| star.type_().color())
            .fold(EnumSet::new(), |colors, color| colors | *color)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Colony {
    star: Piece,
    ships: Vec<OwnedPiece>,
    id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum System<'a> {
    Homeworld(&'a Homeworld),
    Colony(&'a Colony),
}

#[derive(Debug, PartialEq, Eq)]
enum SystemId {
    Homeworld(Player),
    Colony(u64),
}

#[derive(Debug, PartialEq, Eq)]
enum DestId {
    Sys(SystemId),
    NewColony(PieceType),
}

impl<'a> System<'a> {
    fn sizes(&self) -> EnumSet<Size> {
        match self {
            System::Colony(colony) => EnumSet::from(*colony.star.type_().size()),
            System::Homeworld(homeworld) => homeworld.sizes(),
        }
    }

    fn can_reach(&self, other: &System) -> bool {
        self.sizes() & other.sizes() == EnumSet::empty()
    }

    fn ships(&self) -> &Vec<OwnedPiece> {
        match self {
            System::Homeworld(homeworld) => &homeworld.ships,
            System::Colony(colony) => &colony.ships,
        }
    }

    fn id(&self) -> SystemId {
        match self {
            System::Homeworld(homeworld) => SystemId::Homeworld(homeworld.player),
            System::Colony(colony) => SystemId::Colony(colony.id),
        }
    }
}

impl<'a> HasColors for System<'a> {
    fn colors(&self, player: &Player) -> EnumSet<Color> {
        match self {
            System::Colony(colony) => {
                EnumSet::from(*colony.star.type_().color()) | colony.ships.colors(player)
            }
            System::Homeworld(homeworld) => homeworld.colors() | homeworld.ships.colors(player),
        }
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

const HOMEWORLDS_COUNT: usize = 2;
const MAX_COLONIES_COUNT: usize = 16;
const MAX_SYSTEMS_COUNT: usize = HOMEWORLDS_COUNT + MAX_COLONIES_COUNT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    bank: PieceBank,
    homeworlds: [Homeworld; 2],
    colonies: HashMap<u64, Colony>,
    current_player: Player,
    // we use this to check whether it's one of the first two turns, but it will also be useful
    // information generally
    turn_count: u64,
    colony_id_counter: u64,
    avail_std_actions: (AvailableStdAction, u8),
}

enum SystemsIterLoc<'a> {
    Homeworlds(Player),
    Colonies(std::collections::hash_map::Values<'a, u64, Colony>),
}

struct SystemsIter<'a> {
    loc: SystemsIterLoc<'a>,
    game_state: &'a GameState,
}

impl<'a> SystemsIter<'a> {
    fn new(game_state: &'a GameState) -> Self {
        SystemsIter {
            loc: SystemsIterLoc::Homeworlds(Player::First),
            game_state,
        }
    }
}

impl<'a> Iterator for SystemsIter<'a> {
    type Item = System<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.loc {
            SystemsIterLoc::Homeworlds(player) => {
                let result = Some(System::Homeworld(
                    // TODO figure out if there's a safer way to do this
                    &self.game_state.homeworlds[*player as usize],
                ));
                match player {
                    Player::First => self.loc = SystemsIterLoc::Homeworlds(Player::Second),
                    Player::Second => {
                        self.loc = SystemsIterLoc::Colonies(self.game_state.colonies.values())
                    }
                }
                result
            }
            SystemsIterLoc::Colonies(iter) => {
                if let Some(colony) = iter.next() {
                    Some(System::Colony(colony))
                } else {
                    Option::None
                }
            }
        }
    }
}

impl GameState {
    fn systems(&self) -> SystemsIter {
        SystemsIter::new(self)
    }

    fn add_move_curr_cols(&self, actions: &mut Vec<Action>, src_location: &System) {
        for dest_location in self.systems() {
            if src_location.can_reach(&dest_location) {
                let ships_to_move = owned_by(src_location.ships(), &self.current_player);
                for ship in ships_to_move {
                    let action = MoveAction {
                        src_id: src_location.id(),
                        dest_id: DestId::Sys(dest_location.id()),
                        piece_type: *ship.piece.type_(),
                    };
                    // TODO this will cause some redundancy if two pieces of the same type are in
                    // the same location and owned by the same player.  determine what to do about
                    // this.
                    actions.push(Move(action));
                }
            }
        }
    }

    fn add_move_new_cols(&self, actions: &mut Vec<Action>, src_location: &System) {
        let avail_dest_sizes = Size::all_sizes() ^ src_location.sizes();
        for avail_dest_size in avail_dest_sizes {
            for color in Color::iter() {
                let destination_piece = PieceType::new(avail_dest_size, color);
                if self.bank.contains(destination_piece) {
                    let ships_to_move = owned_by(src_location.ships(), &self.current_player);
                    for ship in ships_to_move {
                        let action = MoveAction {
                            src_id: src_location.id(),
                            dest_id: DestId::NewColony(destination_piece),
                            piece_type: *ship.piece.type_(),
                        };
                        actions.push(Move(action));
                    }
                }
            }
        }
    }

    fn add_move_actions(&self, actions: &mut Vec<Action>) {
        for src_location in self.systems() {
            let can_move = src_location
                .colors(&self.current_player)
                .contains(Color::Yellow);
            if can_move {
                self.add_move_curr_cols(actions, &src_location);
                self.add_move_new_cols(actions, &src_location);
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

    fn homeworld_for(&self, player: &Player) {}

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

trait IsAction {}

#[derive(Debug, PartialEq, Eq)]
struct MoveAction {
    src_id: SystemId,
    dest_id: DestId,
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
struct CatastropheAction {}

impl IsAction for CatastropheAction {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AvailableStdAction {
    Any, // any means any of the below in addition to sacrifice
    Capture,
    Build,
    Trade,
    Move,
}

#[derive(Debug, PartialEq, Eq)]
struct SacrificeAction {
    piece_type: PieceType,
    location: SystemId,
}

impl IsAction for SacrificeAction {}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Move(MoveAction),
    Build(BuildAction),
    Trade(TradeAction),
    Capture(CaptureAction),
    Sacrifice(SacrificeAction),
    Catastrophe(CatastropheAction),
}

mod tests {
    use crate::game_state::*;
    use crate::pieces::*;
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::HashMap;

    fn pop_option(bank: &mut PieceBank, type_: PieceType) -> Option<Piece> {
        Some(bank.pop_piece(type_).unwrap())
    }

    fn pop(bank: &mut PieceBank, type_: PieceType) -> Piece {
        bank.pop_piece(type_).unwrap()
    }

    #[test]
    fn do_a_thing() {
        let mut bank = PieceBank::new();
        let mut systems: [Option<System>; MAX_SYSTEMS_COUNT] = Default::default();

        let hw1_stars = [
            pop_option(&mut bank, SMALL_RED),
            pop_option(&mut bank, SMALL_YELLOW),
        ];
        let hw1_ship = OwnedPiece::first(pop(&mut bank, LARGE_GREEN));
        let hw2_stars = [
            pop_option(&mut bank, MEDIUM_BLUE),
            pop_option(&mut bank, LARGE_RED),
        ];
        let hw2_ship = OwnedPiece::second(pop(&mut bank, LARGE_GREEN));

        let game_state = GameState {
            bank,
            homeworlds: [
                Homeworld {
                    stars: hw1_stars,
                    ships: vec![hw1_ship],
                    player: Player::First,
                },
                Homeworld {
                    stars: hw2_stars,
                    ships: vec![hw2_ship],
                    player: Player::Second,
                },
            ],
            colonies: HashMap::new(),
            current_player: Player::First,
            turn_count: 0,
            colony_id_counter: 0,
            avail_std_actions: (AvailableStdAction::Any, 1),
        };

        let mut move_actions = vec![];
        game_state.add_move_actions(&mut move_actions);
        let expected_actions = vec![
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::Sys(SystemId::Homeworld(Player::Second)),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(MEDIUM_RED),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(MEDIUM_GREEN),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(MEDIUM_BLUE),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(MEDIUM_YELLOW),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(LARGE_RED),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(LARGE_GREEN),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(LARGE_BLUE),
                piece_type: LARGE_GREEN,
            }),
            Move(MoveAction {
                src_id: SystemId::Homeworld(Player::First),
                dest_id: DestId::NewColony(LARGE_YELLOW),
                piece_type: LARGE_GREEN,
            }),
        ];
        assert_eq!(expected_actions, move_actions);
    }
}
