#![allow(dead_code)]

pub mod board;

pub fn solve(problem: [[u8; 9]; 9]) -> Result<[[u8; 9]; 9], (&'static str, [[u8; 9]; 9])> {
    use board::Board;
    match Board::try_from(problem) {
        Ok(mut board) => {
            if board.solve() {
                Ok(board.try_into().unwrap())
            } else {
                Err(("No solution found!", board.try_into().unwrap()))
            }
        }
        Err(err) => Err((err, problem)),
    }
}

pub fn solve_str(problem: &str) -> Result<String, (&'static str, String)> {
    use board::Board;
    match Board::try_from(problem) {
        Ok(mut board) => {
            if board.solve() {
                Ok(board.try_into().unwrap())
            } else {
                Err(("No solution found!", board.try_into().unwrap()))
            }
        }
        Err(err) => Err((err, problem.to_string())),
    }
}

pub fn demo() {
    let mut board = board::Board::try_from([
        [0, 6, 0, 0, 0, 3, 0, 0, 7],
        [3, 0, 0, 6, 8, 0, 0, 1, 0],
        [1, 9, 0, 2, 0, 0, 0, 0, 0],
        [6, 8, 5, 0, 0, 0, 1, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 1, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 3, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 2, 0, 0, 0, 9],
        [0, 0, 0, 0, 4, 0, 8, 7, 0],
    ])
    .unwrap();
    println!("{:#}", board);
    board.solve();
    println!("{:#}", board);
}
