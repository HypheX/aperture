use cozy_chess::{Board, Move};
use crate::eval::eval;

pub struct Response {
    pub evaluation: Option<i32>,
    pub mv: Option<Move>
}

pub fn search(board: &Board, depth: i32) -> i32 {
    match depth {
        0 => {
            return eval(&board);
        },
        _ => {
            let mut max: i32 = i32::MIN + 1;
            board.generate_moves(|moves| { 
                for mv in moves {
                    let mut board = board.clone();
                    board.play_unchecked(mv);
                    let evaluation = search(&board, depth-1);
                    if evaluation > max {
                        max = evaluation;
                    }
                } 
                false
            });

            return -max;
        }
    }
}

pub fn init_search(board: &Board, depth: i32) -> Response {
    match depth {
        0 => {panic!("You cannot start a search with 0 depth yet!");}
        _ => {
            let mut max: i32 = i32::MIN;
            let mut res: Response = Response {mv: None, evaluation: None};
            board.generate_moves(|moves| { 
                for mv in moves {
                    let mut board = board.clone();
                    board.play_unchecked(mv);
                    let evaluation = search(&board, depth-1);
                    if evaluation > max {
                        max = evaluation;
                        res = Response{mv: Some(mv), evaluation: Some(evaluation)}
                    }
                } 
                false
            });

            return res
        }
    }
}
pub fn perft(board: &Board, depth: i32) -> u64 { 
    if depth == 0 {
        1
    } else {
        let mut nodes = 0;
        board.generate_moves(|moves| { 
            for mv in moves {
                let mut board = board.clone();
                board.play_unchecked(mv);
                nodes += perft(&board, depth-1)
            }
            false
        });
        nodes
    }
}