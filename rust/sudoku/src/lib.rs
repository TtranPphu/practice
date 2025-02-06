#![allow(dead_code)]

pub mod grid;

use grid::Grid;

pub fn solve(problem: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, ()> {
    let mut grid = Grid::new();
    grid.init(problem);
    grid.solve()?;
    Ok(grid.state().clone())
}

pub fn demo() {
    let mut grid = Grid::new();
    grid.init(vec![
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
    print!("Solving...\n{}", grid);
    match grid.solve() {
        Ok(_) => println!("Solved!\n{}", grid),
        Err(_) => println!("No solution found!\n{}", grid),
    }
}
