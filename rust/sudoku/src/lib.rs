#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::{Arc, Mutex},
    vec,
};

pub fn solve(problem: Vec<Vec<u8>>) -> Result<Grid, Grid> {
    let mut grid = Grid::new();
    match grid.init(problem).solve() {
        Ok(_) => Ok(grid),
        Err(_) => Err(grid),
    }
}

pub fn demo() {
    let mut board = Grid::new();
    board.init(vec![
        vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
        vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
        vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
        vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
        vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
        vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
    ]);
    println!("Solving...\n{}", board);
    match board.solve() {
        Ok(_) => println!("Solved!\n{}", board),
        Err(_) => println!("Unsolvable!\n{}", board),
    }
}

fn solve_v2(problem: Vec<Vec<u8>>) -> Result<Grid2, Grid2> {
    let mut grid = Grid2::new(problem);
    match grid.solve() {
        Ok(_) => Ok(grid),
        Err(_) => Err(grid),
    }
}

fn demo_v2() {
    let mut board = Grid2::new(vec![
        vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
        vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
        vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
        vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
        vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
        vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
    ]);
    println!("Solving...\n{}", board);
    match board.solve() {
        Ok(_) => println!("Solved!\n{}", board),
        Err(_) => println!("Unsolvable!\n{}", board),
    }
}

fn solve_v3(problem: Vec<Vec<u8>>) -> Result<Grid3, Grid3> {
    let grid = Grid3::new(problem);
    match grid.solve() {
        Ok(_) => Ok(grid),
        Err(_) => Err(grid),
    }
}

fn demo_v3() {
    let board = Grid3::new(vec![
        vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
        vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
        vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
        vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
        vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
        vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
    ]);
    println!("Solving...\n{}", board);
    match board.solve() {
        Ok(_) => println!("Solved!\n{}", board),
        Err(_) => println!("Unsolvable!\n{}", board),
    }
}

pub struct Grid3 {
    cells: Vec<Vec<RefCell<u8>>>,
    candidates: Vec<Vec<RefCell<HashSet<u8>>>>,
}

impl Grid3 {
    fn new(_problem: Vec<Vec<u8>>) -> Grid3 {
        todo!()
    }

    fn state(&self) -> Vec<Vec<u8>> {
        self.cells
            .iter()
            .map(|row| row.iter().map(|cell| *cell.borrow()).collect())
            .collect()
    }

    fn solved(&self) -> bool {
        todo!()
    }

    fn solve(&self) -> Result<(), ()> {
        todo!()
    }

    fn basic_strategy(&self) -> bool {
        todo!()
    }

    fn brute_force(&self) -> Result<(), ()> {
        todo!()
    }

    fn set_value(&self, i: usize, j: usize, v: u8) {
        self.cells[i][j].replace(v);
        self.invalidate(i, j, v);
    }

    fn invalidate(&self, i: usize, j: usize, v: u8) {
        self.candidates[i][j].borrow_mut().clear();
        for k in 0..9 {
            self.candidates[i][k].borrow_mut().remove(&v);
            self.candidates[k][j].borrow_mut().remove(&v);
            let (r, c) = Self::transform(i, j);
            self.candidates[r][c].borrow_mut().remove(&v);
        }
    }

    fn transform(i: usize, j: usize) -> (usize, usize) {
        (i / 3 * 3 + j / 3, i % 3 * 3 + j % 3)
    }
}

impl std::fmt::Display for Grid3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "╔═══════╤═══════╤═══════╗")?;
        for (i, row) in self.cells.iter().enumerate() {
            if i == 3 || i == 6 {
                writeln!(f, "╟───────┼───────┼───────╢")?;
            }
            write!(f, "║")?;
            for (j, cell) in row.iter().enumerate() {
                if j == 3 || j == 6 {
                    write!(f, " │")?;
                }
                match *cell.borrow() {
                    0 => write!(f, " .")?,
                    v => write!(f, " {}", v)?,
                };
            }
            writeln!(f, " ║")?;
        }
        writeln!(f, "╚═══════╧═══════╧═══════╝")
    }
}

pub struct Grid2 {
    /// cells[i][j] is the value of cell (i, j)
    cells: Vec<Vec<Rc<RefCell<u8>>>>,
    /// rotated[j][i] is the value of cell (i, j)
    rotated: Vec<Vec<Rc<RefCell<u8>>>>,
    /// subgrids[s][p] is the value of cell (i, j) where (i, j) is the p-th cell in the s-th subgrid
    subgrids: Vec<Vec<Rc<RefCell<u8>>>>,
}

impl Grid2 {
    fn new(problem: Vec<Vec<u8>>) -> Grid2 {
        let cells: Vec<Vec<Rc<RefCell<u8>>>> = problem
            .into_iter()
            .map(|row| row.into_iter().map(|v| Rc::new(RefCell::new(v))).collect())
            .collect();
        let mut rotated = vec![vec![]; 9];
        for row in cells.iter().by_ref() {
            for (j, v) in row.iter().by_ref().enumerate() {
                rotated[j].push(v.clone());
            }
        }
        let mut subgrids = vec![vec![]; 9];
        for (i, row) in cells.iter().by_ref().enumerate() {
            for (j, v) in row.iter().by_ref().enumerate() {
                subgrids[Self::transform(i, j).0].push(v.clone());
            }
        }
        Grid2 {
            cells,
            rotated,
            subgrids,
        }
    }

