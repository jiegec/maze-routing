use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt;
use wasm_bindgen::prelude::*;

mod lee;
mod hadlock;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CellState {
    Empty,
    Blocked,
    Cross,
    // 180 angles
    LR,
    UD,
    // 90 angles
    LU,
    RU,
    LD,
    RD,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CellState::*;
        let ch = match self {
            Empty => '.',
            Blocked => 'x',
            Cross => '╋',
            LR => '━',
            UD => '┃',
            LU => '┛',
            RU => '┗',
            LD => '┓',
            RD => '┏',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    L,
    R,
    D,
    U,
}

impl Direction {
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            L => R,
            R => L,
            D => U,
            U => D,
        }
    }

    fn can_cross(&self, state: &CellState) -> bool {
        use Direction::*;
        match self {
            L | R => *state == CellState::Empty || *state == CellState::UD,
            U | D => *state == CellState::Empty || *state == CellState::LR,
        }
    }

    fn will_cross(&self, state: &CellState) -> bool {
        use Direction::*;
        match self {
            L | R => *state == CellState::UD,
            U | D => *state == CellState::LR,
        }
    }

    fn offset(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            L => (-1, 0),
            R => (1, 0),
            U => (0, 1),
            D => (0, -1),
        }
    }

    fn get_new_cell_state(&self, prev: &Direction, old_state: &CellState) -> CellState {
        use CellState::*;
        use Direction::*;
        match (self, prev) {
            (L, L) | (L, R) | (R, L) | (R, R) => match old_state {
                UD => Cross,
                _ => LR,
            },
            (U, U) | (U, D) | (D, U) | (D, D) => match old_state {
                LR => Cross,
                _ => UD,
            },
            (L, U) | (D, R) => LD,
            (L, D) | (U, R) => LU,
            (R, U) | (D, L) => RD,
            (R, D) | (U, L) => RU,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Maze {
    map: Vec<Vec<CellState>>,
    m: usize,
    n: usize,
}

#[wasm_bindgen]
impl Maze {
    /// Create maze of m x n
    #[wasm_bindgen(constructor)]
    pub fn new(m: usize, n: usize) -> Maze {
        Maze {
            map: vec![vec![CellState::Empty; n]; m],
            m,
            n,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.map[x][y]
    }

    /// fills all points in rectangle (x1, y1) to (x2, y2) to blocked
    pub fn fill(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let from_x = min(x1, x2);
        let to_x = max(x1, x2);
        let from_y = min(y1, y2);
        let to_y = max(y1, y2);
        assert!(to_x < self.m);
        assert!(to_y < self.n);
        for i in from_x..(to_x + 1) {
            for j in from_y..(to_y + 1) {
                self.map[i][j] = CellState::Blocked;
            }
        }
    }

    pub fn clear(&mut self) {
        for line in &mut self.map {
            for item in line {
                *item = CellState::Empty;
            }
        }
    }

    /// Soukup, Jiri. (1978). Fast Maze Router. Proc. DAC. 100- 102. 10.1109/DAC.1978.1585154.
    /// Soukup's algorithm, find one path
    pub fn soukup(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        false
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.n).rev() {
            for x in 0..self.m {
                write!(f, "{}", self.map[x][y])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
