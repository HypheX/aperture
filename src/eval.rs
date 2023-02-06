use cozy_chess::{Board, GameStatus};

const DRAW: i32 = 0;
const WIN: i32 = 1_000_000;
const LOSS: i32 = -1_000_000;

const PAWN: i32 = 100;
const KNIGHT: i32 = 300;
const BISHOP: i32 = 350;
const ROOK: i32 = 500;
const QUEEN: i32 = 900;

pub fn eval(board: &Board) -> i32 {
    

    let evaluation: i32 = match board.status() {
        GameStatus::Won => WIN - (board.halfmove_clock() as i32),
        GameStatus::Drawn => DRAW,
        GameStatus::Ongoing => {1},
    };

    if evaluation > 1 {
        println!("We detected a checkmate");
    }
    evaluation
}