use macro_utils::comprehension as comp;
use std::collections::HashSet;

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

struct Cell {
    value: u8,
    candidates: HashSet<u8>,
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

    #[allow(dead_code)]
    fn result(&self) -> Vec<Vec<u8>> {
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
            candidates: comp!(i for i in 1..10 if i != value).collect(),
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

pub fn solve(problem: Vec<Vec<u8>>) -> Result<Grid, Grid> {
    let mut grid = Grid::new();
    match grid.init(problem).solve() {
        Ok(_) => Ok(grid),
        Err(_) => Err(grid),
    }
}

#[allow(dead_code)]
pub fn demo() {
    match solve(vec![
        vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
        vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
        vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
        vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
        vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
        vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
    ]) {
        Ok(grid) => println!("Solved!\n{}", grid),
        Err(grid) => println!("Unsolvable!\n{}", grid),
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::collections::HashMap;

    #[test]
    fn test_sudoku() {
        let mut expert: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
        expert.insert(
            String::from("problem"),
            vec![
                vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
                vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
                vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
                vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
                vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
                vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
            ],
        );
        expert.insert(
            String::from("expected"),
            vec![
                vec![9, 4, 1, 8, 7, 3, 6, 2, 5],
                vec![5, 8, 7, 2, 6, 4, 9, 1, 3],
                vec![3, 2, 6, 9, 1, 5, 8, 7, 4],
                vec![4, 3, 9, 5, 2, 8, 7, 6, 1],
                vec![2, 7, 8, 1, 4, 6, 3, 5, 9],
                vec![6, 1, 5, 3, 9, 7, 2, 4, 8],
                vec![1, 5, 2, 6, 3, 9, 4, 8, 7],
                vec![7, 6, 3, 4, 8, 1, 5, 9, 2],
                vec![8, 9, 4, 7, 5, 2, 1, 3, 6],
            ],
        );
        let mut extreme: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
        extreme.insert(
            String::from("problem"),
            vec![
                vec![2, 0, 4, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 7, 0, 9, 0, 0, 0, 2],
                vec![0, 0, 0, 0, 0, 0, 7, 0, 0],
                vec![6, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 6, 0, 3],
                vec![8, 0, 1, 0, 4, 9, 0, 0, 0],
                vec![4, 0, 0, 0, 6, 0, 0, 7, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 9],
                vec![0, 0, 0, 3, 0, 0, 5, 6, 0],
            ],
        );
        extreme.insert(
            String::from("expected"),
            vec![
                vec![2, 8, 4, 7, 5, 3, 1, 9, 6],
                vec![1, 5, 7, 4, 9, 6, 8, 3, 2],
                vec![3, 9, 6, 2, 1, 8, 7, 4, 5],
                vec![6, 7, 2, 5, 3, 1, 9, 8, 4],
                vec![5, 4, 9, 8, 2, 7, 6, 1, 3],
                vec![8, 3, 1, 6, 4, 9, 2, 5, 7],
                vec![4, 1, 5, 9, 6, 2, 3, 7, 8],
                vec![7, 6, 3, 1, 8, 5, 4, 2, 9],
                vec![9, 2, 8, 3, 7, 4, 5, 6, 1],
            ],
        );
        let tests = vec![expert, extreme];
        for test in tests {
            let problem = test.get("problem").unwrap();
            let expected = test.get("expected").unwrap();
            match solve(problem.clone()) {
                Ok(grid) => assert_eq!(grid.result(), *expected),
                Err(_) => panic!("Unsolvable!"),
            }
        }
    }
}
