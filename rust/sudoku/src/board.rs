use core::array::from_fn;
use std::{char, collections::HashSet};

pub enum From {
    Array([[u8; 9]; 9]),
    String(String),
}

#[derive(Clone)]
pub struct Board {
    cells: [[u8; 9]; 9],
    candidates: [[HashSet<u8>; 9]; 9],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[0; 9]; 9],
            candidates: from_fn(|_| from_fn(|_| (1..=9).collect())),
        }
    }
}

impl Board {
    pub fn new(problem: From) -> Result<Self, &'static str> {
        match problem {
            From::Array(a) => Self::from_array(a),
            From::String(s) => Self::from_str(s),
        }
    }

    fn from_array(problem: [[u8; 9]; 9]) -> Result<Self, &'static str> {
        let mut r = Board::default();
        for (i, row) in problem.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                match v {
                    0 => (),
                    1..=9 => r.set_value(i, j, v),
                    _ => return Err("Invalid value"),
                }
            }
        }
        Ok(r)
    }

    fn from_str(problem: String) -> Result<Self, &'static str> {
        let mut r = Board::default();
        for (i, j, c) in problem
            .into_bytes()
            .iter()
            .enumerate()
            .map(|(ij, c)| (ij / 9, ij % 9, c))
        {
            match c {
                b'0'..=b'9' => r.set_value(i, j, c - b'0'),
                b'.' => (),
                _ => return Err("Invalid value"),
            }
        }
        Ok(r)
    }

    pub fn solve(&mut self) -> Result<&[[u8; 9]; 9], (&'static str, &[[u8; 9]; 9])> {
        while self.basic_strategies() {}
        if !self.solved() {
            return self.brute_force();
        } else {
            Ok(&self.cells)
        }
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

    fn brute_force(&mut self) -> Result<&[[u8; 9]; 9], (&'static str, &[[u8; 9]; 9])> {
        if let Some((i, j)) = self.first_empty() {
            for &v in self.candidates[i][j].iter() {
                let mut grid_clone = self.clone();
                grid_clone.set_value(i, j, v);
                if grid_clone.solve().is_ok() {
                    *self = grid_clone;
                    return Ok(&self.cells);
                }
            }
        }
        Err(("No solution found!", &self.cells))
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

    pub fn state_str(&self) -> String {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&v| if v == 0 { '.' } else { char::from(b'0' + v) })
            })
            .flatten()
            .collect()
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
