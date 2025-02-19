#![allow(dead_code)]

pub mod board;
pub mod board_v2;

use board::{
    Board,
    From::{Array as BfA, String as BfS},
};

pub fn solve(problem: [[u8; 9]; 9]) -> Result<[[u8; 9]; 9], (&'static str, [[u8; 9]; 9])> {
    match Board::new(BfA(problem)) {
        Ok(mut board) => match board.solve() {
            Ok(state) => Ok(*state),
            Err((err, state)) => Err((err, *state)),
        },
        Err(err) => Err((err, problem)),
    }
}

pub fn solve_str(problem: &str) -> Result<String, (&'static str, String)> {
    match Board::new(BfS(String::from(problem))) {
        Ok(mut board) => match board.solve() {
            Ok(_) => Ok(board.state_str()),
            Err((err, _)) => Err((err, board.state_str())),
        },
        Err(err) => Err((err, problem.to_string())),
    }
}

pub fn demo() {
    if let Ok(mut board) = Board::new(BfA([
        [0, 6, 0, 0, 0, 3, 0, 0, 7],
        [3, 0, 0, 6, 8, 0, 0, 1, 0],
        [1, 9, 0, 2, 0, 0, 0, 0, 0],
        [6, 8, 5, 0, 0, 0, 1, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 1, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 3, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 2, 0, 0, 0, 9],
        [0, 0, 0, 0, 4, 0, 8, 7, 0],
    ])) {
        print!("Solving...\n{}", board);
        match board.solve() {
            Ok(_) => println!("Solved!\n{}", board),
            Err(_) => println!("No solution found!\n{}", board),
        }
    } else {
        println!("Invalid board!");
    }
}

pub fn demo_str() {
    if let Ok(mut board) = Board::new(BfS(String::from(
        "........2..8..91..5......4....9.7.....7.3.8.....8.1.3..4..6...5..97..3..2........",
    ))) {
        print!("Solving...\n{}", board);
        match board.solve() {
            Ok(_) => println!("Solved!\n{}", board),
            Err(_) => println!("No solution found!\n{}", board),
        }
    } else {
        println!("Invalid board!");
    }
}
