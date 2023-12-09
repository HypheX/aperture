use macroquad::material;

use crate::{Board, GameStatus, Piece};

const MATERIAL_ADVANTAGE_MUL: i32 = 1;
pub const WIN_VALUE: i32 = 1_000_000_000;
const ITER_PIECE_TYPES: [Piece; 5] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
];
fn get_value(piece: &Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 280,
        Piece::Bishop => 320,
        Piece::Rook => 500,
        Piece::Queen => 1000,
        Piece::King => 0,
    }
}

pub fn evaluate(board: &Board, depth: u8) -> i32 {
    if board.status() == GameStatus::Won {
        return WIN_VALUE;
    }

    if board.status() == GameStatus::Drawn {
        return 0;
    }

    let mut eval = 0;

    let mut material_advantage = 0;
    for piece_type in ITER_PIECE_TYPES {
        material_advantage += board.colored_pieces(board.side_to_move(), piece_type).len() as i32
            * get_value(&piece_type);
        material_advantage -= board
            .colored_pieces(!board.side_to_move(), piece_type)
            .len() as i32
            * get_value(&piece_type);
    }

    if board
        .colored_pieces(board.side_to_move(), Piece::Bishop)
        .len()
        >= 2
    {
        material_advantage += get_value(&Piece::Pawn);
    }
    if board
        .colored_pieces(!board.side_to_move(), Piece::Bishop)
        .len()
        >= 2
    {
        material_advantage -= get_value(&Piece::Pawn);
    }

    eval += material_advantage * MATERIAL_ADVANTAGE_MUL;

    eval
}
