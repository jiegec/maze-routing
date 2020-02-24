use serde_derive::{Deserialize, Serialize};
use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt;
use wasm_bindgen::prelude::*;

mod hadlock;
mod lee;
mod soukup;
mod stst;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
    // three directions
    LUR,
    URD,
    RDL,
    DLU
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
            LUR => '┻',
            URD => '┣',
            RDL => '┳',
            DLU => '┫',
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
    pub fn fill_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
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

    /// fills all points in rectangle (x1, y1) to (x2, y2) to blocked
    pub fn fill(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Maze {
        let mut inst = self.clone();
        inst.fill_mut(x1, y1, x2, y2);
        inst
    }

    /// set all cells to empty
    pub fn clear_mut(&mut self) {
        for line in &mut self.map {
            for item in line {
                *item = CellState::Empty;
            }
        }
    }

    pub fn apply(&mut self, changes: &ChangeSet) {
        for (x, y, state) in &changes.changes {
            self.map[*x][*y] = *state;
        }
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

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    changes: Vec<(usize, usize, CellState)>,
}

#[wasm_bindgen]
impl ChangeSet {
    pub fn to_js(&self) -> JsValue {
        JsValue::from_serde(&self.changes).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Points {
    points: Vec<(usize, usize)>,
}

#[wasm_bindgen]
impl Points {
    pub fn to_js(&self) -> JsValue {
        JsValue::from_serde(&self.points).unwrap()
    }

    pub fn from_js(js: JsValue) -> Option<Points> {
        JsValue::into_serde(&js).ok().map(|points| Points {
            points
        })
    }
}
