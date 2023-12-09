pub const INF: i32 = 1_500_000_000;
use super::eval;
use cozy_chess::{Board, Color, GameStatus, Move, Piece};

/* pub fn alphabeta(board: &Board, depth: u8, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
    if (depth == 0) || (board.status() != GameStatus::Ongoing) {
        return (eval::evaluate(board, depth), None);
    }
    let mut failhard: bool = false;
    let mut best_move = None;
    board.generate_moves(|moves| {
        for mv in moves {
            let mut board = board.clone();
            board.play_unchecked(mv);
            let score= -alphabeta(&board, depth - 1, -beta, -alpha).0;
            if score >= beta {
                failhard = true;
                best_move = Some(mv);
                return true;
            }
            if score > alpha {
                best_move = Some(mv);
                alpha = score;
            }
        }
        false
    });

    if failhard == true {
        return (beta, best_move);
    }

    (alpha, best_move)
}

*/

pub fn alphabeta(board: &Board, depth: u8, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
    if (depth == 0) || (board.status() != GameStatus::Ongoing) {
        return (eval::evaluate(board, depth), None);
    }
    let mut best_move = None;
    let mut best_eval = -INF;
    board.generate_moves(|moves| {
        for mv in moves {
            let mut board = board.clone();
            board.play_unchecked(mv);
            let score = -(alphabeta(&board, depth-1, alpha, beta).0);
            if score > best_eval {
                best_eval = score;
                best_move = Some(mv);
            }
        }
        false
    });

    (best_eval, best_move)
}
