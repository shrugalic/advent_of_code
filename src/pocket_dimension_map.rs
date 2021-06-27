use super::*;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct PocketDimensionMap {
    states: HashMap<(isize, isize, isize), State>,
}

impl From<&Vec<String>> for PocketDimensionMap {
    fn from(grid: &Vec<String>) -> Self {
        let states = grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| ((0isize, y as isize, x as isize), State::from(c)))
                    .collect::<Vec<((isize, isize, isize), State)>>()
            })
            .flatten()
            .collect();
        PocketDimensionMap { states }
    }
}

impl Display for PocketDimensionMap {
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

impl PocketDimension<(isize, isize, isize)> for PocketDimensionMap {
    fn active_cube_count(&self) -> usize {
        self.states
            .iter()
            .filter(|(_, state)| state == &&State::Active)
            .count()
    }

    fn get_state_at(&self, pos: &(isize, isize, isize)) -> State {
        *self.states.get(&pos).unwrap_or(&State::Inactive)
    }

    fn is_active(&self, pos: &(isize, isize, isize)) -> bool {
        self.states.get(&pos) == Some(&State::Active)
    }

    fn set_state_at(&mut self, pos: &(isize, isize, isize), state: State) {
        self.states.insert(*pos, state);
    }

    fn safe_neighbors_of(&self, pos: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        self.offsets()
            .iter()
            .map(|&offset| (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2))
            .collect()
    }

    fn active_neighbor_count_of(&self, pos: &(isize, isize, isize)) -> usize {
        self.safe_neighbors_of(pos)
            .iter()
            .filter(|&&pos| self.is_active(&pos))
            .count()
    }

    fn offsets(&self) -> Vec<(isize, isize, isize)> {
        OFFSETS_3D.to_vec()
    }
}
impl ExecutableCycle for PocketDimensionMap {
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

        PocketDimensionMap { states }.trim()
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

impl PocketDimensionMap {
    fn z_range(&self) -> RangeInclusive<isize> {
        let zs = self.states.iter().map(|((z, _, _), _)| z);
        let z_min = *zs.clone().min().unwrap_or(&0);
        let z_max = *zs.max().unwrap_or(&-1);
        z_min..=z_max
    }

    fn y_range(&self) -> RangeInclusive<isize> {
        let ys = self.states.iter().map(|((_, y, _), _)| y);
        let y_min = *ys.clone().min().unwrap_or(&0);
        let y_max = *ys.max().unwrap_or(&-1);
        y_min..=y_max
    }

    fn x_range(&self) -> RangeInclusive<isize> {
        let xs = self.states.iter().map(|((_, _, x), _)| x);
        let x_min = *xs.clone().min().unwrap_or(&0);
        let x_max = *xs.max().unwrap_or(&-1);
        x_min..=x_max
    }

    fn remove_values_if_all_are_inactive_where<F>(&mut self, position_filter: F)
    where
        F: Fn(&(isize, isize, isize)) -> bool,
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

    fn expand(&self) -> HashMap<(isize, isize, isize), State> {
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
