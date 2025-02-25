#![allow(dead_code)]

pub mod board;
pub mod board_v3;

pub fn solve<T>(problem: T) -> Result<T, (&'static str, T)>
where
    T: Clone,
    board::Board: TryFrom<T, Error = &'static str> + TryInto<T, Error = &'static str>,
{
    match board::Board::try_from(problem.clone()) {
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

pub fn demo_v3() {
    if let Ok(mut board) = board_v3::Board::try_from([
        [0, 6, 0, 0, 0, 3, 0, 0, 7],
        [3, 0, 0, 6, 8, 0, 0, 1, 0],
        [1, 9, 0, 2, 0, 0, 0, 0, 0],
        [6, 8, 5, 0, 0, 0, 1, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 1, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 3, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 2, 0, 0, 0, 9],
        [0, 0, 0, 0, 4, 0, 8, 7, 0],
    ]) {
        println!("{:#}", board);
        board.solve();
        println!("{:#}", board);
    }
}
