mod game_state;
mod pieces;

fn main() {
    let large_gt_small = pieces::Size::Large > pieces::Size::Small;
    println!("well? {}", large_gt_small);

    // let piece = piece_types::PieceType::Green(piece_types::Size::Large);
    let piece_type = pieces::PieceType::new(pieces::Color::Blue, pieces::Size::Small);
    let piece = pieces::Piece::new(piece_type, 0);

    println!("well? {:?}", piece);
}
