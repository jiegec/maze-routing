//! C. Y. Lee, "An Algorithm for Path Connections and Its Applications," in IRE Transactions on Electronic Computers, vol. EC-10, no. 3, pp. 346-365, Sept. 1961.
use super::*;
use std::collections::BTreeSet;

#[derive(Eq, PartialEq)]
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
            Ordering::Equal => match other.dist.cmp(&self.dist) {
                Ordering::Equal => match self.x.cmp(&other.x) {
                    Ordering::Equal => self.y.cmp(&other.y),
                    res @ _ => res,
                },
                res @ _ => res,
            },
            res @ _ => res,
        }
    }
}

impl PartialOrd for LeeMinCrossingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct LeeMinEdgeEffectState {
    x: usize,
    y: usize,
    edges: usize,
    dist: usize,
}

impl Ord for LeeMinEdgeEffectState {
    fn cmp(&self, other: &Self) -> Ordering {
        // from small to big
        match other.edges.cmp(&self.edges) {
            Ordering::Equal => match other.dist.cmp(&self.dist) {
                Ordering::Equal => match self.x.cmp(&other.x) {
                    Ordering::Equal => self.y.cmp(&other.y),
                    res @ _ => res,
                },
                res @ _ => res,
            },
            res @ _ => res,
        }
    }
}

impl PartialOrd for LeeMinEdgeEffectState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[wasm_bindgen]
impl Maze {
    /// Lee's algorithm, find shortest path
    pub fn lee(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        if x1 == x2 && y1 == y2 {
            changes.push((x1, y1, CellState::Blocked));
            return Some(ChangeSet { changes: changes });
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
        None
    }

    /// Lee's algorithm, find shortest path
    pub fn lee_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.lee(x1, y1, x2, y2) {
            Some(changes) => {
                self.apply(&changes);
                true
            }
            None => false,
        }
    }

    /// Lee's algorithm, find shortest path with minimum crossing
    pub fn lee_minimum_crossing(
        &self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        if x1 == x2 && y1 == y2 {
            changes.push((x1, y1, CellState::Blocked));
            return Some(ChangeSet { changes: changes });
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
        None
    }

    /// Lee's algorithm, find shortest path with minimum crossing
    pub fn lee_minimum_crossing_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.lee_minimum_crossing(x1, y1, x2, y2) {
            Some(changes) => {
                self.apply(&changes);
                true
            }
            None => false,
        }
    }