    fn solved(&mut self) -> bool {
        self.cells
            .iter()
            .by_ref()
            .all(|row| row.iter().by_ref().all(|cell| *cell.borrow() != 0))
    }

    /// Solve the puzzle by alternating between basic strategy and brute force.
    fn solve(&mut self) -> Result<(), ()> {
        while self.basic_strategy() {}
        if !self.solved() {
            self.brute_force()?;
        }
        Ok(())
    }

    /// Apply basic strategy to solve the puzzle.
    /// The basic strategy consists of two steps:
    /// 1. If a cell has only one candidate, fill it with the candidate.
    /// 2. If a candidate is not candidate for any other cell in the same row, column,
    ///   or subgrid, fill the cell with the candidate.
    fn basic_strategy(&mut self) -> bool {
        let mut updated = false;
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell.borrow() == 0 {
                    let candidates = self.candidates(i, j);
                    if candidates.len() == 1 {
                        *cell.borrow_mut() = candidates[0];
                        updated = true;
                    } else {
                        for &v in &candidates {
                            if self.unique(i, j, v) {
                                *cell.borrow_mut() = v;
                                updated = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        updated
    }

    /// Apply brute force to solve the puzzle.
    fn brute_force(&mut self) -> Result<(), ()> {
        if let Ok((i, j)) = || -> Result<(usize, usize), ()> {
            for (i, row) in self.cells.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if *cell.borrow() == 0 {
                        return Ok((i, j));
                    }
                }
            }
            Err(())
        }() {
            for &v in &self.candidates(i, j) {
                let back_up: Vec<Vec<u8>> = self.state();
                *self.cells[i][j].borrow_mut() = v;
                if self.solve().is_ok() && self.solved() {
                    return Ok(());
                }
                self.restore(back_up);
            }
        }
        Err(())
    }

    pub fn state(&self) -> Vec<Vec<u8>> {
        self.cells
            .iter()
            .map(|row| row.iter().map(|cell| *cell.borrow()).collect())
            .collect()
    }

    fn restore(&mut self, back_up: Vec<Vec<u8>>) {
        for (i, row) in self.cells.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell.borrow_mut() = back_up[i][j];
            }
        }
    }

    /// List of viable candidates for cell (i, j).
    fn candidates(&self, i: usize, j: usize) -> Vec<u8> {
        let mut candidates = vec![];
        let (s, _) = Self::transform(i, j);

        for v in 1..10 {
            let ref_v = Rc::new(RefCell::new(v));
            if !self.cells[i].contains(&ref_v)
                && !self.rotated[j].contains(&ref_v)
                && !self.subgrids[s].contains(&ref_v)
            {
                candidates.push(v);
            }
        }
        candidates
    }

    /// Check if a candidate is not candidate for any other cell in the same row, column,
    /// or subgrid.
    fn unique(&self, i: usize, j: usize, v: u8) -> bool {
        let (s, p) = Self::transform(i, j);
        for k in 0..9 {
            if k != i && self.candidates(k, j).contains(&v) {
                return false;
            }
            if k != j && self.candidates(i, k).contains(&v) {
                return false;
            }
            let (r, c) = Self::transform(s, k);
            if k != p && self.candidates(r, c).contains(&v) {
                return false;
            }
        }
        true
    }

    /// Transform the index of a cell in the grid to the index of the subgrid or vice versa.
    #[inline]
    fn transform(i: usize, j: usize) -> (usize, usize) {
        (i / 3 * 3 + j / 3, i % 3 * 3 + j % 3)
    }
}

impl std::fmt::Display for Grid2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "╔═══════╤═══════╤═══════╗")?;
        for (i, row) in self.cells.iter().enumerate() {
            if i == 3 || i == 6 {
                writeln!(f, "╟───────┼───────┼───────╢")?;
            }
            write!(f, "║")?;
            for (j, cell) in row.iter().enumerate() {
                if j == 3 || j == 6 {
                    write!(f, " │")?;
                }
                match *cell.borrow() {
                    0 => write!(f, " .")?,
                    v => write!(f, " {}", v)?,
                };
            }
            writeln!(f, " ║")?;
        }
        writeln!(f, "╚═══════╧═══════╧═══════╝")
    }
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

