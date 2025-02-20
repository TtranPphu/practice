fn main() {
    divan::main();
}

use sudoku::{solve, solve_str};

#[divan::bench]
fn expert() {
    let _ = solve([
        [9, 0, 0, 0, 7, 0, 0, 2, 0],
        [0, 8, 0, 0, 6, 0, 0, 0, 3],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 1],
        [2, 0, 0, 0, 4, 6, 3, 0, 0],
        [6, 0, 0, 0, 9, 0, 0, 0, 8],
        [0, 5, 0, 0, 3, 0, 0, 0, 7],
        [0, 0, 0, 4, 0, 1, 0, 0, 0],
        [0, 9, 0, 7, 0, 0, 1, 0, 0],
    ]);
}

#[divan::bench]
fn expert_str() {
    let _ = solve_str(
        "9...7..2..8..6...3.........4.......12...463..6...9...8.5..3...7...4.1....9.7..1..",
    );
}

#[divan::bench]
fn extreme() {
    let _ = solve([
        [2, 0, 4, 0, 0, 0, 0, 0, 0],
        [1, 0, 7, 0, 9, 0, 0, 0, 2],
        [0, 0, 0, 0, 0, 0, 7, 0, 0],
        [6, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 2, 0, 6, 0, 3],
        [8, 0, 1, 0, 4, 9, 0, 0, 0],
        [4, 0, 0, 0, 6, 0, 0, 7, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 9],
        [0, 0, 0, 3, 0, 0, 5, 6, 0],
    ]);
}

#[divan::bench]
fn extreme_str() {
    let _ = solve_str(
        "2.4......1.7.9...2......7..6....1.......2.6.38.1.49...4...6..7.........9...3..56.",
    );
}

#[divan::bench]
fn free() {
    let _ = solve([
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
}

#[divan::bench]
fn free_str() {
    let _ = solve_str(
        ".................................................................................",
    );
}

#[divan::bench]
fn master() {
    let _ = solve([
        [0, 6, 0, 0, 0, 3, 0, 0, 7],
        [3, 0, 0, 6, 8, 0, 0, 1, 0],
        [1, 9, 0, 2, 0, 0, 0, 0, 0],
        [6, 8, 5, 0, 0, 0, 1, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 1, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 3, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 2, 0, 0, 0, 9],
        [0, 0, 0, 0, 4, 0, 8, 7, 0],
    ]);
}

#[divan::bench]
fn master_str() {
    let _ = solve_str(
        ".6...3..73..68..1.19.2.....685...13..........21.......4.3.....6....2...9....4.87.",
    );
}
