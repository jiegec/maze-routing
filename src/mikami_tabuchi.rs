use super::*;

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

    pub fn mikami_tabuchi_mut(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.mikami_tabuchi(x1, y1, x2, y2) {
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
}
