use cozy_chess::{Board};
use std::time::{Duration, Instant};
mod gui;
mod eval;
mod search;


fn main() {
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/8/1R5K w - - 0 1", false).unwrap();
    let res = search::init_search(&board, 6);
    println!("{0:?} | {1:?}", res.mv, res.evaluation);
}
