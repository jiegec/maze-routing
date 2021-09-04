//! Single Trunk Steiner Tree

use super::*;

#[wasm_bindgen]
impl Maze {
    /// Single Trunk Steiner Tree Algorithm
    pub fn stst(&self, points: &Points) -> Option<ChangeSet> {
        use CellState::*;
        let points = points.get();
        if points.len() == 0 {
            return Some(ChangeSet { changes: vec![] });
        }
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
        'next_h: for (_point_x, point_y) in &points {
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
            if self.map[min_x][*point_y] != Empty {
                continue 'next_h;
            }
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
            if self.map[max_x][*point_y] != Empty {
                continue 'next_h;
            }
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
                if self.map[x][*point_y] != Empty
                    && (on[x - min_x] || up[x - min_x] || down[x - min_x])
                {
                    // non-LR
                    continue 'next_h;
                }
                if on[x - min_x] {
                    // handled below
                } else if up[x - min_x] && down[x - min_x] {
                    changes.push((x, *point_y, Cross));
                } else if up[x - min_x] {
                    changes.push((x, *point_y, LUR));
                } else if down[x - min_x] {
                    changes.push((x, *point_y, RDL));
                } else {
                    if self.map[x][*point_y] == Empty {
                        changes.push((x, *point_y, LR));
                    } else if self.map[x][*point_y] == UD {
                        changes.push((x, *point_y, Cross));
                    } else {
                        continue 'next_h;
                    }
                }
            }

            for (new_x, new_y) in &points {
                if new_y == point_y {
                    // handled below
                    continue;
                } else if new_y > point_y {
                    for y in (point_y + 1)..*new_y {
                        if self.map[*new_x][y] == Empty {
                            changes.push((*new_x, y, UD));
                        } else if self.map[*new_x][y] == LR {
                            changes.push((*new_x, y, Cross));
                        } else {
                            continue 'next_h;
                        }
                    }
                } else {
                    for y in *new_y..*point_y {
                        if self.map[*new_x][y] == Empty {
                            changes.push((*new_x, y, UD));
                        } else if self.map[*new_x][y] == LR {
                            changes.push((*new_x, y, Cross));
                        } else {
                            continue 'next_h;
                        }
                    }
                }
            }

            for (x, y) in &points {
                changes.push((*x, *y, Blocked));
            }

            // remove duplicate assignments to one cell
            changes.sort_by_key(|(x, y, _state)| (*x, *y));
            changes.reverse();
            changes.dedup_by_key(|(x, y, _state)| (*x, *y));

            match &ans {
                Some(old) => {
                    if old.changes.len() > changes.len() {
                        ans = Some(ChangeSet { changes: changes })
                    }
                }
                None => ans = Some(ChangeSet { changes: changes }),
            }
        }

        // try vertical
        'next_v: for (point_x, _point_y) in &points {
            // construct a trunk from this point
            let mut changes: Vec<(usize, usize, CellState)> = vec![];
            let mut left = vec![false; max_y - min_y + 1];
            let mut right = vec![false; max_y - min_y + 1];
            let mut on = vec![false; max_y - min_y + 1];
            for (new_x, new_y) in &points {
                if new_x == point_x {
                    on[new_y - min_y] = true;
                } else if new_x > point_x {
                    right[new_y - min_y] = true;
                } else {
                    left[new_y - min_y] = true;
                }
            }

            // y = min_y
            if self.map[*point_x][min_y] != Empty {
                continue 'next_v;
            }
            if on[0] {
                // handled below
            } else if right[0] && left[0] {
                changes.push((*point_x, min_y, LUR));
            } else if right[0] {
                changes.push((*point_x, min_y, RU));
            } else if left[0] {
                changes.push((*point_x, min_y, LU));
            } else {
                unreachable!();
            }

            // y = max_y
            if self.map[*point_x][max_y] != Empty {
                continue 'next_v;
            }
            if on[max_y - min_y] {
                // handled below
            } else if right[max_y - min_y] && left[max_y - min_y] {
                changes.push((*point_x, max_y, RDL));
            } else if right[max_y - min_y] {
                changes.push((*point_x, max_y, RD));
            } else if left[max_y - min_y] {
                changes.push((*point_x, max_y, LD));
            } else {
                unreachable!();
            }

            for y in (min_y + 1)..(max_y) {
                if self.map[*point_x][y] != Empty
                    && (on[y - min_y] || right[y - min_y] || left[y - min_y])
                {
                    // non-UD
                    continue 'next_v;
                }
                if on[y - min_y] {
                    // handled below
                } else if right[y - min_y] && left[y - min_y] {
                    changes.push((*point_x, y, Cross));
                } else if right[y - min_y] {
                    changes.push((*point_x, y, URD));
                } else if left[y - min_y] {
                    changes.push((*point_x, y, DLU));
                } else {
                    if self.map[*point_x][y] == Empty {
                        changes.push((*point_x, y, UD));
                    } else if self.map[*point_x][y] == LR {
                        changes.push((*point_x, y, Cross));
                    } else {
                        continue 'next_v;
                    }
                }
            }

            for (new_x, new_y) in &points {
                if new_x == point_x {
                    // handled below
                    continue;
                } else if new_x > point_x {
                    for x in (point_x + 1)..*new_x {
                        if self.map[x][*new_y] == Empty {
                            changes.push((x, *new_y, LR));
                        } else if self.map[x][*new_y] == UD {
                            changes.push((x, *new_y, Cross));
                        } else {
                            continue 'next_v;
                        }
                    }
                } else {
                    for x in *new_x..*point_x {
                        if self.map[x][*new_y] == Empty {
                            changes.push((x, *new_y, LR));
                        } else if self.map[x][*new_y] == UD {
                            changes.push((x, *new_y, Cross));
                        } else {
                            continue 'next_v;
                        }
                    }
                }
            }

            for (x, y) in &points {
                changes.push((*x, *y, Blocked));
            }

            // remove duplicate assignments to one cell
            changes.sort_by_key(|(x, y, _state)| (*x, *y));
            changes.reverse();
            changes.dedup_by_key(|(x, y, _state)| (*x, *y));

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

    /// Single Trunk Steiner Tree Algorithm
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
    use quickcheck::*;

    #[test]
    fn stst() {
        let mut maze = Maze::new(3, 3);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 0), (1, 1), (2, 2)]
        }));
        println!("{}", maze);
        assert!(!maze.stst_mut(&Points {
            points: vec![(2, 0), (0, 2)]
        }));

        let mut maze = Maze::new(5, 5);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0), (4, 4)]
        }));
        println!("{}", maze);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 3), (3, 3)]
        }));
        println!("{}", maze);

        let mut maze = Maze::new(5, 5);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 2), (1, 1), (2, 0), (2, 2), (3, 4), (4, 0)]
        }));
        println!("{}", maze);
        assert!(maze.stst_mut(&Points {
            points: vec![(0, 3), (4, 3)]
        }));
        println!("{}", maze);
    }

    #[test]
    fn stst_regression_1() {
        let mut maze = Maze::new(2, 1);
        assert!(maze.stst_mut(&Points { points: vec![] }));
        println!("{}", maze);
    }

    #[test]
    fn stst_regression_2() {
        let mut maze = Maze::new(2, 6);
        assert!(maze.stst_mut(&Points {
            points: vec![(1, 1), (0, 4), (0, 2), (1, 4)]
        }));
        println!("{}", maze);
    }

    quickcheck! {
        fn qc_stst(m: usize, n: usize, points: Vec<(usize, usize)>) -> bool {
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
            maze.stst_mut(&Points {
                points
            }) && maze.verify()
        }
    }
}
