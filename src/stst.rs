//! Single Trunk Steiner Tree

use super::*;

impl Maze {
    pub fn stst(&self, points: &Points) -> Option<ChangeSet> {
        use CellState::*;
        let mut points = points.points.clone();
        if points.len() == 0 {
            return None;
        }
        points.sort();
        points.dedup();
        for (x, y) in &points {
            if self.map[*x][*y] != Empty {
                return None;
            }
        }

        let (mut min_x, mut min_y) = points[0];
        let (mut max_x, mut max_y) = points[0];
        for (x, y) in &points[1..] {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        let mut ans: Option<ChangeSet> = None;

        // try horizontal
        for (_point_x, point_y) in &points {
            // construct a trunk from this point
            let mut changes: Vec<(usize, usize, CellState)> = vec![];
            let mut down = vec![false; max_x - min_x + 1];
            let mut up = vec![false; max_x - min_x + 1];
            let mut on = vec![false; max_x - min_x + 1];
            for (new_x, new_y) in &points {
                if new_y == point_y {
                    on[new_x - min_x] = true;
                } else if new_y > point_y {
                    up[new_x - min_x] = true;
                } else {
                    down[new_x - min_x] = true;
                }
            }

            // x = min_x
            if on[0] {
                // handled below
            } else if up[0] && down[0] {
                changes.push((min_x, *point_y, URD));
            } else if up[0] {
                changes.push((min_x, *point_y, RU));
            } else if down[0] {
                changes.push((min_x, *point_y, RD));
            } else {
                unreachable!();
            }

            // x = max_x
            if on[max_x - min_x] {
                // handled below
            } else if up[max_x - min_x] && down[max_x - min_x] {
                changes.push((max_x, *point_y, DLU));
            } else if up[max_x - min_x] {
                changes.push((max_x, *point_y, LU));
            } else if down[max_x - min_x] {
                changes.push((max_x, *point_y, LD));
            } else {
                unreachable!();
            }

            for x in (min_x + 1)..(max_x) {
                if on[x - min_x] {
                    // handled below
                } else if up[x - min_x] && down[x - min_x] {
                    changes.push((x, *point_y, Cross));
                } else if up[x - min_x] {
                    changes.push((x, *point_y, LUR));
                } else if down[x - min_x] {
                    changes.push((x, *point_y, RDL));
                } else {
                    changes.push((x, *point_y, LR));
                }
            }

            for (new_x, new_y) in &points {
                if new_y == point_y {
                    // handled above
                    continue;
                } else if new_y > point_y {
                    for y in (point_y + 1)..*new_y {
                        changes.push((*new_x, y, UD));
                    }
                } else {
                    for y in *new_y..*point_y {
                        changes.push((*new_x, y, UD));
                    }
                }
            }

            for (x, y) in &points {
                changes.push((*x, *y, Blocked));
            }
            match &ans {
                Some(old) => {
                    if old.changes.len() > changes.len() {
                        ans = Some(ChangeSet { changes: changes })
                    }
                }
                None => ans = Some(ChangeSet { changes: changes }),
            }
        }

        ans
    }

    pub fn stst_mut(&mut self, points: &Points) -> bool {
        match self.stst(points) {
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
    fn stst() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 0), (1, 1), (2, 2)]
        }));
        println!("{}", maze);

        let mut maze = Maze::new(5, 5);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0), (4, 4)]
        }));
        println!("{}", maze);
    }
}