struct Cell {
    value: u8,
    candidates: std::collections::HashSet<u8>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: vec![vec![Cell::new(0); 9]; 9],
        }
    }

    fn init(&mut self, grid: Vec<Vec<u8>>) -> &mut Self {
        for i in 0..9 {
            for j in 0..9 {
                if grid[i][j] != 0 {
                    self.set_value(i, j, grid[i][j]);
                }
            }
        }
        self
    }

    fn update_validity(&mut self, i: usize, j: usize) {
        let candidadte = self.cells[i][j].value;
        if candidadte == 0 {
            return;
        }
        self.cells[i][j].candidates.clear();
        for k in 0..9 {
            self.cells[i][k].remove(candidadte);
            self.cells[k][j].remove(candidadte);
            self.cells[i / 3 * 3 + k / 3][j / 3 * 3 + k % 3].remove(candidadte);
        }
    }

    fn set_value(&mut self, i: usize, j: usize, v: u8) {
        self.cells[i][j].value = v;
        self.update_validity(i, j);
    }

    fn check_unique_candidate(&self, i: usize, j: usize, v: u8) -> bool {
        for k in 0..9 {
            if k != i && self.cells[k][j].candidates.contains(&v) {
                return false;
            }
            if k != j && self.cells[i][k].candidates.contains(&v) {
                return false;
            }
            if k != i * 3 + j
                && self.cells[i / 3 * 3 + k / 3][j / 3 * 3 + k % 3]
                    .candidates
                    .contains(&v)
            {
                return false;
            }
        }
        true
    }

    fn basic_strategy(&mut self) -> bool {
        let mut updated = false;
        for i in 0..9 {
            for j in 0..9 {
                let cell = &self.cells[i][j];
                if cell.value == 0 {
                    if cell.candidates.len() == 1 {
                        let value = *cell.candidates.iter().next().unwrap();
                        self.set_value(i, j, value);
                        updated = true;
                    } else {
                        for &v in &cell.candidates {
                            if self.check_unique_candidate(i, j, v) {
                                self.set_value(i, j, v);
                                updated = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        updated
    }

    #[allow(dead_code)]
    fn brute_force(&mut self) -> Result<(), ()> {
        // find the first empty cell
        if let Ok((i, j, cell)) = || -> Result<(usize, usize, &Cell), ()> {
            for i in 0..9 {
                for j in 0..9 {
                    let cell = &self.cells[i][j];
                    if cell.value == 0 {
                        return Ok((i, j, cell));
                    }
                }
            }
            Err(())
        }() {
            // try all posible candidates
            for &v in &cell.candidates {
                let mut clone = self.clone();
                clone.set_value(i, j, v);
                if clone.solve().is_ok() && clone.solved() {
                    *self = clone;
                    return Ok(());
                }
            }
        }
        Err(())
    }

    #[allow(dead_code)]
    fn brute_force_threaded(&mut self) -> Result<(), ()> {
        if let Ok((i, j, cell)) = || -> Result<(usize, usize, &Cell), ()> {
            for i in 0..9 {
                for j in 0..9 {
                    let cell = &self.cells[i][j];
                    if cell.value == 0 {
                        return Ok((i, j, cell));
                    }
                }
            }
            Err(())
        }() {
            let mut threads = vec![];
            for &v in &cell.candidates {
                let mut clone = self.clone();
                let found = Arc::new(Mutex::new(false));
                clone.set_value(i, j, v);
                threads.push(std::thread::spawn(move || {
                    {
                        let found = found.lock().unwrap();
                        if *found {
                            return Err(());
                        }
                    }
                    if clone.solve().is_ok() && clone.solved() {
                        {
                            let mut found = found.lock().unwrap();
                            *found = true;
                        }
                        return Ok(clone);
                    }
                    Err(())
                }));
            }
            for thread in threads {
                if let Ok(Ok(clone)) = thread.join() {
                    *self = clone;
                    return Ok(());
                }
            }
        }
        Err(())
    }

    fn solve(&mut self) -> Result<(), ()> {
        while self.basic_strategy() {}
        if !self.solved() {
            self.brute_force()?;
        }
        Ok(())
    }

    fn solved(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.value != 0))
    }

    #[allow(dead_code)]
    pub fn state(&self) -> Vec<Vec<u8>> {
        self.cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.value).collect())
            .collect()
    }
}

impl Cell {
    fn new(value: u8) -> Cell {
        Cell {
            value,
            candidates: macros::comprehension!(i for i in 1..10 if i != value).collect(),
        }
    }

    fn remove(&mut self, candidate: u8) {
        self.candidates.remove(&candidate);
    }
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        Grid {
            cells: self.cells.iter().map(|row| row.clone()).collect(),
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "╔═══════╤═══════╤═══════╗")?;
        for (i, row) in self.cells.iter().enumerate() {
            if i == 3 || i == 6 {
                writeln!(f, "╟───────┼───────┼───────╢")?;
            }
            write!(f, "║")?;
            for (j, cell) in row.iter().enumerate() {
                if j == 3 || j == 6 {
                    write!(f, " │")?;
                }
                write!(f, " {}", cell)?;
            }
            writeln!(f, " ║")?;
        }
        writeln!(f, "╚═══════╧═══════╧═══════╝")
    }
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        Cell {
            value: self.value,
            candidates: self.candidates.clone(),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.value == 0 {
            write!(f, ".")
        } else {
            write!(f, "{}", self.value)
        }
    }
}
