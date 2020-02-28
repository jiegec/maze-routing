use serde_derive::{Deserialize, Serialize};
use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt;
use wasm_bindgen::prelude::*;

mod hadlock;
mod lee;
mod mikami_tabuchi;
mod soukup;
mod stst;

/// Cell's state
#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CellState {
    /// Empty cell
    Empty,
    /// Blocked cell
    Blocked,
    /// Up, right, down and left
    Cross,
    // 180 angles
    /// Left and right
    LR,
    /// Up and down
    UD,
    // 90 angles
    /// Left and up
    LU,
    /// Right and up
    RU,
    /// Left and down
    LD,
    /// Right and down
    RD,
    // three directions
    /// Left, up and right
    LUR,
    /// Up, right and down
    URD,
    /// Right, down and left
    RDL,
    /// Down, left and up
    DLU,
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

impl CellState {
    // L, R, U, D
    fn extract(&self) -> [bool; 4] {
        use CellState::*;
        match self {
            Empty => [false, false, false, false],
            Blocked => [true, true, true, true],
            Cross => [true, true, true, true],
            LR => [true, true, false, false],
            UD => [false, false, true, true],
            LU => [true, false, true, false],
            LD => [true, false, false, true],
            RU => [false, true, true, false],
            RD => [false, true, false, true],
            LUR => [true, true, true, false],
            URD => [false, true, true, true],
            RDL => [true, true, false, true],
            DLU => [true, false, true, true],
        }
    }
}

/// Four directions
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    L,
    R,
    D,
    U,
}

impl Direction {
    /// Get the opposite direction
    pub fn opposite(&self) -> Direction {
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

    fn index(&self) -> usize {
        use Direction::*;
        match self {
            L => 0,
            R => 1,
            U => 2,
            D => 3,
        }
    }

    fn get_new_cell_state(&self, prev: &Direction, old_state: &CellState) -> CellState {
        use CellState::*;
        if *old_state == Blocked {
            return Blocked;
        }
        // L, R, U, D
        let mut directions = old_state.extract();
        directions[self.index()] = true;
        directions[prev.opposite().index()] = true;
        // L, R, U, D
        match directions {
            [false, false, true, true] => UD,
            [false, true, false, true] => RD,
            [false, true, true, false] => RU,
            [true, false, false, true] => LD,
            [true, false, true, false] => LU,
            [true, true, false, false] => LR,
            [false, true, true, true] => URD,
            [true, false, true, true] => DLU,
            [true, true, false, true] => RDL,
            [true, true, true, false] => LUR,
            [true, true, true, true] => Cross,
            _ => unreachable!(),
        }
    }
}

/// A grid maze.
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

    /// Set all points in rectangle (x1, y1) to (x2, y2) to empty
    pub fn clean_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let from_x = min(x1, x2);
        let to_x = max(x1, x2);
        let from_y = min(y1, y2);
        let to_y = max(y1, y2);
        assert!(to_x < self.m);
        assert!(to_y < self.n);
        for i in from_x..(to_x + 1) {
            for j in from_y..(to_y + 1) {
                self.map[i][j] = CellState::Empty;
            }
        }
    }

    /// Set all points in rectangle (x1, y1) to (x2, y2) to empty
    pub fn clean(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Maze {
        let mut inst = self.clone();
        inst.clean_mut(x1, y1, x2, y2);
        inst
    }

    /// Set all cells to empty
    pub fn clear_mut(&mut self) {
        for line in &mut self.map {
            for item in line {
                *item = CellState::Empty;
            }
        }
    }

    /// Apply changeset
    pub fn apply(&mut self, changes: &ChangeSet) {
        for (x, y, state) in &changes.changes {
            self.map[*x][*y] = *state;
        }
    }

    /// Sanity check
    pub fn verify(&self) -> bool {
        use Direction::*;
        for x in 0..self.m {
            for y in 0..self.n {
                if self.map[x][y] == CellState::Blocked || self.map[x][y] == CellState::Empty {
                    continue;
                }
                // L, R, U, D
                let ways = self.map[x][y].extract();
                let dirs = [L, R, U, D];
                for i in 0..4 {
                    if ways[i] {
                        let (dx, dy) = dirs[i].offset();
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx < 0 || nx >= self.m as isize || ny < 0 || ny >= self.n as isize {
                            return false;
                        }
                        let cell = self.map[nx as usize][ny as usize];
                        let ways_other = cell.extract();
                        if !ways_other[dirs[i].opposite().index()] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
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

/// Represent a changeset of maze cells.
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    changes: Vec<(usize, usize, CellState)>,
}

#[wasm_bindgen]
impl ChangeSet {
    /// For use in JS.
    pub fn to_js(&self) -> JsValue {
        JsValue::from_serde(&self.changes).unwrap()
    }
}

/// A vector of points.
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Points {
    points: Vec<(usize, usize)>,
}

impl Points {
    pub fn new(points: &[(usize, usize)]) -> Points {
        Points {
            points: Vec::from(points),
        }
    }

    /// get a deduped and sorted list of points
    fn get(&self) -> Vec<(usize, usize)> {
        let mut res = self.points.clone();
        res.sort();
        res.dedup();
        res
    }
}

#[wasm_bindgen]
impl Points {
    /// For use in JS.
    pub fn to_js(&self) -> JsValue {
        JsValue::from_serde(&self.points).unwrap()
    }

    #[wasm_bindgen(constructor)]
    pub fn from_js(js: JsValue) -> Points {
        JsValue::into_serde(&js)
            .ok()
            .map(|points| Points { points })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;
    use serde_json;

    #[test]
    fn serde_points() {
        println!(
            "{}",
            serde_json::to_string(&Points {
                points: vec![(1, 2), (3, 4)]
            })
            .unwrap()
        );
    }

    quickcheck! {
        fn qc_sanity(m: usize, n: usize) -> bool {
            let maze = Maze::new(m, n);
            maze.verify()
        }
    }
}
