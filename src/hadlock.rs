//! Hadlock, Frank O.. “A shortest path algorithm for grid graphs.” Networks 7 (1977): 323-334.
use super::*;

fn manhattan_dist(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    max(x1, x2) - min(x1, x2) + max(y1, y2) - min(y1, y2)
}

#[derive(Eq)]
struct HadlockCrossingState {
    x: usize,
    y: usize,
    dist: usize,
}

impl Ord for HadlockCrossingState {
    fn cmp(&self, other: &Self) -> Ordering {
        // from small to big
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for HadlockCrossingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HadlockCrossingState {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

#[wasm_bindgen]
impl Maze {
    /// Hadlock's algorithm, find shortest path like a*
    pub fn hadlock(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        if self.map[x1][y1] != CellState::Empty || self.map[x2][y2] != CellState::Empty {
            return None;
        }
        if x1 == x2 && y1 == y2 {
            changes.push((x1, y1, CellState::Blocked));
            return Some(ChangeSet { changes });
        }

        let mut queue = BinaryHeap::new();
        let mut dir_map = vec![vec![None; self.n]; self.m];
        dir_map[x1][y1] = Some(L);
        queue.push(HadlockCrossingState {
            x: x1,
            y: y1,
            dist: manhattan_dist(x1, y1, x2, y2),
        });
        let m = self.m as isize;
        let n = self.n as isize;
        while let Some(HadlockCrossingState { x, y, dist: _dist }) = queue.pop() {
            if x == x2 && y == y2 {
                // found
                let mut direction = dir_map[x][y].unwrap();
                let mut cur_x = x;
                let mut cur_y = y;
                loop {
                    let (dx, dy) = direction.offset();
                    let new_x = (cur_x as isize + dx) as usize;
                    let new_y = (cur_y as isize + dy) as usize;
                    if new_x == x1 && new_y == y1 {
                        break;
                    }
                    let new_direction = dir_map[new_x][new_y].unwrap();
                    changes.push((
                        new_x,
                        new_y,
                        new_direction.get_new_cell_state(&direction, &self.map[new_x][new_y]),
                    ));
                    cur_x = new_x;
                    cur_y = new_y;
                    direction = new_direction;
                }

                changes.push((x1, y1, CellState::Blocked));
                changes.push((x2, y2, CellState::Blocked));
                return Some(ChangeSet { changes });
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
                        let towards = match direction {
                            L => x2 < x as usize,
                            R => x2 > x as usize,
                            U => y2 > y as usize,
                            D => y2 < y as usize,
                        };
                        queue.push(HadlockCrossingState {
                            x: new_x,
                            y: new_y,
                            dist: manhattan_dist(new_x, new_y, x2, y2) + towards as usize,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn hadlock_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.hadlock(x1, y1, x2, y2) {
            Some(changes) => {
                self.apply(&changes);
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn hadlock() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.hadlock_mut(1, 0, 1, 2));
        println!("{}", maze);
        assert!(maze.hadlock_mut(0, 1, 2, 0));
        println!("{}", maze);
        assert!(!maze.hadlock_mut(0, 2, 2, 2));

        // taken from http://cc.ee.ntu.edu.tw/~jhjiang/instruction/courses/spring11-eda/lec06-3_4p.pdf
        let mut maze = Maze::new(13, 13);
        maze.fill_mut(5, 0, 5, 4);
        maze.fill_mut(3, 5, 5, 5);
        maze.fill_mut(5, 6, 5, 8);
        maze.fill_mut(5, 11, 8, 11);
        maze.fill_mut(6, 10, 6, 10);
        assert!(maze.hadlock_mut(3, 4, 9, 6));
        println!("{}", maze);
        assert_eq!(maze.get(2, 4), CellState::RU);
        assert_eq!(maze.get(4, 6), CellState::LU);
    }

    quickcheck! {
        fn qc_hadlock(m: usize, n: usize, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
            if m == 0 || n == 0 {
                return true;
            }
            // check oom
            if m > 10000 || n > 10000 {
                return true
            }

            let mut maze = Maze::new(m, n);
            maze.hadlock_mut(x1 % m, y1 % n, x2 % m, y2 % n) && maze.verify()
        }

        fn qc_hadlock_many(m: usize, n: usize, points: Vec<(usize, usize, usize, usize)>) -> bool {
            if m == 0 || n == 0 {
                return true;
            }
            // check oom
            if m > 10000 || n > 10000 {
                return true
            }

            let mut maze = Maze::new(m, n);
            for (x1, y1, x2, y2) in points {
                maze.hadlock_mut(x1 % m, y1 % n, x2 % m, y2 % n);
            }
            maze.verify()
        }
    }
}
