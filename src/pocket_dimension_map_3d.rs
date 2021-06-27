use super::*;

type Coord3 = (isize, isize, isize);

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
        self.offsets()
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

    fn offsets(&self) -> Vec<Coord3> {
        OFFSETS_3D.to_vec()
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

mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn part1_ex_map_active_cube_count() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        assert_eq!(pd.active_cube_count(), 5);
    }

    #[test]
    fn part1_ex_map_set_state_at() {
        let mut pd = PocketDimensionMap3D::from(&example_1_initial());
        pd.set_state_at(&(0, 0, 2), State::Active);
        assert_eq!(pd.active_cube_count(), 6);
    }

    #[test]
    fn part1_ex_map_set_inactive() {
        let mut pd = PocketDimensionMap3D::from(&example_1_initial());
        pd.set_state_at(&(0, 1, 2), State::Inactive);
        assert_eq!(pd.active_cube_count(), 4);
    }

    #[test]
    fn part1_ex_map_get_state_at() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        // left column
        assert_eq!(pd.get_state_at(&(0, 0, 0)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 1, 0)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 2, 0)), State::Active);
        // middle column
        assert_eq!(pd.get_state_at(&(0, 0, 1)), State::Active);
        assert_eq!(pd.get_state_at(&(0, 1, 1)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 2, 1)), State::Active);
        // right column
        assert_eq!(pd.get_state_at(&(0, 0, 2)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 1, 2)), State::Active);
        assert_eq!(pd.get_state_at(&(0, 2, 2)), State::Active);
    }

    #[test]
    fn part1_ex_map_is_active_at() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        // left column
        assert!(pd.is_active(&(0, 2, 0)));
        // middle column
        assert!(pd.is_active(&(0, 0, 1)));
        assert!(pd.is_active(&(0, 2, 1)));
        // right column
        assert!(pd.is_active(&(0, 1, 2)));
        assert!(pd.is_active(&(0, 2, 2)));
    }

    #[test]
    fn part1_ex_map_active_neighbor_count_of() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        // left column
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 0)), 1);
        assert_eq!(pd.active_neighbor_count_of(&(0, 1, 0)), 3);
        assert_eq!(pd.active_neighbor_count_of(&(0, 2, 0)), 1);
        // middle column
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 1)), 1);
        assert_eq!(pd.active_neighbor_count_of(&(0, 1, 1)), 5);
        assert_eq!(pd.active_neighbor_count_of(&(0, 2, 1)), 3);
        // right column
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 2)), 2);
        assert_eq!(pd.active_neighbor_count_of(&(0, 1, 2)), 3);
        assert_eq!(pd.active_neighbor_count_of(&(0, 2, 2)), 2);
    }

    #[test]
    fn part1_ex_map_neighbors() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        assert_eq!(
            pd.safe_neighbors_of(&(0, 0, 0)),
            [
                (-1, -1, -1),
                (-1, -1, 0),
                (-1, -1, 1),
                (-1, 0, -1),
                (-1, 0, 0),
                (-1, 0, 1),
                (-1, 1, -1),
                (-1, 1, 0),
                (-1, 1, 1),
                (0, -1, -1),
                (0, -1, 0),
                (0, -1, 1),
                (0, 0, -1),
                // (0, 0, 0),
                (0, 0, 1),
                (0, 1, -1),
                (0, 1, 0),
                (0, 1, 1),
                (1, -1, -1),
                (1, -1, 0),
                (1, -1, 1),
                (1, 0, -1),
                (1, 0, 0),
                (1, 0, 1),
                (1, 1, -1),
                (1, 1, 0),
                (1, 1, 1),
            ]
        );
    }

    #[test]
    fn part1_ex_map_execute_cycle() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_1);
    }

    #[test]
    fn part1_ex_map_execute_2_cycles() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial.execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_2);
    }

    #[test]
    fn part1_ex_map_execute_3_cycles() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial.execute_cycle().execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_3);
    }

    #[test]
    fn part1_ex_map_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle();
        assert_eq!(next.active_cube_count(), 112);
    }

    #[test]
    fn part1_map_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionMap3D::from(&input());
        let next = initial
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle();
        assert_eq!(next.active_cube_count(), 291);
    }

    #[test]
    fn part1_ex_map_vec_deque() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        assert_eq!(format!("{}", pd), format!("z=0\n{}", EXAMPLE1_INITIAL));
    }

    #[test]
    fn part1_input_map_vec_deque() {
        let pd = PocketDimensionMap3D::from(&input());
        assert_eq!(
            format!("{}", pd),
            format!(
                "z=0\n{}",
                "##.#....
...#...#
.#.#.##.
..#.#...
.###....
.##.#...
#.##..##
#.####.."
            )
        );
    }
}
