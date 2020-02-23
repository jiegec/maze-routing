use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt;
use wasm_bindgen::prelude::*;

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
            Empty => ' ',
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
    pub m: usize,
    pub n: usize,
}

#[derive(Eq)]
struct LeeMinCrossingState {
    x: usize,
    y: usize,
    crosses: usize,
    dist: usize,
}

impl Ord for LeeMinCrossingState {
    fn cmp(&self, other: &Self) -> Ordering {
        // from small to big
        match other.crosses.cmp(&self.crosses) {
            Ordering::Equal => other.dist.cmp(&self.dist),
            res @ _ => res
        }
    }
}

impl PartialOrd for LeeMinCrossingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LeeMinCrossingState {
    fn eq(&self, other: &Self) -> bool {
        self.crosses == other.crosses && self.dist == other.dist
    }
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

    // Lee's algorithm, find shortest path
    pub fn lee(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        use Direction::*;
        if self.map[x1][y1] != CellState::Empty || self.map[x2][y2] != CellState::Empty {
            return false;
        }
        if x1 == x2 && y1 == y2 {
            self.map[x1][y1] = CellState::Blocked;
            return true;
        }

        let mut queue = VecDeque::new();
        let mut dir_map = vec![vec![None; self.n]; self.m];
        dir_map[x1][y1] = Some(L);
        queue.push_back((x1, y1));
        let m = self.m as isize;
        let n = self.n as isize;
        while let Some((x, y)) = queue.pop_front() {
            if x == x2 && y == y2 {
                // found
                let mut direction = dir_map[x][y].unwrap();
                let mut cur_x = x;
                let mut cur_y = y;
                while cur_x != x1 || cur_y != y1 {
                    let (dx, dy) = direction.offset();
                    let new_x = (cur_x as isize + dx) as usize;
                    let new_y = (cur_y as isize + dy) as usize;
                    let new_direction = dir_map[new_x][new_y].unwrap();
                    self.map[new_x][new_y] =
                        new_direction.get_new_cell_state(&direction, &self.map[new_x][new_y]);
                    cur_x = new_x;
                    cur_y = new_y;
                    direction = new_direction;
                }

                self.map[x1][y1] = CellState::Blocked;
                self.map[x2][y2] = CellState::Blocked;
                return true;
            }

            let x = x as isize;
            let y: isize = y as isize;
            for direction in &[L, R, U, D] {
                let (dx, dy) = direction.offset();
                let new_x = x + dx;
                let new_y = y + dy;
                if 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    if dir_map[new_x][new_y].is_none()
                        && direction.can_cross(&self.map[new_x][new_y])
                    {
                        dir_map[new_x as usize][new_y as usize] = Some(direction.opposite());
                        queue.push_back((new_x, new_y));
                    }
                }
            }
        }
        false
    }

    // Lee's algorithm, find shortest path with minimum crossing
    pub fn lee_minimum_crossing(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        use Direction::*;
        if self.map[x1][y1] != CellState::Empty || self.map[x2][y2] != CellState::Empty {
            return false;
        }
        if x1 == x2 && y1 == y2 {
            self.map[x1][y1] = CellState::Blocked;
            return true;
        }

        let mut queue = BinaryHeap::new();
        let mut dir_map = vec![vec![None; self.n]; self.m];
        dir_map[x1][y1] = Some(L);
        queue.push(LeeMinCrossingState {
            x: x1,
            y: y1,
            crosses: 0,
            dist: 0,
        });
        let m = self.m as isize;
        let n = self.n as isize;
        while let Some(LeeMinCrossingState {
            x,
            y,
            crosses,
            dist,
        }) = queue.pop()
        {
            if x == x2 && y == y2 {
                // found
                let mut direction = dir_map[x][y].unwrap();
                let mut cur_x = x;
                let mut cur_y = y;
                while cur_x != x1 || cur_y != y1 {
                    let (dx, dy) = direction.offset();
                    let new_x = (cur_x as isize + dx) as usize;
                    let new_y = (cur_y as isize + dy) as usize;
                    let new_direction = dir_map[new_x][new_y].unwrap();
                    self.map[new_x][new_y] =
                        new_direction.get_new_cell_state(&direction, &self.map[new_x][new_y]);
                    cur_x = new_x;
                    cur_y = new_y;
                    direction = new_direction;
                }

                self.map[x1][y1] = CellState::Blocked;
                self.map[x2][y2] = CellState::Blocked;
                return true;
            }

            let x = x as isize;
            let y: isize = y as isize;
            for direction in &[L, R, U, D] {
                let (dx, dy) = direction.offset();
                let new_x = x + dx;
                let new_y = y + dy;
                if 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    if dir_map[new_x][new_y].is_none()
                        && direction.can_cross(&self.map[new_x][new_y])
                    {
                        dir_map[new_x as usize][new_y as usize] = Some(direction.opposite());
                        queue.push(LeeMinCrossingState {
                            x: new_x,
                            y: new_y,
                            crosses: crosses
                                + direction.will_cross(&self.map[new_x][new_y]) as usize,
                            dist: dist + 1,
                        });
                    }
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lee_basic() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.lee(1, 0, 1, 2));
        println!("{}", maze);
        assert!(maze.lee(0, 1, 2, 0));
        println!("{}", maze);
        assert!(!maze.lee(0, 2, 2, 2));
    }

    #[test]
    fn lee_min_crossing() {
        let mut maze = Maze::new(4, 4);
        assert!(maze.lee(0, 1, 2, 1));
        println!("{}", maze);
        let mut maze_orig = maze.clone();
        assert!(maze_orig.lee(1, 0, 1, 2));
        println!("{}", maze);
        assert_eq!(maze_orig.get(3, 1), CellState::Empty);
        assert!(maze.lee_minimum_crossing(1, 0, 1, 2));
        println!("{}", maze);
        assert_eq!(maze.get(3, 1), CellState::UD);
    }
}
