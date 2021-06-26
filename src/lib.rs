use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Clone)]
enum State {
    Active,
    Inactive,
}
impl State {
    fn to_char(&self) -> char {
        match self {
            State::Active => '#',
            State::Inactive => '.',
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct PocketDimensionVecDeque {
    // 3-dimensional state [z][y][x]
    states: VecDeque<VecDeque<VecDeque<State>>>,
}
impl From<&Vec<String>> for PocketDimensionVecDeque {
    fn from(grid: &Vec<String>) -> Self {
        let center_plane_rows = grid
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => State::Inactive,
                        '#' => State::Active,
                        _ => panic!("Invalid char {}", c),
                    })
                    .collect::<VecDeque<State>>()
            })
            .collect::<VecDeque<VecDeque<State>>>();
        let states = VecDeque::from(vec![center_plane_rows]);
        PocketDimensionVecDeque { states }
    }
}
impl Display for PocketDimensionVecDeque {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = String::new();
        let offset = self.states.len() / 2;
        for (z, rows) in self.states.iter().enumerate() {
            s.push_str(&*format!("z={}\n", z as isize - offset as isize));
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
    fn depth(&self) -> usize; // z
    fn height(&self) -> usize; // y
    fn width(&self) -> usize; // x
    fn active_cube_count(&self) -> usize;
    fn set_active(&mut self, pos: (isize, isize, isize));
    fn set_inactive(&mut self, pos: (isize, isize, isize));
    fn get_state_at(&self, pos: (isize, isize, isize)) -> State;
    fn is_active(&self, pos: (isize, isize, isize)) -> bool;
    fn set_state_at(&mut self, pos: (isize, isize, isize), state: State);
    fn safe_offsets(&self, pos: (isize, isize, isize)) -> Vec<(isize, isize, isize)>;
    fn safe_neighbors_of(&self, pos: (isize, isize, isize)) -> Vec<(isize, isize, isize)>;
    fn active_neighbor_count_of(&self, pos: (isize, isize, isize)) -> usize;
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
    fn create_new_of_size(depth: usize, height: usize, width: usize) -> Self;
}
impl ExecutableCycle for PocketDimensionVecDeque {
    fn execute_cycle(self) -> Self {
        // allocate a new pocket dimension that extends 1 cube further out on all 6 sides
        let mut prev: PocketDimensionVecDeque = ExecutableCycle::create_new_of_size(
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
                    let state = self.get_state_at((z, y, x));
                    let new_pos = (z + 1, y + 1, x + 1);
                    prev.set_state_at(new_pos, state);
                }
            }
        }
        let mut next: PocketDimensionVecDeque = ExecutableCycle::create_new_of_size(
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
                    let pos = (z, y, x);
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

        // trim z:
        let mut z = 0;
        while z < next.depth() {
            let no_active_cubes = (0..next.height()).into_iter().all(|y| {
                (0..next.width()).into_iter().all(|x| {
                    next.get_state_at((z as isize, y as isize, x as isize)) == State::Inactive
                })
            });
            if no_active_cubes {
                // println!("For z = {} no cubes are active, removing them", z);
                next.states.remove(z);
            } else {
                z += 1;
            }
        }
        // trim y
        let mut y = 0;
        while y < next.height() {
            let all_inactive = (0..next.depth()).into_iter().all(|z| {
                (0..next.width()).into_iter().all(|x| {
                    next.get_state_at((z as isize, y as isize, x as isize)) == State::Inactive
                })
            });
            if all_inactive {
                // println!("For y = {} no cubes are active, removing them", y);
                (0..next.depth()).into_iter().for_each(|z| {
                    next.states[z].remove(y);
                });
            } else {
                y += 1;
            }
        }
        // trim x:
        let mut x = 0;
        while x < next.width() {
            let all_inactive = (0..next.height()).into_iter().all(|y| {
                (0..next.depth()).into_iter().all(|z| {
                    next.get_state_at((z as isize, y as isize, x as isize)) == State::Inactive
                })
            });
            if all_inactive {
                // println!("For x = {} no cubes are active, removing them", x);
                (0..next.depth()).into_iter().for_each(|z| {
                    (0..next.height()).into_iter().for_each(|y| {
                        next.states[z][y].remove(x);
                    })
                });
            } else {
                x += 1;
            }
        }
        // return it
        next
    }

