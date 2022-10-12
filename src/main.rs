mod game_state;
mod piece_types;

fn main() {
    let large_gt_small = piece_types::Size::Large > piece_types::Size::Small;
    println!("well? {}", large_gt_small);

    let piece = piece_types::PieceType::Green(piece_types::Size::Large);
    println!("well? {}", piece);
}
