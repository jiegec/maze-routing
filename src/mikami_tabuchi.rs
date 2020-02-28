//! Mikami, Koichi. “Tabuchi: a computer program for optimal routing of printed circuit connectors.” (1970).
use super::*;
use std::collections::BTreeSet;

#[wasm_bindgen]
impl Maze {
    /// Mikami-Tabuchi's algorithm, line search
    pub fn mikami_tabuchi(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        if x1 == x2 && y1 == y2 {
            changes.push((x1, y1, CellState::Blocked));
            return Some(ChangeSet { changes: changes });
        }

        let mut queue = VecDeque::new();
        let mut dir_map = vec![vec![None; self.n]; self.m];
        dir_map[x1][y1] = Some(L);
        queue.push_back((x1, y1, L));
        queue.push_back((x1, y1, R));
        queue.push_back((x1, y1, U));
        queue.push_back((x1, y1, D));
        let m = self.m as isize;
        let n = self.n as isize;
        while let Some((x, y, direction)) = queue.pop_front() {
            if x == x2 && y == y2 {
                // found
                let mut direction = dir_map[x][y].unwrap();
                let mut cur_x = x;
                let mut cur_y = y;
                while cur_x != x1 || cur_y != y1 {
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
                return Some(ChangeSet { changes: changes });
            }

            let x = x as isize;
            let y: isize = y as isize;
            let (dx, dy) = direction.offset();
            let mut new_x = x + dx;
            let mut new_y = y + dy;
            while 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                if dir_map[new_x as usize][new_y as usize].is_none()
                    && direction.can_cross(&self.map[new_x as usize][new_y as usize])
                {
                    dir_map[new_x as usize][new_y as usize] = Some(direction.opposite());
                    match direction {
                        L | R => {
                            queue.push_back((new_x as usize, new_y as usize, U));
                            queue.push_back((new_x as usize, new_y as usize, D));
                        }
                        U | D => {
                            queue.push_back((new_x as usize, new_y as usize, L));
                            queue.push_back((new_x as usize, new_y as usize, R));
                        }
                    }
                    new_x += dx;
                    new_y += dy;
                } else {
                    break;
                }
            }
        }
        None
    }

    /// Mikami-Tabuchi's algorithm, line search
    pub fn mikami_tabuchi_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.mikami_tabuchi(x1, y1, x2, y2) {
            Some(changes) => {
                self.apply(&changes);
                true
            }
            None => false,
        }
    }

    /// Mikami-Tabuchi's algorithm for multiple points, line search
    pub fn mikami_tabuchi_multi(&self, points: &Points) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        let mut points = points.points.clone();
        points.sort();
        points.dedup();
        if points.len() == 0 {
            return Some(ChangeSet { changes });
        } else if points.len() == 1 {
            changes.push((points[0].0, points[0].1, CellState::Blocked));
            return Some(ChangeSet { changes });
        }

        let mut dest_points = BTreeSet::new();
        for point in &points[1..] {
            dest_points.insert(point);
        }

        let mut queue = VecDeque::new();
        let mut dir_map = vec![vec![None; self.n]; self.m];
        let mut new_map = self.map.clone();
        let (x1, y1) = points[0];
        dir_map[x1][y1] = Some(L);
        queue.push_back((x1, y1, L));
        queue.push_back((x1, y1, R));
        queue.push_back((x1, y1, U));
        queue.push_back((x1, y1, D));
        new_map[x1][y1] = CellState::Blocked;
        changes.push((x1, y1, CellState::Blocked));

        let m = self.m as isize;
        let n = self.n as isize;
        while let Some((x, y, direction)) = queue.pop_front() {
            if dest_points.contains(&(x, y)) {
                // found
                dest_points.remove(&(x, y));
                new_map[x][y] = CellState::Blocked;
                changes.push((x, y, CellState::Blocked));

                let mut direction = dir_map[x][y].unwrap();
                let mut cur_x = x;
                let mut cur_y = y;
                while cur_x != x1 || cur_y != y1 {
                    let (dx, dy) = direction.offset();
                    let new_x = (cur_x as isize + dx) as usize;
                    let new_y = (cur_y as isize + dy) as usize;
                    if new_map[new_x][new_y] == CellState::Blocked {
                        break;
                    }

                    let new_direction = dir_map[new_x][new_y].unwrap();
                    let new_cell_state =
                        new_direction.get_new_cell_state(&direction, &new_map[new_x][new_y]);
                    new_map[new_x][new_y] = new_cell_state;
                    changes.push((new_x, new_y, new_cell_state));
                    cur_x = new_x;
                    cur_y = new_y;
                    direction = new_direction;
                }

                if dest_points.len() == 0 {
                    return Some(ChangeSet { changes: changes });
                }
            }

            let x = x as isize;
            let y: isize = y as isize;
            let (dx, dy) = direction.offset();
            let mut new_x = x + dx;
            let mut new_y = y + dy;
            while 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                if dir_map[new_x as usize][new_y as usize].is_none()
                    && direction.can_cross(&new_map[new_x as usize][new_y as usize])
                {
                    dir_map[new_x as usize][new_y as usize] = Some(direction.opposite());
                    match direction {
                        L | R => {
                            queue.push_back((new_x as usize, new_y as usize, U));
                            queue.push_back((new_x as usize, new_y as usize, D));
                        }
                        U | D => {
                            queue.push_back((new_x as usize, new_y as usize, L));
                            queue.push_back((new_x as usize, new_y as usize, R));
                        }
                    }
                    new_x += dx;
                    new_y += dy;
                } else {
                    break;
                }
            }
        }
        None
    }

    /// Mikami-Tabuchi's algorithm for multiple points, line search
    pub fn mikami_tabuchi_multi_mut(&mut self, points: &Points) -> bool {
        match self.mikami_tabuchi_multi(points) {
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
    fn mikami_tabuchi() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.mikami_tabuchi_mut(0, 0, 2, 2));
        println!("{}", maze);

        let mut maze = Maze::new(3, 3);
        maze.fill_mut(2, 0, 2, 0);
        assert!(maze.mikami_tabuchi_mut(0, 0, 2, 2));
        println!("{}", maze);
    }

    #[test]
    fn mikami_tabuchi_multi() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.mikami_tabuchi_multi_mut(&Points {
            points: vec![(0, 0), (1, 1), (2, 2)]
        }));
        println!("{}", maze);
        assert!(!maze.mikami_tabuchi_multi_mut(&Points {
            points: vec![(2, 0), (0, 2)]
        }));

        let mut maze = Maze::new(5, 5);
        assert!(maze.mikami_tabuchi_multi_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0), (4, 4)]
        }));
        println!("{}", maze);
        assert!(maze.mikami_tabuchi_multi_mut(&Points {
            points: vec![(0, 3), (2, 3)]
        }));
        println!("{}", maze);

        let mut maze = Maze::new(5, 5);
        assert!(maze.mikami_tabuchi_multi_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0)]
        }));
        println!("{}", maze);
        assert!(maze.mikami_tabuchi_multi_mut(&Points {
            points: vec![(0, 3), (4, 3)]
        }));
        println!("{}", maze);
    }

    quickcheck! {
        fn qc_mikami_tabuchi_many(m: usize, n: usize, points: Vec<(usize, usize, usize, usize)>) -> bool {
            if m == 0 || n == 0 {
                return true;
            }
            let mut maze = Maze::new(m, n);
            for (x1, y1, x2, y2) in points {
                maze.mikami_tabuchi_mut(x1 % m, y1 % n, x2 % m, y2 % n);
            }
            maze.verify()
        }
    }
}
