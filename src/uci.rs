use std::thread;
use std::str::SplitWhitespace;

use cozy_chess::{Board, Move, File, Square, Piece};
use macroquad::input;

use super::{Engine, INF};
use super::search;

pub fn parse(input: &str, engine: &mut Engine) {
    let mut input_iter = input.split_whitespace();
    loop {
        let next = input_iter.next();
        if let None = next {
            break;
        };

        let str = next.unwrap();
        match str {
            "uci" => {
                uci();
                break;
            }
            "ucinewgame" => break,
            "position" => {
                position(input_iter, engine);
                break;
            }
            "isready" => {
                isready();
                break;
            }
            "go" => {
                go(input_iter, engine);
                break;
            },
            "stop" => {
                stop(input_iter, engine);
                break;
            }
            _ => {}
        }
    }
}

fn go(mut input_iter: SplitWhitespace<'_>, engine: &mut Engine) {
    engine.searching = true;

    let board = engine.root_board.clone();
    thread::spawn(|| {
        let thread_board = board;
        let (eval, best) = search::alphabeta(&thread_board, 4, -INF, INF);
        println!("info score cp {}", eval);
        let mv = best.unwrap();
        let uci_move = to_uci_castling(&thread_board, mv, false);
        println!("bestmove {}", uci_move)
    });
}

fn stop(mut input_iter: SplitWhitespace<'_>, engine: &mut Engine) {
    engine.searching = false;
}
fn position(mut input_iter: SplitWhitespace<'_>, engine: &mut Engine) {
    let pos_type = input_iter
        .next()
        .expect("position: missing argument, expecting [ fen fenstring | startpos ] ");
    match pos_type {
        "fen" => {
            let mut fen_vec = Vec::new();
            for _ in 0..6 {
                fen_vec.push(input_iter.next().expect("Invalid FEN"));
            }
            engine.root_board = Board::from_fen(&fen_vec.join(" "), false).expect("Invalid FEN");
        }
        "startpos" => {
            engine.root_board = Board::startpos();
        }
        _ => panic!(
            "position: invalid argument, expecting [ fen fenstring | startpos ], got {} ",
            pos_type
        ),
    }

    if let Some("moves") = input_iter.next() {
        for mv in input_iter {
            let cozy_move = from_uci_castling(&engine.root_board, mv.parse().unwrap(), false);
            engine.root_board.play(cozy_move);
        }
    }
}

fn uci() {
    println!("id name Aperture {}", crate::VERSION);
    println!("id author xelph");
    println!("uciok");
}

fn isready() {
    println!("readyok");
}


fn to_uci_castling(board: &Board, mut mv: Move, chess960: bool) -> Move {
    if chess960 {
        return mv;
    }
    if board.color_on(mv.from) == board.color_on(mv.to) {
        if mv.to.file() > mv.from.file() {
            mv.to = Square::new(File::G, mv.to.rank());
        } else {
            mv.to = Square::new(File::C, mv.to.rank());
        }
    }
    mv
}

fn from_uci_castling(board: &Board, mut mv: Move, chess960: bool) -> Move {
    if chess960 {
        return mv;
    }
    if mv.from.file() == File::E && board.piece_on(mv.from) == Some(Piece::King) {
        if mv.to.file() == File::G {
            mv.to = Square::new(File::H, mv.to.rank());
        } else if mv.to.file() == File::C {
            mv.to = Square::new(File::A, mv.to.rank());
        }
    }
    mv
}