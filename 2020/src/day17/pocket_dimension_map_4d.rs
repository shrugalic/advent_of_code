use super::*;

type Coord4 = (isize, isize, isize, isize);

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct PocketDimensionMap4D {
    states: HashMap<Coord4, State>,
}

impl From<&Vec<String>> for PocketDimensionMap4D {
    fn from(grid: &Vec<String>) -> Self {
        let (w, z) = (0isize, 0isize);
        let states = grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| ((w, z, y as isize, x as isize), State::from(c)))
                    .collect::<Vec<(Coord4, State)>>()
            })
            .flatten()
            .collect();
        PocketDimensionMap4D { states }
    }
}

impl Display for PocketDimensionMap4D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = String::new();
        for w in self.w_range() {
            for z in self.z_range() {
                s.push_str(&*format!("z={}, w={}\n", z, w));
                for y in self.y_range() {
                    for x in self.x_range() {
                        s.push(
                            self.states
                                .get(&(w, z, y, x))
                                .expect(&*format!("Non-existent pos ({},{},{},{})\nw: {:?}\nz: {:?}\ny: {:?}\nx: {:?}", w, z, y, x, self.w_range(), self.z_range(), self.y_range(), self.x_range()))
                                .to_char(),
                        );
                    }
                    s.push('\n');
                }
                s.push('\n');
            }
        }
        write!(f, "{}", s.trim())
    }
}

impl PocketDimension<Coord4> for PocketDimensionMap4D {
    fn active_cube_count(&self) -> usize {
        self.states
            .iter()
            .filter(|(_, state)| state == &&State::Active)
            .count()
    }

    fn get_state_at(&self, pos: &Coord4) -> State {
        *self.states.get(&pos).unwrap_or(&State::Inactive)
    }

    fn is_active(&self, pos: &Coord4) -> bool {
        self.states.get(&pos) == Some(&State::Active)
    }

    fn set_state_at(&mut self, pos: &Coord4, state: State) {
        self.states.insert(*pos, state);
    }

    fn safe_neighbors_of(&self, pos: &Coord4) -> Vec<Coord4> {
        PocketDimensionMap4D::offsets()
            .iter()
            .map(|&offset| {
                (
                    pos.0 + offset.0,
                    pos.1 + offset.1,
                    pos.2 + offset.2,
                    pos.3 + offset.3,
                )
            })
            .collect()
    }

    fn active_neighbor_count_of(&self, pos: &Coord4) -> usize {
        self.safe_neighbors_of(pos)
            .iter()
            .filter(|&&pos| self.is_active(&pos))
            .count()
    }

    fn offsets() -> Vec<Coord4> {
        let range: RangeInclusive<isize> = -1..=1;
        let mut offsets = vec![];
        for w in range.clone() {
            for z in range.clone() {
                for y in range.clone() {
                    for x in range.clone() {
                        if w != 0 || z != 0 || y != 0 || x != 0 {
                            offsets.push((w, z, y, x))
                        }
                    }
                }
            }
        }
        offsets
    }
}
impl ExecutableCycle for PocketDimensionMap4D {
    fn execute_cycle(self) -> Self {
        let states = self
            .expand()
            .iter()
            .map(|(new_pos, curr_state)| {
                let active_neighbor_count = self.active_neighbor_count_of(new_pos);
                let new_state = match (&curr_state, active_neighbor_count) {
                    (State::Active, 2..=3) | (State::Inactive, 3) => State::Active,
                    (_, _) => State::Inactive,
                };
                (*new_pos, new_state)
            })
            .collect::<HashMap<_, _>>();

        PocketDimensionMap4D { states }.trim()
    }

    fn trim(mut self) -> Self {
        // Trim both ends of the w-range
        loop {
            let w = *self.w_range().start();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.0 == w) {
                break;
            }
        }
        loop {
            let w = *self.w_range().end();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.0 == w) {
                break;
            }
        }
        // Trim both ends of the z-range
        loop {
            let z = *self.z_range().start();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.1 == z) {
                break;
            }
        }
        loop {
            let z = *self.z_range().end();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.1 == z) {
                break;
            }
        }
        // Trim both ends of the y-range
        loop {
            let y = *self.y_range().start();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.2 == y) {
                break;
            }
        }
        loop {
            let y = *self.y_range().end();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.2 == y) {
                break;
            }
        }
        // Trim both ends of the x-range
        loop {
            let x = *self.x_range().start();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.3 == x) {
                break;
            }
        }
        loop {
            let x = *self.x_range().end();
            if 0 == self.removed_values_if_all_are_inactive_where(|pos| pos.3 == x) {
                break;
            }
        }

        self
    }
}

impl PocketDimensionMap4D {
    fn w_range(&self) -> RangeInclusive<isize> {
        self.range(|(w, _, _, _)| *w)
    }

    fn z_range(&self) -> RangeInclusive<isize> {
        self.range(|(_, z, _, _)| *z)
    }

    fn y_range(&self) -> RangeInclusive<isize> {
        self.range(|(_, _, y, _)| *y)
    }

    fn x_range(&self) -> RangeInclusive<isize> {
        self.range(|(_, _, _, x)| *x)
    }

    fn range<F>(&self, value_filter: F) -> RangeInclusive<isize>
    where
        F: Fn(&Coord4) -> isize,
    {
        let values = self.states.iter().map(|(pos, _)| value_filter(pos));
        let min = values.clone().min().unwrap_or(0);
        let max = values.max().unwrap_or(0);
        min..=max
    }

    fn removed_values_if_all_are_inactive_where<F>(&mut self, position_filter: F) -> usize
    where
        F: Fn(&Coord4) -> bool,
    {
        let mut rm_count = 0;
        let filtered: Vec<_> = self
            .states
            .iter()
            .filter(|(pos, _)| position_filter(pos))
            .map(|(pos, state)| (*pos, *state))
            .collect();
        if filtered.iter().all(|(_, state)| state == &State::Inactive) {
            filtered.iter().for_each(|(pos, _)| {
                rm_count += 1;
                self.states.remove(pos);
            });
        }
        rm_count
    }

    fn expand(&self) -> HashMap<Coord4, State> {
        let w_range = (self.w_range().start() - 1)..=(self.w_range().end() + 1);
        let z_range = (self.z_range().start() - 1)..=(self.z_range().end() + 1);
        let y_range = (self.y_range().start() - 1)..=(self.y_range().end() + 1);
        let x_range = (self.x_range().start() - 1)..=(self.x_range().end() + 1);

        let mut states = self.states.clone();
        w_range.for_each(|w| {
            z_range.clone().for_each(|z| {
                y_range.clone().for_each(|y| {
                    x_range.clone().for_each(|x| {
                        states.entry((w, z, y, x)).or_insert(State::Inactive);
                    });
                });
            });
        });
        states
    }
}