    fn create_new_of_size(depth: usize, height: usize, width: usize) -> Self {
        PocketDimensionVecDeque {
            states: VecDeque::from(vec![
                VecDeque::from(vec![
                    VecDeque::from(vec![
                        State::Inactive;
                        width
                    ]);
                    height
                ]);
                depth
            ]),
        }
    }
}

impl PocketDimensionVecDeque {
    fn range_checked(
        pd: &dyn PocketDimension,
        pos: (isize, isize, isize),
    ) -> (usize, usize, usize) {
        let check_range = |num, lo, hi| {
            if !(lo..hi as isize).contains(&num) {
                panic!("{} not in range {}..{}", num, lo, hi);
            }
        };
        check_range(pos.0, 0, pd.depth());
        check_range(pos.1, 0, pd.height());
        check_range(pos.2, 0, pd.width());
        (pos.0 as usize, pos.1 as usize, pos.2 as usize)
    }
    // returns true if the given pos is safely within all bounds
    fn is_safe_pos(pd: &dyn PocketDimension, pos: (isize, isize, isize)) -> bool {
        let is_in_range = |num, lo, hi| (lo..hi as isize).contains(&num);
        is_in_range(pos.0, 0, pd.depth())
            && is_in_range(pos.1, 0, pd.height())
            && is_in_range(pos.2, 0, pd.width())
    }
}
impl PocketDimension for PocketDimensionVecDeque {
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

