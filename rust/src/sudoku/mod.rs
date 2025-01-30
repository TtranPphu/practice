use macro_utils::comprehension as comp;
use std::collections::HashSet;

struct Cell {
    value: u8,
    candidates: HashSet<u8>,
}

impl Cell {
    fn new(value: u8) -> Cell {
        Cell {
            value,
            candidates: comp!(i for i in 1..10 if i != value).collect(),
        }
    }

    fn invalidate(&mut self, candidate: u8) {
        self.candidates.remove(&candidate);
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

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(grid: Option<Vec<Vec<u8>>>) -> Grid {
        let mut r = Grid {
            cells: vec![vec![Cell::new(0); 9]; 9],
        };
        if let Some(grid) = grid {
            for i in 0..9 {
                for j in 0..9 {
                    r.set_value(i, j, grid[i][j]);
                }
            }
        }
        r
    }

    fn update_validity(&mut self, i: usize, j: usize) {
        let v = self.cells[i][j].value;
        if v == 0 {
            return;
        }
        self.cells[i][j].candidates.clear();
        for k in 0..9 {
            self.cells[i][k].invalidate(v);
            self.cells[k][j].invalidate(v);
            self.cells[i / 3 * 3 + k / 3][j / 3 * 3 + k % 3].invalidate(v);
        }
    }

    fn set_value(&mut self, i: usize, j: usize, value: u8) {
        self.cells[i][j].value = value;
        self.update_validity(i, j);
    }

    fn check_unique_candidate(&self, i: usize, j: usize, value: u8) -> bool {
        for k in 0..9 {
            if k != i && self.cells[k][j].candidates.contains(&value) {
                return false;
            }
            if k != j && self.cells[i][k].candidates.contains(&value) {
                return false;
            }
            if k != i * 3 + j
                && self.cells[i / 3 * 3 + k / 3][j / 3 * 3 + k % 3]
                    .candidates
                    .contains(&value)
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

    fn brute_force(&mut self) -> Result<(), ()> {
        for i in 0..9 {
            for j in 0..9 {
                let cell = &self.cells[i][j];
                if cell.value == 0 {
                    for &v in &cell.candidates {
                        let mut clone = self.clone();
                        clone.set_value(i, j, v);
                        if clone.solve().is_ok() && clone.solved() {
                            *self = clone;
                            return Ok(());
                        }
                    }
                    return Err(());
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
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        Grid::new(Some(
            self.cells
                .iter()
                .map(|row| row.iter().map(|cell| cell.value).collect())
                .collect(),
        ))
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

pub fn demo() {
    let mut grid = Grid::new(Some(vec![
        vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
        vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
        vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
        vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
        vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
        vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
    ]));
    match grid.solve() {
        Ok(_) => println!("Solved!"),
        Err(_) => println!("Unsolvable!"),
    };
    println!("{}", grid);
}
