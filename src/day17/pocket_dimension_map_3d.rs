use super::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct PocketDimensionMap3D {
    states: HashMap<Coord3, State>,
}

impl From<&Vec<String>> for PocketDimensionMap3D {
    fn from(grid: &Vec<String>) -> Self {
        let states = grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| ((0isize, y as isize, x as isize), State::from(c)))
                    .collect::<Vec<(Coord3, State)>>()
            })
            .flatten()
            .collect();
        PocketDimensionMap3D { states }
    }
}

impl Display for PocketDimensionMap3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = String::new();
        for z in self.z_range() {
            s.push_str(&*format!("z={}\n", z));
            for y in self.y_range() {
                for x in self.x_range() {
                    s.push(self.states.get(&(z, y, x)).unwrap().to_char());
                }
                s.push('\n');
            }
            s.push('\n');
        }
        write!(f, "{}", s.trim())
    }
}

impl PocketDimension<Coord3> for PocketDimensionMap3D {
    fn active_cube_count(&self) -> usize {
        self.states
            .iter()
            .filter(|(_, state)| state == &&State::Active)
            .count()
    }

    fn get_state_at(&self, pos: &Coord3) -> State {
        *self.states.get(&pos).unwrap_or(&State::Inactive)
    }

    fn is_active(&self, pos: &Coord3) -> bool {
        self.states.get(&pos) == Some(&State::Active)
    }

    fn set_state_at(&mut self, pos: &Coord3, state: State) {
        self.states.insert(*pos, state);
    }

    fn safe_neighbors_of(&self, pos: &Coord3) -> Vec<Coord3> {
        PocketDimensionMap3D::offsets()
            .iter()
            .map(|&offset| (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2))
            .collect()
    }

    fn active_neighbor_count_of(&self, pos: &Coord3) -> usize {
        self.safe_neighbors_of(pos)
            .iter()
            .filter(|&&pos| self.is_active(&pos))
            .count()
    }

    fn offsets() -> Vec<Coord3> {
        crate::day17::offsets_3d()
    }
}
impl ExecutableCycle for PocketDimensionMap3D {
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

        PocketDimensionMap3D { states }.trim()
    }

    fn trim(mut self) -> Self {
        self.z_range()
            .for_each(|z| self.remove_values_if_all_are_inactive_where(|pos| pos.0 == z));
        self.y_range()
            .for_each(|y| self.remove_values_if_all_are_inactive_where(|pos| pos.1 == y));
        self.x_range()
            .for_each(|x| self.remove_values_if_all_are_inactive_where(|pos| pos.2 == x));
        self
    }
}

impl PocketDimensionMap3D {
    fn z_range(&self) -> RangeInclusive<isize> {
        self.range(|(z, _, _)| *z)
    }

    fn y_range(&self) -> RangeInclusive<isize> {
        self.range(|(_, y, _)| *y)
    }

    fn x_range(&self) -> RangeInclusive<isize> {
        self.range(|(_, _, x)| *x)
    }

    fn range<F>(&self, select_from: F) -> RangeInclusive<isize>
    where
        F: Fn(&Coord3) -> isize,
    {
        let values = self.states.iter().map(|(pos, _)| select_from(pos));
        let min = values.clone().min().unwrap_or(0);
        let max = values.max().unwrap_or(0);
        min..=max
    }

    fn remove_values_if_all_are_inactive_where<F>(&mut self, position_filter: F)
    where
        F: Fn(&Coord3) -> bool,
    {
        let filtered: Vec<_> = self
            .states
            .iter()
            .filter(|(pos, _)| position_filter(pos))
            .map(|(pos, state)| (*pos, *state))
            .collect();
        if filtered.iter().all(|(_, state)| state == &State::Inactive) {
            filtered.iter().for_each(|(pos, _)| {
                self.states.remove(pos);
            });
        }
    }

    fn expand(&self) -> HashMap<Coord3, State> {
        let z_range = (self.z_range().start() - 1)..=(self.z_range().end() + 1);
        let y_range = (self.y_range().start() - 1)..=(self.y_range().end() + 1);
        let x_range = (self.x_range().start() - 1)..=(self.x_range().end() + 1);

        let mut states = self.states.clone();
        z_range.for_each(|z| {
            y_range.clone().for_each(|y| {
                x_range.clone().for_each(|x| {
                    states.entry((z, y, x)).or_insert(State::Inactive);
                });
            });
        });
        states
    }
}
