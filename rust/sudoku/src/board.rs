use std::collections::HashSet;

#[derive(Clone)]
pub struct Board {
    cells: Vec<Vec<u8>>,
    candidates: Vec<Vec<HashSet<u8>>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: vec![vec![0; 9]; 9],
            candidates: vec![vec![(1..=9).collect(); 9]; 9],
        }
    }

    pub fn init(&mut self, problem: Vec<Vec<u8>>) -> &mut Self {
        for (i, row) in problem.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value != 0 {
                    self.set_value(i, j, value);
                }
            }
        }
        self
    }

    pub fn from_str(&mut self, problem: &str) -> &mut Self {
        for (i, j, c) in problem
            .chars()
            .enumerate()
            .map(|(ij, c)| (ij / 9, ij % 9, c))
        {
            match c {
                '1'..='9' => self.set_value(i, j, c as u8 - b'0'),
                '.' => (),
                _ => panic!("Invalid character: {}", c),
            }
        }
        self
    }

    pub fn solve(&mut self) -> Result<(), ()> {
        while self.basic_strategies() {}
        if !self.solved() {
            self.brute_force()?;
        }
        Ok(())
    }

    fn solved(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&v| v != 0))
    }

    fn basic_strategies(&mut self) -> bool {
        let mut updated = false;

        for i in 0..9 {
            for j in 0..9 {
                if self.cells[i][j] == 0 {
                    if self.candidates[i][j].len() == 1 {
                        let v = *self.candidates[i][j].iter().next().unwrap();
                        self.set_value(i, j, v);
                        updated = true;
                    } else {
                        for v in self.candidates[i][j].iter() {
                            if self.unique_candidate(i, j, *v) {
                                self.set_value(i, j, *v);
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

    fn unique_candidate(&self, i: usize, j: usize, v: u8) -> bool {
        let (s, p) = Self::convert(i, j);
        for k in 0..9 {
            if k != j && self.candidates[i][k].contains(&v) {
                return false;
            }
            if k != i && self.candidates[k][j].contains(&v) {
                return false;
            }
            let (si, sj) = Self::convert(s, k);
            if k != p && self.candidates[si][sj].contains(&v) {
                return false;
            }
        }
        true
    }

    fn brute_force(&mut self) -> Result<(), ()> {
        if let Some((i, j)) = self.first_empty() {
            for &v in self.candidates[i][j].iter() {
                let mut grid_clone = self.clone();
                grid_clone.set_value(i, j, v);
                if grid_clone.solve().is_ok() {
                    *self = grid_clone;
                    return Ok(());
                }
            }
        }
        Err(())
    }

    fn first_empty(&self) -> Option<(usize, usize)> {
        self.cells.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &v)| if v == 0 { Some((i, j)) } else { None })
        })
    }

    fn set_value(&mut self, i: usize, j: usize, v: u8) {
        self.cells[i][j] = v;
        self.invalidate(i, j, v);
    }

    /// Remove v from the candidates of the cells in the same row, column, and subgrid.
    fn invalidate(&mut self, i: usize, j: usize, v: u8) {
        self.candidates[i][j].clear();
        let (s, _) = Self::convert(i, j);
        for (k, (si, sj)) in (0..9).map(|k| (k, Self::convert(s, k))) {
            self.candidates[i][k].remove(&v);
            self.candidates[k][j].remove(&v);
            self.candidates[si][sj].remove(&v);
        }
    }

    pub fn state(&self) -> &Vec<Vec<u8>> {
        self.cells.as_ref()
    }

    pub fn state_str(&self) -> String {
        self.cells
            .iter()
            .map(|row: &Vec<u8>| row.iter().map(|&v| v.to_string()).collect::<String>())
            .collect::<String>()
    }

    /// Convert (i, j) to (s, p) where:
    /// - s: index of the 3x3 subgrid and
    /// - p: index within the subgrid.
    fn convert(i: usize, j: usize) -> (usize, usize) {
        (i / 3 * 3 + j / 3, i % 3 * 3 + j % 3)
    }
}

impl std::fmt::Display for Board {
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
                match cell {
                    0 => write!(f, " .")?,
                    v => write!(f, " {}", v)?,
                };
            }
            writeln!(f, " ║")?;
        }
        writeln!(f, "╚═══════╧═══════╧═══════╝")
    }
}
