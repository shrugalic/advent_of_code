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
        let mut next_states = HashMap::new();
        self.states.iter().for_each(|(old_pos, _old_state)| {
            self.safe_neighbors_of(old_pos).iter().for_each(|new_pos| {
                let curr_state = self.get_state_at(new_pos);
                let active_neighbor_count = self.active_neighbor_count_of(new_pos);
                let new_state = match (&curr_state, active_neighbor_count) {
                    (State::Active, 2..=3) | (State::Inactive, 3) => State::Active,
                    (_, _) => State::Inactive,
                };
                next_states.insert(*new_pos, new_state);
            });
        });

        PocketDimensionMap {
            states: next_states,
        }
        .trim()
    }

    fn trim(mut self) -> Self {
        // trim z:
        for z in self.z_range() {
            let pos_filter = |pos: &(isize, isize, isize)| pos.0 == z;
            self.remove_inactive_values(pos_filter)
        }
        // trim y
        for y in self.y_range() {
            let pos_filter = |pos: &(isize, isize, isize)| pos.1 == y;
            self.remove_inactive_values(pos_filter)
        }
        // trim x:
        for x in self.x_range() {
            let pos_filter = |pos: &(isize, isize, isize)| pos.2 == x;
            self.remove_inactive_values(pos_filter)
        }
        self
    }
}

impl PocketDimensionMap {
    fn z_range(&self) -> RangeInclusive<isize> {
        let z_min = *self
            .states
            .iter()
            .map(|((z, _, _), _)| z)
            .min()
            .unwrap_or(&0);
        let z_max = *self
            .states
            .iter()
            .map(|((z, _, _), _)| z)
            .max()
            .unwrap_or(&-1);
        z_min..=z_max
    }

    fn y_range(&self) -> RangeInclusive<isize> {
        let y_min = *self
            .states
            .iter()
            .map(|((_, y, _), _)| y)
            .min()
            .unwrap_or(&0);
        let y_max = *self
            .states
            .iter()
            .map(|((_, y, _), _)| y)
            .max()
            .unwrap_or(&-1);
        y_min..=y_max
    }

    fn x_range(&self) -> RangeInclusive<isize> {
        let x_min = *self
            .states
            .iter()
            .map(|((_, _, x), _)| x)
            .min()
            .unwrap_or(&0);
        let x_max = *self
            .states
            .iter()
            .map(|((_, _, x), _)| x)
            .max()
            .unwrap_or(&-1);
        x_min..=x_max
    }

    fn remove_inactive_values<F>(&mut self, filter: F)
    where
        F: Fn(&(isize, isize, isize)) -> bool,
    {
        let states_in_this_plane: Vec<_> = self
            .states
            .iter()
            .filter(|(pos, _)| filter(pos))
            .map(|(pos, state)| (*pos, *state))
            .collect();
        if states_in_this_plane
            .iter()
            .all(|(_, state)| state == &State::Inactive)
        {
            states_in_this_plane.iter().for_each(|(pos, _)| {
                self.states.remove(pos);
            });
        }
    }
}
