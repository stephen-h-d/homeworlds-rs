mod game_state;
mod piece_types;

fn main() {
    let large_gt_small = piece_types::Size::Large > piece_types::Size::Small;
    println!("well? {}", large_gt_small);

    // let piece = piece_types::PieceType::Green(piece_types::Size::Large);
    let piece_type =
        piece_types::PieceType::new(piece_types::Color::Blue, piece_types::Size::Small);
    let piece = piece_types::Piece::new(piece_type, 0);

    println!("well? {:?}", piece);
}
