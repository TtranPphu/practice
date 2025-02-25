use std::num::NonZero;

#[derive(Debug, Clone)]
pub struct Board {
    cells: [Cell; 81],
    groups: [Candidates; 27],
    unsets: u8,
}

#[derive(Debug, Clone, Copy)]
struct BoundedCell<const MAX: u8>(Option<NonZero<u8>>);
type Cell = BoundedCell<9>;

#[derive(Debug, Clone, Copy)]
struct BoundedCandidates<const OPTIONS: u8>(u16);
type Candidates = BoundedCandidates<9>;

#[derive(Debug)]
pub enum Error {
    NoSolutionFound,
    MaxValueExceeded { value: u8, max: u8 },
    InvalidSize { size: usize, max: usize },
    InvalidCharacter { value: char },
}

impl Board {
    pub fn new(problem: [u8; 81]) -> Result<Self, Error> {
        let mut board = Self::default();
        problem
            .iter()
            .enumerate()
            .try_for_each(|(index, &value)| board.set_cell(index, value))?;
        Ok(board)
    }

    fn set_cell(&mut self, index: usize, value: u8) -> Result<(), Error> {
        self.cells[index] = Cell::new(value)?;
        if let Some(v) = self.cells[index].0 {
            self.groups[Self::ROWS[index]].remove(v.get());
            self.groups[Self::COLUMNS[index]].remove(v.get());
            self.groups[Self::SUBGRID[index]].remove(v.get());
            if self.unsets > 0 {
                self.unsets -= 1;
            } else {
                println!("{:#}", self);
                panic!("No more unsets");
            }
        }
        Ok(())
    }

    pub fn solve(&mut self) -> bool {
        while self.basic_strategy() {}
        true
    }

    pub fn solved(&self) -> bool {
        self.unsets == 0
    }

    fn basic_strategy(&mut self) -> bool {
        let mut changed = false;
        for i in 0..81 {
            if self.cells[i].0.is_none() {
                if let Some(candidate) = self.candidates(i).single_or_none().0 {
                    let _ = self.set_cell(i, candidate.get());
                    changed = true;
                }
            }
        }
        changed
    }

    fn candidates(&self, index: usize) -> Candidates {
        self.row_candidates(index)
            .inner_join(&self.column_candidates(index))
            .inner_join(&self.subgrid_candidates(index))
    }

    fn row_candidates(&self, index: usize) -> &Candidates {
        &self.groups[Self::ROWS[index]]
    }

    fn column_candidates(&self, index: usize) -> &Candidates {
        &self.groups[9 + Self::COLUMNS[index]]
    }

    fn subgrid_candidates(&self, index: usize) -> &Candidates {
        &self.groups[18 + Self::SUBGRID[index]]
    }

    const ROWS: [usize; 81] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ];

    const COLUMNS: [usize; 81] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2,
        3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5,
        6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    ];

    const SUBGRID: [usize; 81] = [
        0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3,
        4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
        8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8,
    ];
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [Cell::default(); 81],
            groups: [Candidates::default(); 27],
            unsets: 81,
        }
    }
}

impl TryFrom<[u8; 81]> for Board {
    type Error = Error;

    fn try_from(problem: [u8; 81]) -> Result<Self, Self::Error> {
        Self::new(problem)
    }
}

impl TryFrom<[[u8; 9]; 9]> for Board {
    type Error = Error;

    fn try_from(problem: [[u8; 9]; 9]) -> Result<Self, Self::Error> {
        let mut flat = [0; 81];
        problem.iter().enumerate().for_each(|(row, values)| {
            values.iter().enumerate().for_each(|(column, &value)| {
                flat[row * 9 + column] = value;
            });
        });
        Self::new(flat)
    }
}

impl TryFrom<String> for Board {
    type Error = Error;

    fn try_from(problem: String) -> Result<Self, Self::Error> {
        if problem.len() != 81 {
            return Err(Error::InvalidSize {
                size: problem.len(),
                max: 81,
            });
        }
        let mut flat = [0; 81];
        problem.chars().enumerate().try_for_each(|(index, value)| {
            let value = value
                .to_digit(10)
                .ok_or_else(|| Error::InvalidCharacter { value })? as u8;
            flat[index] = value;
            Ok(())
        })?;
        Self::new(flat)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line = |i: usize| {
            if let Ok(row) = self.cells[i * 9..i * 9 + 9].try_into() {
                Ok(Row::new(row))
            } else {
                Err(std::fmt::Error)
            }
        };
        writeln!(f, "╔═══════╤═══════╤═══════╗")?;
        for i in 0..9 {
            writeln!(f, "{}", line(i)?)?;
            if i == 2 || i == 5 {
                writeln!(f, "╟───────┼───────┼───────╢")?;
            }
        }
        writeln!(f, "╚═══════╧═══════╧═══════╝")
    }
}

pub struct Row([Cell; 9]);

impl Row {
    fn new(cells: [Cell; 9]) -> Self {
        Self(cells)
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cell = |i: usize| self.0[i];
        write!(f, "║")?;
        for i in 0..9 {
            write!(f, " {}", cell(i))?;
            if i == 2 || i == 5 {
                write!(f, " │")?;
            }
        }
        write!(f, " ║")
    }
}

impl<const MAX: u8> BoundedCell<MAX> {
    const EMPTY: Self = Self(None);

    fn new(value: u8) -> Result<Self, Error> {
        if value > MAX {
            return Err(Error::MaxValueExceeded { value, max: MAX });
        }
        Ok(Self(NonZero::new(value)))
    }
}

impl<const MAX: u8> Default for BoundedCell<MAX> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::fmt::Display for BoundedCell<9> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "."),
        }
    }
}

impl<const OPTIONS: u8> BoundedCandidates<OPTIONS> {
    const ALL: Self = Self((1 << OPTIONS) - 1);

    fn remove(&mut self, value: u8) {
        self.0 &= !(1 << (value - 1));
    }

    fn single_or_none(&self) -> BoundedCell<OPTIONS> {
        if self.0 & (self.0 - 1) == 0 {
            BoundedCell::new(self.0.trailing_zeros() as u8 + 1).unwrap()
        } else {
            BoundedCell::default()
        }
    }

    fn inner_join(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl<const OPTIONS: u8> Default for BoundedCandidates<OPTIONS> {
    fn default() -> Self {
        Self::ALL
    }
}