    fn set_active(&mut self, pos: (isize, isize, isize)) {
        let pos = PocketDimensionVecDeque::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2] = State::Active;
    }

    fn set_inactive(&mut self, pos: (isize, isize, isize)) {
        let pos = PocketDimensionVecDeque::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2] = State::Inactive;
    }

    fn get_state_at(&self, pos: (isize, isize, isize)) -> State {
        let pos = PocketDimensionVecDeque::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2].clone()
    }

    fn is_active(&self, pos: (isize, isize, isize)) -> bool {
        let pos = PocketDimensionVecDeque::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2] == State::Active
    }

    fn set_state_at(&mut self, pos: (isize, isize, isize), state: State) {
        let pos = PocketDimensionVecDeque::range_checked(self, pos);
        self.states[pos.0][pos.1][pos.2] = state;
    }

    fn safe_offsets(&self, pos: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        PocketDimensionVecDeque::range_checked(self, pos);
        OFFSETS
            .iter()
            .filter(|&offset| {
                PocketDimensionVecDeque::is_safe_pos(
                    self,
                    (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2),
                )
            })
            .cloned()
            .collect()
    }
    fn safe_neighbors_of(&self, pos: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        self.safe_offsets(pos)
            .iter()
            .map(|offset| (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2))
            .collect()
    }

    fn active_neighbor_count_of(&self, pos: (isize, isize, isize)) -> usize {
        self.safe_neighbors_of(pos)
            .iter()
            .filter(|&neighbor| self.is_active(*neighbor))
            .count()
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

    #[test]
    fn part1_ex_pocket_dimension_depth() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.depth(), 1);
    }
    #[test]
    fn part1_ex_pocket_dimension_height() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.height(), 3);
    }
    #[test]
    fn part1_ex_pocket_dimension_width() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.width(), 3);
    }

    #[test]
    fn part1_ex_pocket_dimension_active_cube_count() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(pd.active_cube_count(), 5);
    }

    #[test]
    #[should_panic]
    fn part1_ex_pocket_dimension_set_active_panics_if_out_of_bounds_1() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines("#"));
        pd.set_active((1, 0, 0));
    }
    #[test]
    #[should_panic]
    fn part1_ex_pocket_dimension_set_active_panics_if_out_of_bounds_2() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines("#"));
        pd.set_active((0, 1, 0));
    }
    #[test]
    #[should_panic]
    fn part1_ex_pocket_dimension_set_active_panics_if_out_of_bounds_3() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines("#"));
        pd.set_active((0, 0, 1));
    }
    #[test]
    #[should_panic]
    fn part1_ex_pocket_dimension_set_active_panics_if_out_of_bounds_4() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines("#"));
        pd.set_active((-1, 0, 0));
    }
    #[test]
    #[should_panic]
    fn part1_ex_pocket_dimension_set_active_panics_if_out_of_bounds_5() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines("#"));
        pd.set_active((0, -1, 0));
    }
    #[test]
    #[should_panic]
    fn part1_ex_pocket_dimension_set_active_panics_if_out_of_bounds_6() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines("#"));
        pd.set_active((0, 0, -1));
    }

    #[test]
    fn part1_ex_pocket_dimension_set_active() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        pd.set_active((0, 0, 2));
        assert_eq!(pd.active_cube_count(), 6);
    }
    #[test]
    fn part1_ex_pocket_dimension_set_inactive() {
        let mut pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        pd.set_inactive((0, 1, 2));
        assert_eq!(pd.active_cube_count(), 4);
    }
    #[test]
    fn part1_ex_pocket_dimension_get_state_at() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        // left column
        assert_eq!(pd.get_state_at((0, 0, 0)), State::Inactive);
        assert_eq!(pd.get_state_at((0, 1, 0)), State::Inactive);
        assert_eq!(pd.get_state_at((0, 2, 0)), State::Active);
        // middle column
        assert_eq!(pd.get_state_at((0, 0, 1)), State::Active);
        assert_eq!(pd.get_state_at((0, 1, 1)), State::Inactive);
        assert_eq!(pd.get_state_at((0, 2, 1)), State::Active);
        // right column
        assert_eq!(pd.get_state_at((0, 0, 2)), State::Inactive);
        assert_eq!(pd.get_state_at((0, 1, 2)), State::Active);
        assert_eq!(pd.get_state_at((0, 2, 2)), State::Active);
    }
    #[test]
    fn part1_ex_pocket_dimension_is_active_at() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        // left column
        assert!(pd.is_active((0, 2, 0)));
        // middle column
        assert!(pd.is_active((0, 0, 1)));
        assert!(pd.is_active((0, 2, 1)));
        // right column
        assert!(pd.is_active((0, 1, 2)));
        assert!(pd.is_active((0, 2, 2)));
    }
    #[test]
    fn part1_ex_pocket_dimension_active_neighbor_count_of() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        // left column
        assert_eq!(pd.active_neighbor_count_of((0, 0, 0)), 1);
        assert_eq!(pd.active_neighbor_count_of((0, 1, 0)), 3);
        assert_eq!(pd.active_neighbor_count_of((0, 2, 0)), 1);
        // middle column
        assert_eq!(pd.active_neighbor_count_of((0, 0, 1)), 1);
        assert_eq!(pd.active_neighbor_count_of((0, 1, 1)), 5);
        assert_eq!(pd.active_neighbor_count_of((0, 2, 1)), 3);
        // right column
        assert_eq!(pd.active_neighbor_count_of((0, 0, 2)), 2);
        assert_eq!(pd.active_neighbor_count_of((0, 1, 2)), 3);
        assert_eq!(pd.active_neighbor_count_of((0, 2, 2)), 2);
    }

    #[test]
    fn part1_ex_pocket_dimension_safe_offsets() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(
            pd.safe_offsets((0, 1, 1)),
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
            pd.safe_offsets((0, 0, 0)),
            [(0, 0, 1), (0, 1, 0), (0, 1, 1)]
        );
        assert_eq!(
            pd.safe_offsets((0, 2, 2)),
            [(0, -1, -1), (0, -1, 0), (0, 0, -1)]
        );
    }

    #[test]
    fn part1_ex_pocket_dimension_safe_neighbors() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(
            pd.safe_neighbors_of((0, 1, 1)),
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
            pd.safe_neighbors_of((0, 0, 0)),
            [(0, 0, 1), (0, 1, 0), (0, 1, 1)]
        );
        assert_eq!(
            pd.safe_neighbors_of((0, 2, 2)),
            [(0, 1, 1), (0, 1, 2), (0, 2, 1)]
        );
    }

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
    fn part1_ex_pocket_dimension_execute_cycle() {
        let initial = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_1);
    }

    #[test]
    fn part1_ex_pocket_dimension_execute_2_cycles() {
        let initial = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_2);
    }

    #[test]
    fn part1_ex_pocket_dimension_execute_3_cycles() {
        let initial = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        let next = initial.execute_cycle().execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_3);
    }

    #[test]
    fn part1_ex_pocket_dimension_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
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
    fn part1_pocket_dimension_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionVecDeque::from(&read_file_to_lines("input.txt"));
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
    fn part1_ex_pocket_dimension_vec_deque() {
        let pd = PocketDimensionVecDeque::from(&read_str_to_lines(EXAMPLE1_INITIAL));
        assert_eq!(format!("{}", pd), format!("z=0\n{}", EXAMPLE1_INITIAL));
    }

    #[test]
    fn part1_input_pocket_dimension_vec_deque() {
        let pd = PocketDimensionVecDeque::from(&read_file_to_lines("input.txt"));
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
