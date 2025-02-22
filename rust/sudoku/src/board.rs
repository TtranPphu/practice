use std::num::NonZero;

#[derive(Clone, Debug)]
pub struct Board {
    cells: [Cell; 81],
    candidateses: [Candidates; 81],
    unsets: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Candidates(u16);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cell(Option<NonZero<u8>>);

impl Board {
    pub fn new(problem: [u8; 81]) -> Self {
        let mut board = Self::default();
        for (i, &v) in problem.iter().enumerate() {
            if let Some(value) = NonZero::new(v) {
                board.set(i, value);
            }
        }
        board
    }

    pub fn solve(&mut self) -> bool {
        while self.basic_strategies() {}
        if !self.solved() {
            if let Some(board) = self.brute_force() {
                *self = board;
            }
        }
        self.solved()
    }

    pub fn solved(&self) -> bool {
        self.unsets == 0
    }

    fn brute_force(&self) -> Option<Self> {
        if let Some((index, candidate)) = self
            .candidateses
            .iter()
            .enumerate()
            .filter(|(_, &c)| c.count() > 1)
            .min_by_key(|&(_, c)| c.count())
        {
            for value in candidate.iter() {
                let mut board = self.clone();
                board.set(index, value);
                if board.solve() {
                    return Some(board);
                }
            }
            None
        } else {
            None
        }
    }

    fn basic_strategies(&mut self) -> bool {
        let mut changed = false;
        for i in 0..81 {
            if let Some(value) = self.naked_single(i) {
                self.set(i, value);
                changed = true;
            } else if let Some(value) = self.hidden_single(i) {
                self.set(i, value);
                changed = true;
            }
        }
        changed
    }

    fn naked_single(&self, index: usize) -> Option<NonZero<u8>> {
        self.candidateses[index].single_or_none()
    }

    fn hidden_single(&self, index: usize) -> Option<NonZero<u8>> {
        Self::neighbors(index)
            .into_iter()
            .filter(|&n| n != index)
            .filter(|&n| self.cells[n].value().is_none())
            .fold(self.candidateses[index], |c, n| {
                c.exclude(&self.candidateses[n])
            })
            .single_or_none()
    }

    fn set(&mut self, index: usize, value: NonZero<u8>) {
        let v = value.get();
        self.unsets -= 1;
        self.cells[index] = Cell(Some(value));
        self.candidateses[index] = Candidates::NONE;
        for &neighbor in &Self::neighbors(index) {
            self.candidateses[neighbor].remove(&v);
        }
    }

    fn neighbors(index: usize) -> [usize; 27] {
        let (i, j) = (index / 9, index % 9);
        let (s, p) = (i / 3 * 3, j / 3 * 3);
        let mut neighbors = [0; 27];
        for k in 0..9 {
            neighbors[k] = i * 9 + k;
            neighbors[9 + k] = k * 9 + j;
            let (si, sj) = (s + k / 3, p + k % 3);
            neighbors[18 + k] = si * 9 + sj;
        }
        neighbors
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [Cell::EMPTY; 81],
            candidateses: [Candidates::ALL; 81],
            unsets: 81,
        }
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
        let cell = |i: usize| self.0[i].value().map_or('.', |v| (v + b'0') as char);
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

impl TryFrom<[u8; 81]> for Board {
    type Error = &'static str;

    fn try_from(value: [u8; 81]) -> Result<Self, Self::Error> {
        Ok(Self::new(value))
    }
}

impl TryInto<[[u8; 9]; 9]> for Board {
    type Error = &'static str;

    fn try_into(self) -> Result<[[u8; 9]; 9], Self::Error> {
        let mut problem = [[0; 9]; 9];
        for (i, row) in problem.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if let Some(value) = self.cells[i * 9 + j].value() {
                    *cell = value;
                }
            }
        }
        Ok(problem)
    }
}

impl TryFrom<[[u8; 9]; 9]> for Board {
    type Error = &'static str;

    fn try_from(value: [[u8; 9]; 9]) -> Result<Self, Self::Error> {
        let mut problem = [0; 81];
        for (i, row) in value.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                problem[i * 9 + j] = v;
            }
        }
        Ok(Self::new(problem))
    }
}

impl TryInto<String> for Board {
    type Error = &'static str;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self
            .cells
            .iter()
            .map(|c| match c.value() {
                Some(v) => (v + b'0') as char,
                None => '.',
            })
            .collect())
    }
}

impl TryFrom<String> for Board {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 81 {
            let mut problem = [0; 81];
            for (i, c) in value.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    problem[i] = d as u8;
                } else if c != '.' {
                    return Err("Invalid character");
                }
            }
            Ok(Self::new(problem))
        } else {
            Err("Invalid length")
        }
    }
}

impl TryFrom<&str> for Board {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 81 {
            let mut problem = [0; 81];
            for (i, c) in value.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    problem[i] = d as u8;
                } else if c != '.' {
                    return Err("Invalid character");
                }
            }
            Ok(Self::new(problem))
        } else {
            Err("Invalid length")
        }
    }
}

impl Candidates {
    const NONE: Self = Self(0);
    const ALL: Self = Self(0b111_111_111);

    fn insert(&mut self, value: &u8) {
        self.0 |= 1 << value - 1;
    }

    fn insert_some(&mut self, others: &Candidates) {
        self.0 |= others.0;
    }

    fn merge(&self, others: &Candidates) -> Candidates {
        Self(self.0 | others.0)
    }

    fn remove(&mut self, value: &u8) {
        self.0 &= !(1 << value - 1);
    }

    fn remove_some(&mut self, others: &Candidates) {
        self.0 &= !others.0;
    }

    fn exclude(&self, others: &Candidates) -> Candidates {
        Self(self.0 & !others.0)
    }

    fn single_or_none(&self) -> Option<NonZero<u8>> {
        if self.0 > 0 && self.0 & (self.0 - 1) == 0 {
            if let Some(value) = NonZero::new(self.0.trailing_zeros() as u8 + 1) {
                return Some(value);
            }
        }
        None
    }

    fn contains(&self, value: &u8) -> bool {
        self.0 & (1 << value - 1) != 0
    }

    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn iter(self) -> impl Iterator<Item = NonZero<u8>> {
        (1..=9)
            .filter(move |v| self.contains(v))
            .map(|v| NonZero::new(v).expect("Cannot be zero"))
    }
}

impl TryFrom<u16> for Candidates {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 0b111_111_111 {
            return Ok(Self(value));
        }
        Err("Only 9 bits are allowed")
    }
}

impl Cell {
    const EMPTY: Self = Self(None);

    fn value(self) -> Option<u8> {
        self.0.map(|v| v.get())
    }
}

impl TryFrom<u8> for Cell {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 10 {
            if let Some(v) = NonZero::new(value) {
                return Ok(Self(Some(v)));
            }
            return Ok(Self(None));
        }
        Err("Value must be between 0 and 9")
    }
}