    /// Lee's algorithm, find shortest path with minimum edge effect
    pub fn lee_minimum_edge_effect(
        &self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        if x1 == x2 && y1 == y2 {
            changes.push((x1, y1, CellState::Blocked));
            return Some(ChangeSet { changes: changes });
        }

        let mut queue = BinaryHeap::new();
        let mut dir_map = vec![vec![None; self.n]; self.m];
        dir_map[x1][y1] = Some(L);
        queue.push(LeeMinEdgeEffectState {
            x: x1,
            y: y1,
            edges: 0,
            dist: 0,
        });
        let m = self.m as isize;
        let n = self.n as isize;
        while let Some(LeeMinEdgeEffectState { x, y, edges, dist }) = queue.pop() {
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
                        dir_map[new_x][new_y] = Some(direction.opposite());
                        // edge effect: the number of neighbors that are not one of [Empty, LR, UD]
                        let mut new_edges = 0;
                        for direction in &[L, R, U, D] {
                            let (dx, dy) = direction.offset();
                            let new_x = new_x as isize + dx;
                            let new_y = new_y as isize + dy;
                            if 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                                let state = self.map[new_x as usize][new_y as usize];
                                if state != CellState::Empty
                                    && state != CellState::LR
                                    && state != CellState::UD
                                {
                                    new_edges += 1;
                                }
                            }
                        }
                        queue.push(LeeMinEdgeEffectState {
                            x: new_x,
                            y: new_y,
                            edges: edges + new_edges,
                            dist: dist + 1,
                        });
                    }
                }
            }
        }
        None
    }

    /// Lee's algorithm, find shortest path with minimum edge effect
    pub fn lee_minimum_edge_effect_mut(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        match self.lee_minimum_edge_effect(x1, y1, x2, y2) {
            Some(changes) => {
                self.apply(&changes);
                true
            }
            None => false,
        }
    }

    /// Lee's algorithm, find rectified steiner tree
    pub fn lee_multi(&self, points: &Points) -> Option<ChangeSet> {
        use Direction::*;
        let mut changes = vec![];
        let points = points.get();
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
        queue.push_back((x1, y1));
        new_map[x1][y1] = CellState::Blocked;
        changes.push((x1, y1, CellState::Blocked));

        let m = self.m as isize;
        let n = self.n as isize;
        while let Some((x, y)) = queue.pop_front() {
            if dest_points.contains(&(x, y)) {
                // found
                dest_points.remove(&(x, y));
                new_map[x][y] = CellState::Blocked;
                changes.push((x, y, CellState::Blocked));

                let mut direction = dir_map[x][y].unwrap();
                let mut cur_x = x;
                let mut cur_y = y;
                loop {
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
            for direction in &[L, R, U, D] {
                let (dx, dy) = direction.offset();
                let new_x = x + dx;
                let new_y = y + dy;
                if 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    if dir_map[new_x][new_y].is_none()
                        && direction.can_cross(&new_map[new_x][new_y])
                    {
                        dir_map[new_x as usize][new_y as usize] = Some(direction.opposite());
                        queue.push_back((new_x, new_y));
                    }
                }
            }
        }
        None
    }

    /// Lee's algorithm, find rectified steiner tree
    pub fn lee_multi_mut(&mut self, points: &Points) -> bool {
        match self.lee_multi(points) {
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
    fn lee_basic() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.lee_mut(1, 0, 1, 2));
        println!("{}", maze);
        assert!(maze.lee_mut(0, 1, 2, 0));
        println!("{}", maze);
        assert!(!maze.lee_mut(0, 2, 2, 2));
    }

    #[test]
    fn lee_min_crossing() {
        let mut maze = Maze::new(4, 4);
        assert!(maze.lee_mut(0, 1, 2, 1));
        println!("{}", maze);
        let mut maze_orig = maze.clone();
        assert!(maze_orig.lee_mut(1, 0, 1, 2));
        println!("{}", maze);
        assert_eq!(maze_orig.get(3, 1), CellState::Empty);
        assert!(maze.lee_minimum_crossing_mut(1, 0, 1, 2));
        println!("{}", maze);
        assert_eq!(maze.get(3, 1), CellState::UD);
    }

    #[test]
    fn lee_min_edge_effect() {
        let mut maze = Maze::new(4, 4);
        assert!(maze.lee_mut(1, 1, 3, 1));
        println!("{}", maze);
        let mut maze_orig = maze.clone();
        assert!(maze_orig.lee_mut(0, 0, 2, 2));
        println!("{}", maze_orig);
        assert!(maze.lee_minimum_edge_effect_mut(0, 0, 2, 2));
        println!("{}", maze);
        assert_eq!(maze.get(0, 2), CellState::UD);

        let mut maze = Maze::new(5, 5);
        assert!(maze.lee_minimum_edge_effect_mut(0, 0, 4, 4));
        println!("{}", maze);
    }

    #[test]
    fn lee_multi() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.lee_multi_mut(&Points {
            points: vec![(0, 0), (1, 1), (2, 2)]
        }));
        println!("{}", maze);
        assert!(!maze.lee_multi_mut(&Points {
            points: vec![(2, 0), (0, 2)]
        }));

        let mut maze = Maze::new(5, 5);
        assert!(maze.lee_multi_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0), (4, 4)]
        }));
        println!("{}", maze);
        assert!(maze.lee_multi_mut(&Points {
            points: vec![(0, 3), (2, 3)]
        }));
        println!("{}", maze);

        let mut maze = Maze::new(5, 5);
        assert!(maze.lee_multi_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0)]
        }));
        println!("{}", maze);
        assert!(maze.lee_multi_mut(&Points {
            points: vec![(0, 3), (4, 3)]
        }));
        println!("{}", maze);
    }

    #[test]
    fn lee_regression_1() {
        let mut maze = Maze::new(2, 1);
        assert!(maze.lee_mut(1, 0, 0, 0));
        println!("{}", maze);
    }

    #[test]
    fn lee_multi_regression_1() {
        let mut maze = Maze::new(2, 1);
        assert!(maze.lee_multi_mut(&Points {
            points: vec![(0, 0), (0, 0)]
        }));
        println!("{}", maze);
    }

    quickcheck! {
        fn qc_lee(m: usize, n: usize, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
            if m == 0 || n == 0 {
                return true;
            }
            // check oom
            if m > 10000 || n > 10000 {
                return true
            }

            let mut maze = Maze::new(m, n);
            maze.lee_mut(x1 % m, y1 % n, x2 % m, y2 % n) && maze.verify()
        }

        fn qc_lee_many(m: usize, n: usize, points: Vec<(usize, usize, usize, usize)>) -> bool {
            if m == 0 || n == 0 {
                return true;
            }
            // check oom
            if m > 10000 || n > 10000 {
                return true
            }

            let mut maze = Maze::new(m, n);
            for (x1, y1, x2, y2) in points {
                maze.lee_mut(x1 % m, y1 % n, x2 % m, y2 % n);
            }
            maze.verify()
        }

        fn qc_lee_multi(m: usize, n: usize, points: Vec<(usize, usize)>) -> bool {
            if m == 0 || n == 0 {
                return true;
            }
            // check oom
            if m > 10000 || n > 10000 {
                return true
            }

            let mut points = points.clone();
            for point in &mut points {
                point.0 %= m;
                point.1 %= n;
            }
            let mut maze = Maze::new(m, n);
            maze.lee_multi_mut(&Points {
                points
            }) && maze.verify()
        }
    }
}
