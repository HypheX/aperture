use std::thread;
use std::str::SplitWhitespace;

use cozy_chess::{Board, Move};
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
    let search_handle = thread::spawn(|| {
        let thread_board = board;
        let (_, best) = search::alphabeta(&thread_board, 50, -INF, INF);
        println!("bestmove {}", best.unwrap())
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
            engine.root_board.play(mv.parse().unwrap());
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
