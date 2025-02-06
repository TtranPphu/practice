#![allow(dead_code)]

pub mod board;

use board::Board;

pub fn solve(problem: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, ()> {
    let mut board = Board::new();
    board.init(problem);
    board.solve()?;
    Ok(board.state().clone())
}

pub fn solve_str(problem: &str) -> Result<String, ()> {
    let mut board = Board::new();
    board.from_str(problem);
    board.solve()?;
    Ok(board.state_str())
}

pub fn demo() {
    let mut board = Board::new();
    board.init(vec![
        vec![0, 6, 0, 0, 0, 3, 0, 0, 7],
        vec![3, 0, 0, 6, 8, 0, 0, 1, 0],
        vec![1, 9, 0, 2, 0, 0, 0, 0, 0],
        vec![6, 8, 5, 0, 0, 0, 1, 3, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 3, 0, 0, 0, 0, 0, 6],
        vec![0, 0, 0, 0, 2, 0, 0, 0, 9],
        vec![0, 0, 0, 0, 4, 0, 8, 7, 0],
    ]);
    print!("Solving...\n{}", board);
    match board.solve() {
        Ok(_) => println!("Solved!\n{}", board),
        Err(_) => println!("No solution found!\n{}", board),
    }
}

pub fn demo_str() {
    let mut board = Board::new();
    board.from_str(
        "........2..8..91..5......4....9.7.....7.3.8.....8.1.3..4..6...5..97..3..2........",
    );
    print!("Solving...\n{}", board);
    match board.solve() {
        Ok(_) => println!("Solved!\n{}", board),
        Err(_) => println!("No solution found!\n{}", board),
    }
}

pub fn gen() {
    let problems =
        vec!["........8..3...4...9..2..6.....79.......612...6.5.2.7...8...5...1.....2.4.5.....3"];
    for problem in problems {
        let mut board = Board::new();
        board.from_str(problem);
        match board.solve() {
            Ok(_) => {
                println!("{}", board.state_str());
            }
            Err(_) => {
                println!("Unsolvable");
            }
        }
    }
}
