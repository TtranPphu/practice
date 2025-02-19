use std::num::NonZero;

#[derive(Clone, Debug)]
pub struct Board {
    cells: [u8; 81],
    candidateses: [Candidates; 81],
    unset: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Candidates(u16);

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

    fn solved(&self) -> bool {
        self.unset == 0
    }

    fn naked_single(&self, index: usize) -> Option<NonZero<u8>> {
        let candidate = &self.candidateses[index];
        if candidate.is_alone() {
            if let Some(value) = NonZero::new(candidate.to_value()) {
                return Some(value);
            }
        }
        None
    }

    fn hidden_single(&self, index: usize) -> Option<NonZero<u8>> {
        let neighbors = Self::neighbors(index);
        let candidate = neighbors
            .into_iter()
            .filter(|&n| self.cells[n] == 0)
            .fold(self.candidateses[index], |c, n| c & self.candidateses[n]);
        if candidate.is_alone() {
            if let Some(value) = NonZero::new(candidate.to_value()) {
                return Some(value);
            }
        }
        None
    }

    fn set(&mut self, index: usize, value: NonZero<u8>) {
        let v = value.get();
        self.unset -= 1;
        self.cells[index] = v;
        self.candidateses[index] = Candidates::none();
        for &neighbor in &Self::neighbors(index) {
            self.candidateses[neighbor] &= Candidates::from_value(v);
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
            cells: [0; 81],
            candidateses: [Candidates::all(); 81],
            unset: 81,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "╔═══════╤═══════╤═══════╗")?;
        for index in 0..81 {
            if index == 27 || index == 54 {
                writeln!(f, "╟───────┼───────┼───────╢")?;
            }
            if index % 9 == 0 {
                write!(f, "║")?;
            } else if index % 3 == 0 {
                write!(f, "│")?;
            }
            write!(
                f,
                " {}",
                if self.cells[index] == 0 {
                    '.'
                } else {
                    char::from(b'0' + self.cells[index])
                }
            )?;
            if index % 9 == 8 {
                writeln!(f, " ║")?;
            }
        }
        writeln!(f, "╚═══════╧═══════╧═══════╝")
    }
}

impl Candidates {
    fn none() -> Self {
        Self(0)
    }

    fn all() -> Self {
        Self(0b111_111_1110)
    }

    fn is_alone(self) -> bool {
        self.0 & (self.0 - 1) == 0
    }

    fn contains(self, value: u8) -> bool {
        self.0 & (1 << value) != 0
    }

    fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    fn iter(self) -> impl Iterator<Item = NonZero<u8>> {
        (1..=9)
            .filter(move |&v| self.contains(v))
            .map(|v| NonZero::new(v).unwrap())
    }

    fn to_value(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    fn from_value(value: u8) -> Self {
        Self(1 << value)
    }
}

impl<T> From<T> for Candidates
where
    T: Into<u16>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl std::ops::BitAnd for Candidates {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Candidates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
