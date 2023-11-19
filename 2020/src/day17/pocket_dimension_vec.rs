use super::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct PocketDimensionVec {
    // 3-dimensional state [z][y][x]
    states: Vec<Vec<Vec<State>>>,
}

impl From<&Vec<String>> for PocketDimensionVec {
    fn from(grid: &Vec<String>) -> Self {
        let states = vec![grid
            .iter()
            .map(|row| row.chars().map(State::from).collect::<Vec<State>>())
            .collect::<Vec<Vec<State>>>()];
        PocketDimensionVec { states }
    }
}

impl Display for PocketDimensionVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = String::new();
        let z_offset = self.states.len() / 2;
        for (z, rows) in self.states.iter().enumerate() {
            s.push_str(&*format!("z={}\n", z as isize - z_offset as isize));
            for row in rows.iter() {
                let row_string: String = row.iter().map(|s| s.to_char()).collect();
                s.push_str(&*row_string.to_string());
                s.push('\n');
            }
            s.push('\n');
        }
        write!(f, "{}", s.trim())
    }
}

impl PocketDimension<(isize, isize, isize)> for PocketDimensionVec {
    fn active_cube_count(&self) -> usize {
        self.states
            .iter()
            .map(|rows| {
                rows.iter()
                    .map(|row| row.iter().filter(|&s| s == &State::Active).count())
                    .sum::<usize>()
            })
            .sum()
    }

    fn get_state_at(&self, pos: &(isize, isize, isize)) -> State {
        let pos = PocketDimensionVec::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2]
    }

    fn is_active(&self, pos: &(isize, isize, isize)) -> bool {
        let pos = PocketDimensionVec::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2] == State::Active
    }

    fn set_state_at(&mut self, pos: &(isize, isize, isize), state: State) {
        let pos = PocketDimensionVec::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2] = state;
    }

    fn safe_neighbors_of(&self, pos: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        self.safe_offsets(pos)
            .iter()
            .map(|offset| (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2))
            .collect()
    }

    fn active_neighbor_count_of(&self, pos: &(isize, isize, isize)) -> usize {
        self.safe_neighbors_of(pos)
            .iter()
            .filter(|&neighbor| self.is_active(neighbor))
            .count()
    }

    fn offsets() -> Vec<(isize, isize, isize)> {
        crate::day17::offsets_3d()
    }
}

impl ExecutableCycle for PocketDimensionVec {
    fn execute_cycle(self) -> Self {
        // allocate a new pocket dimension that extends 1 cube further out on all 6 sides
        let mut prev: PocketDimensionVec = PocketDimensionVec::create_new_of_size(
            self.depth() + 2,
            self.height() + 2,
            self.width() + 2,
        );
        // set its states to a copy of the previous pocket dimension;
        // note this copy is offset such that it is in the middle
        for z in 0..self.depth() {
            let z = z as isize;
            for y in 0..self.height() {
                let y = y as isize;
                for x in 0..self.width() {
                    let x = x as isize;
                    let state = self.get_state_at(&(z, y, x));
                    let new_pos = (z + 1, y + 1, x + 1);
                    prev.set_state_at(&new_pos, state);
                }
            }
        }
        let mut next: PocketDimensionVec = PocketDimensionVec::create_new_of_size(
            self.depth() + 2,
            self.height() + 2,
            self.width() + 2,
        );
        // go through all these locations and determine the new state
        for z in 0..prev.depth() {
            let z = z as isize;
            for y in 0..prev.height() {
                let y = y as isize;
                for x in 0..prev.width() {
                    let x = x as isize;
                    let pos = &(z, y, x);
                    let prev_state = prev.get_state_at(pos);
                    let active_neighbor_count = prev.active_neighbor_count_of(pos);
                    let new_state = match (&prev_state, active_neighbor_count) {
                        (State::Active, 2..=3) | (State::Inactive, 3) => State::Active,
                        (_, _) => State::Inactive,
                    };
                    next.set_state_at(pos, new_state);
                }
            }
        }
        // println!("----prev:----\n{}\n", prev);
        // println!("----next:----\n{}\n", next);

        next.trim()
    }

    fn trim(mut self) -> Self {
        // trim z:
        let mut z = 0;
        while z < self.depth() {
            let no_active_cubes = (0..self.height()).into_iter().all(|y| {
                (0..self.width())
                    .into_iter()
                    .all(|x| !self.is_active(&(z as isize, y as isize, x as isize)))
            });
            if no_active_cubes {
                // println!("For z = {} no cubes are active, removing them", z);
                self.states.remove(z);
            } else {
                z += 1;
            }
        }
        // trim y
        let mut y = 0;
        while y < self.height() {
            let all_inactive = (0..self.depth()).into_iter().all(|z| {
                (0..self.width())
                    .into_iter()
                    .all(|x| !self.is_active(&(z as isize, y as isize, x as isize)))
            });
            if all_inactive {
                // println!("For y = {} no cubes are active, removing them", y);
                (0..self.depth()).into_iter().for_each(|z| {
                    self.states[z].remove(y);
                });
            } else {
                y += 1;
            }
        }
        // trim x:
        let mut x = 0;
        while x < self.width() {
            let all_inactive = (0..self.height()).into_iter().all(|y| {
                (0..self.depth())
                    .into_iter()
                    .all(|z| !self.is_active(&(z as isize, y as isize, x as isize)))
            });
            if all_inactive {
                // println!("For x = {} no cubes are active, removing them", x);
                (0..self.depth()).into_iter().for_each(|z| {
                    (0..self.height()).into_iter().for_each(|y| {
                        self.states[z][y].remove(x);
                    })
                });
            } else {
                x += 1;
            }
        }
        // return it
        self
    }
}

impl PocketDimensionVec {
    pub(crate) fn depth(&self) -> usize {
        self.states.len()
    }

    pub(crate) fn height(&self) -> usize {
        if self.states.is_empty() {
            0
        } else {
            self.states[0].len()
        }
    }

    pub(crate) fn width(&self) -> usize {
        if self.states.is_empty() || self.states[0].is_empty() {
            0
        } else {
            self.states[0][0].len()
        }
    }

    pub(crate) fn safe_offsets(&self, pos: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        PocketDimensionVec::range_checked(self, pos);
        PocketDimensionVec::offsets()
            .iter()
            .filter(|&offset| {
                PocketDimensionVec::is_safe_pos(
                    self,
                    &(pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2),
                )
            })
            .cloned()
            .collect()
    }

    fn create_new_of_size(depth: usize, height: usize, width: usize) -> Self {
        PocketDimensionVec {
            states: vec![vec![vec![State::Inactive; width]; height]; depth],
        }
    }

    fn range_checked(&self, pos: &(isize, isize, isize)) -> (usize, usize, usize) {
        let check_range = |num, lo, hi| {
            if !(lo..hi as isize).contains(&num) {
                panic!("{} not in range {}..{}", num, lo, hi);
            }
        };
        check_range(pos.0, 0, self.depth());
        check_range(pos.1, 0, self.height());
        check_range(pos.2, 0, self.width());
        (pos.0 as usize, pos.1 as usize, pos.2 as usize)
    }
    // returns true if the given pos is safely within all bounds
    fn is_safe_pos(&self, pos: &(isize, isize, isize)) -> bool {
        let is_in_range = |num, lo, hi| (lo..hi as isize).contains(&num);
        is_in_range(pos.0, 0, self.depth())
            && is_in_range(pos.1, 0, self.height())
            && is_in_range(pos.2, 0, self.width())
    }
}
