use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug, Copy, Clone)]
enum State {
    Active,
    Inactive,
}
impl State {
    fn to_char(self) -> char {
        match self {
            State::Active => '#',
            State::Inactive => '.',
        }
    }
}
impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => State::Inactive,
            '#' => State::Active,
            _ => panic!("Invalid char {}", c),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct PocketDimensionVec {
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

trait PocketDimension {
    fn active_cube_count(&self) -> usize;
    fn get_state_at(&self, pos: &(isize, isize, isize)) -> State;
    fn is_active(&self, pos: &(isize, isize, isize)) -> bool;
    fn set_state_at(&mut self, pos: &(isize, isize, isize), state: State);
    fn safe_neighbors_of(&self, pos: &(isize, isize, isize)) -> Vec<(isize, isize, isize)>;
    fn active_neighbor_count_of(&self, pos: &(isize, isize, isize)) -> usize;
}
const OFFSETS: [(isize, isize, isize); 26] = [
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
];

trait ExecutableCycle {
    fn execute_cycle(self) -> Self;
    fn trim(self) -> Self;
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
    fn depth(&self) -> usize {
        self.states.len()
    }

    fn height(&self) -> usize {
        if self.states.is_empty() {
            0
        } else {
            self.states[0].len()
        }
    }

    fn width(&self) -> usize {
        if self.states.is_empty() || self.states[0].is_empty() {
            0
        } else {
            self.states[0][0].len()
        }
    }

    fn safe_offsets(&self, pos: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        PocketDimensionVec::range_checked(self, pos);
        OFFSETS
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
impl PocketDimension for PocketDimensionVec {
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
}

#[derive(PartialEq, Debug, Clone)]
struct PocketDimensionMap {
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

impl PocketDimension for PocketDimensionMap {
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
        OFFSETS
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

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE1_INITIAL: &str = ".#.
..#
###";

    const EXAMPLE1_STEP_1: &str = "z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.";

    const EXAMPLE1_STEP_2: &str = "z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....";

    const EXAMPLE1_STEP_3: &str = "z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
.......";

    #[test]
    fn part1_ex_vec_depth() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.depth(), 1);
    }
    #[test]
    fn part1_ex_vec_height() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.height(), 3);
    }
    #[test]
    fn part1_ex_vec_width() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.width(), 3);
    }

    #[test]
    fn part1_ex_vec_active_cube_count() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.active_cube_count(), 5);
    }

    #[test]
    #[should_panic]
    fn part1_ex_vec_set_state_at_panics_if_out_of_bounds_1() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines("#"));
        pd.set_state_at(&(1, 0, 0), State::Active);
    }
    #[test]
    #[should_panic]
    fn part1_ex_vec_set_state_at_panics_if_out_of_bounds_2() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines("#"));
        pd.set_state_at(&(0, 1, 0), State::Active);
    }
    #[test]
    #[should_panic]
    fn part1_ex_vec_set_state_at_panics_if_out_of_bounds_3() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines("#"));
        pd.set_state_at(&(0, 0, 1), State::Active);
    }
    #[test]
    #[should_panic]
    fn part1_ex_vec_set_state_at_panics_if_out_of_bounds_4() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines("#"));
        pd.set_state_at(&(-1, 0, 0), State::Active);
    }
    #[test]
    #[should_panic]
    fn part1_ex_vec_set_state_at_panics_if_out_of_bounds_5() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines("#"));
        pd.set_state_at(&(0, -1, 0), State::Active);
    }
    #[test]
    #[should_panic]
    fn part1_ex_vec_set_state_at_panics_if_out_of_bounds_6() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines("#"));
        pd.set_state_at(&(0, 0, -1), State::Active);
    }

    #[test]
    fn part1_ex_vec_set_state_at() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        pd.set_state_at(&(0, 0, 2), State::Active);
        assert_eq!(pd.active_cube_count(), 6);
    }
    #[test]
    fn part1_ex_vec_set_inactive() {
        let mut pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        pd.set_state_at(&(0, 1, 2), State::Inactive);
        assert_eq!(pd.active_cube_count(), 4);
    }
    #[test]
    fn part1_ex_vec_get_state_at() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
    fn part1_ex_vec_is_active_at() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
    fn part1_ex_vec_active_neighbor_count_of() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
    fn part1_ex_vec_safe_offsets() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(
            pd.safe_offsets(&(0, 1, 1)),
            [
                (0, -1, -1),
                (0, -1, 0),
                (0, -1, 1),
                (0, 0, -1),
                (0, 0, 1),
                (0, 1, -1),
                (0, 1, 0),
                (0, 1, 1)
            ]
        );
        assert_eq!(
            pd.safe_offsets(&(0, 0, 0)),
            [(0, 0, 1), (0, 1, 0), (0, 1, 1)]
        );
        assert_eq!(
            pd.safe_offsets(&(0, 2, 2)),
            [(0, -1, -1), (0, -1, 0), (0, 0, -1)]
        );
    }

    #[test]
    fn part1_ex_vec_safe_neighbors() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(
            pd.safe_neighbors_of(&(0, 1, 1)),
            [
                (0, 0, 0),
                (0, 0, 1),
                (0, 0, 2),
                (0, 1, 0),
                (0, 1, 2),
                (0, 2, 0),
                (0, 2, 1),
                (0, 2, 2)
            ]
        );
        assert_eq!(
            pd.safe_neighbors_of(&(0, 0, 0)),
            [(0, 0, 1), (0, 1, 0), (0, 1, 1)]
        );
        assert_eq!(
            pd.safe_neighbors_of(&(0, 2, 2)),
            [(0, 1, 1), (0, 1, 2), (0, 2, 1)]
        );
    }

    #[test]
    fn part1_ex_vec_execute_cycle() {
        let initial = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_1);
    }

    #[test]
    fn part1_ex_vec_execute_2_cycles() {
        let initial = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_2);
    }

    #[test]
    fn part1_ex_vec_execute_3_cycles() {
        let initial = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle().execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_3);
    }

    #[test]
    fn part1_ex_vec_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
    fn part1_vec_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionVec::from(&read_file_to_lines("input.txt"));
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
    fn part1_ex_vec_vec_deque() {
        let pd = PocketDimensionVec::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(format!("{}", pd), format!("z=0\n{}", EXAMPLE1_INITIAL));
    }

    #[test]
    fn part1_input_vec_vec_deque() {
        let pd = PocketDimensionVec::from(&read_file_to_lines("input.txt"));
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

    ////////////////////

    #[test]
    fn part1_ex_map_active_cube_count() {
        let pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.active_cube_count(), 5);
    }

    #[test]
    fn part1_ex_map_set_state_at() {
        let mut pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        pd.set_state_at(&(0, 0, 2), State::Active);
        assert_eq!(pd.active_cube_count(), 6);
    }
    #[test]
    fn part1_ex_map_set_inactive() {
        let mut pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        pd.set_state_at(&(0, 1, 2), State::Inactive);
        assert_eq!(pd.active_cube_count(), 4);
    }
    #[test]
    fn part1_ex_map_get_state_at() {
        let pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
        let pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
        let pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
        let pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
        let initial = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_1);
    }

    #[test]
    fn part1_ex_map_execute_2_cycles() {
        let initial = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_2);
    }

    #[test]
    fn part1_ex_map_execute_3_cycles() {
        let initial = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle().execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_3);
    }

    #[test]
    fn part1_ex_map_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
        let initial = PocketDimensionMap::from(&read_file_to_lines("input.txt"));
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
        let pd = PocketDimensionMap::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(format!("{}", pd), format!("z=0\n{}", EXAMPLE1_INITIAL));
    }

    #[test]
    fn part1_input_map_vec_deque() {
        let pd = PocketDimensionMap::from(&read_file_to_lines("input.txt"));
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
