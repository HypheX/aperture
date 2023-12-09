mod eval;
mod search;
mod uci;

use io::Read;
use std::io;
use uci::parse;

use cozy_chess::{Board, Color, GameStatus, Move, Piece};
use search::INF;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/* fn main() {
    let board = Board::startpos();
    for depth in 0..=10 {
        let start = Instant::now();
        let eval = search::alphabeta(&board, depth, -INF, INF);
        let elapsed = start.elapsed().as_secs_f64();
        println!("ply: {depth} | {eval} | t: {elapsed}");
    }
}
*/

pub struct Engine {
    searching: bool,
    root_board: Board,
}
fn main() {
    let mut engine = Engine {
        searching: false,
        root_board: Board::default(),
    };

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        uci::parse(&buffer, &mut engine);
    }
}
