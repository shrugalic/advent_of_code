mod pocket_dimension_map_3d;
pub(crate) mod pocket_dimension_map_4d;
pub(crate) mod pocket_dimension_vec;

use crate::line_reader::read_file_to_lines;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum State {
    Active,
    Inactive,
}

pub(crate) trait PocketDimension<POS> {
    fn active_cube_count(&self) -> usize;
    fn get_state_at(&self, pos: &POS) -> State;
    fn is_active(&self, pos: &POS) -> bool;
    fn set_state_at(&mut self, pos: &POS, state: State);
    fn safe_neighbors_of(&self, pos: &POS) -> Vec<POS>;
    fn active_neighbor_count_of(&self, pos: &POS) -> usize;
    fn offsets() -> Vec<POS>;
}

pub(crate) trait ExecutableCycle {
    fn execute_cycle(self) -> Self;
    fn trim(self) -> Self;
}

type Coord3 = (isize, isize, isize);

fn offsets_3d() -> Vec<Coord3> {
    let range: RangeInclusive<isize> = -1..=1;
    let mut offsets = vec![];
    for z in range.clone() {
        for y in range.clone() {
            for x in range.clone() {
                if z != 0 || y != 0 || x != 0 {
                    offsets.push((z, y, x))
                }
            }
        }
    }
    offsets
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

pub(crate) fn day17_input() -> Vec<String> {
    read_file_to_lines("input/day17.txt")
}

#[cfg(test)]
mod tests {
    use crate::line_reader::read_str_to_lines;

    use super::*;
    use crate::day17::pocket_dimension_map_3d::*;
    use crate::day17::pocket_dimension_map_4d::*;
    use crate::day17::pocket_dimension_vec::*;
    use crate::day17::{ExecutableCycle, PocketDimension, State};

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

    fn example_1_initial() -> Vec<String> {
        read_str_to_lines(EXAMPLE1_INITIAL)
    }

    const EXAMPLE2_INITIAL: &str = "z=0, w=0
.#.
..#
###";

    const EXAMPLE2_STEP_1: &str = "z=-1, w=-1
#..
..#
.#.

z=0, w=-1
#..
..#
.#.

z=1, w=-1
#..
..#
.#.

z=-1, w=0
#..
..#
.#.

z=0, w=0
#.#
.##
.#.

z=1, w=0
#..
..#
.#.

z=-1, w=1
#..
..#
.#.

z=0, w=1
#..
..#
.#.

z=1, w=1
#..
..#
.#.";

    const EXAMPLE2_STEP_2: &str = "z=-2, w=-2
.....
.....
..#..
.....
.....

z=-1, w=-2
.....
.....
.....
.....
.....

z=0, w=-2
###..
##.##
#...#
.#..#
.###.

z=1, w=-2
.....
.....
.....
.....
.....

z=2, w=-2
.....
.....
..#..
.....
.....

z=-2, w=-1
.....
.....
.....
.....
.....

z=-1, w=-1
.....
.....
.....
.....
.....

z=0, w=-1
.....
.....
.....
.....
.....

z=1, w=-1
.....
.....
.....
.....
.....

z=2, w=-1
.....
.....
.....
.....
.....

z=-2, w=0
###..
##.##
#...#
.#..#
.###.

z=-1, w=0
.....
.....
.....
.....
.....

z=0, w=0
.....
.....
.....
.....
.....

z=1, w=0
.....
.....
.....
.....
.....

z=2, w=0
###..
##.##
#...#
.#..#
.###.

z=-2, w=1
.....
.....
.....
.....
.....

z=-1, w=1
.....
.....
.....
.....
.....

z=0, w=1
.....
.....
.....
.....
.....

z=1, w=1
.....
.....
.....
.....
.....

z=2, w=1
.....
.....
.....
.....
.....

z=-2, w=2
.....
.....
..#..
.....
.....

z=-1, w=2
.....
.....
.....
.....
.....

z=0, w=2
###..
##.##
#...#
.#..#
.###.

z=1, w=2
.....
.....
.....
.....
.....

z=2, w=2
.....
.....
..#..
.....
.....";

    // -------------------------- PocketDimensionVec --------------------------

    #[test]
    fn part1_ex_vec_depth() {
        let pd = PocketDimensionVec::from(&example_1_initial());
        assert_eq!(pd.depth(), 1);
    }

    #[test]
    fn part1_ex_vec_height() {
        let pd = PocketDimensionVec::from(&example_1_initial());
        assert_eq!(pd.height(), 3);
    }

    #[test]
    fn part1_ex_vec_width() {
        let pd = PocketDimensionVec::from(&example_1_initial());
        assert_eq!(pd.width(), 3);
    }

    #[test]
    fn part1_ex_vec_active_cube_count() {
        let pd = PocketDimensionVec::from(&example_1_initial());
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
        let mut pd = PocketDimensionVec::from(&example_1_initial());
        pd.set_state_at(&(0, 0, 2), State::Active);
        assert_eq!(pd.active_cube_count(), 6);
    }
    #[test]
    fn part1_ex_vec_set_inactive() {
        let mut pd = PocketDimensionVec::from(&example_1_initial());
        pd.set_state_at(&(0, 1, 2), State::Inactive);
        assert_eq!(pd.active_cube_count(), 4);
    }

    #[test]
    fn part1_ex_vec_get_state_at() {
        let pd = PocketDimensionVec::from(&example_1_initial());
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
        let pd = PocketDimensionVec::from(&example_1_initial());
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
        let pd = PocketDimensionVec::from(&example_1_initial());
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
        let pd = PocketDimensionVec::from(&example_1_initial());
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
        let pd = PocketDimensionVec::from(&example_1_initial());
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
        let initial = PocketDimensionVec::from(&example_1_initial());
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_1);
    }

    #[test]
    fn part1_ex_vec_execute_2_cycles() {
        let initial = PocketDimensionVec::from(&example_1_initial());
        let next = initial.execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_2);
    }

    #[test]
    fn part1_ex_vec_execute_3_cycles() {
        let initial = PocketDimensionVec::from(&example_1_initial());
        let next = initial.execute_cycle().execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_3);
    }

    #[test]
    fn part1_ex_vec_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionVec::from(&example_1_initial());
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
    fn part1() {
        let initial = PocketDimensionVec::from(&day17_input());
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
        let pd = PocketDimensionVec::from(&example_1_initial());
        assert_eq!(format!("{}", pd), format!("z=0\n{}", EXAMPLE1_INITIAL));
    }

    #[test]
    fn part1_input_vec_vec_deque() {
        let pd = PocketDimensionVec::from(&day17_input());
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

    // -------------------------- PocketDimensionMap3D --------------------------

    #[test]
    fn part1_ex_map_3d_active_cube_count() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        assert_eq!(pd.active_cube_count(), 5);
    }

    #[test]
    fn part1_ex_map_3d_set_state_at() {
        let mut pd = PocketDimensionMap3D::from(&example_1_initial());
        pd.set_state_at(&(0, 0, 2), State::Active);
        assert_eq!(pd.active_cube_count(), 6);
    }

    #[test]
    fn part1_ex_map_3d_set_inactive() {
        let mut pd = PocketDimensionMap3D::from(&example_1_initial());
        pd.set_state_at(&(0, 1, 2), State::Inactive);
        assert_eq!(pd.active_cube_count(), 4);
    }

    #[test]
    fn part1_ex_map_3d_get_state_at() {
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
    fn part1_ex_map_3d_is_active_at() {
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
    fn part1_ex_map_3d_active_neighbor_count_of() {
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
    fn part1_ex_map_3d_execute_cycle() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_1);
    }

    #[test]
    fn part1_ex_map_3d_execute_2_cycles() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial.execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_2);
    }

    #[test]
    fn part1_ex_map_3d_execute_3_cycles() {
        let initial = PocketDimensionMap3D::from(&example_1_initial());
        let next = initial.execute_cycle().execute_cycle().execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE1_STEP_3);
    }

    #[test]
    fn part1_ex_map_3d_active_cube_count_after_6_cycles() {
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
    fn part1_map_3d_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionMap3D::from(&day17_input());
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
    fn part1_ex_map_3d_display_works() {
        let pd = PocketDimensionMap3D::from(&example_1_initial());
        assert_eq!(format!("{}", pd), format!("z=0\n{}", EXAMPLE1_INITIAL));
    }

    #[test]
    fn part1_input_map_3d_display_works() {
        let pd = PocketDimensionMap3D::from(&day17_input());
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

    // -------------------------- PocketDimensionMap4D --------------------------

    #[test]
    fn part1_ex_map_4d_active_cube_count() {
        let pd = PocketDimensionMap4D::from(&example_1_initial());
        assert_eq!(pd.active_cube_count(), 5);
    }

    #[test]
    fn part1_ex_map_4d_set_state_at() {
        let mut pd = PocketDimensionMap4D::from(&example_1_initial());
        pd.set_state_at(&(0, 0, 0, 2), State::Active);
        assert_eq!(pd.active_cube_count(), 6);
    }

    #[test]
    fn part1_ex_map_4d_set_inactive() {
        let mut pd = PocketDimensionMap4D::from(&example_1_initial());
        pd.set_state_at(&(0, 0, 1, 2), State::Inactive);
        assert_eq!(pd.active_cube_count(), 4);
    }

    #[test]
    fn part1_ex_map_4d_get_state_at() {
        let pd = PocketDimensionMap4D::from(&example_1_initial());
        // left column
        assert_eq!(pd.get_state_at(&(0, 0, 0, 0)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 0, 1, 0)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 0, 2, 0)), State::Active);
        // middle column
        assert_eq!(pd.get_state_at(&(0, 0, 0, 1)), State::Active);
        assert_eq!(pd.get_state_at(&(0, 0, 1, 1)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 0, 2, 1)), State::Active);
        // right column
        assert_eq!(pd.get_state_at(&(0, 0, 0, 2)), State::Inactive);
        assert_eq!(pd.get_state_at(&(0, 0, 1, 2)), State::Active);
        assert_eq!(pd.get_state_at(&(0, 0, 2, 2)), State::Active);
    }

    #[test]
    fn part1_ex_map_4d_is_active_at() {
        let pd = PocketDimensionMap4D::from(&example_1_initial());
        // left column
        assert!(pd.is_active(&(0, 0, 2, 0)));
        // middle column
        assert!(pd.is_active(&(0, 0, 0, 1)));
        assert!(pd.is_active(&(0, 0, 2, 1)));
        // right column
        assert!(pd.is_active(&(0, 0, 1, 2)));
        assert!(pd.is_active(&(0, 0, 2, 2)));
    }

    #[test]
    fn part1_ex_map_4d_active_neighbor_count_of() {
        let pd = PocketDimensionMap4D::from(&example_1_initial());
        // left column
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 0, 0)), 1);
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 1, 0)), 3);
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 2, 0)), 1);
        // middle column
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 0, 1)), 1);
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 1, 1)), 5);
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 2, 1)), 3);
        // right column
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 0, 2)), 2);
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 1, 2)), 3);
        assert_eq!(pd.active_neighbor_count_of(&(0, 0, 2, 2)), 2);
    }

    #[test]
    fn part1_ex_map_4d_initial_state_format() {
        let initial = PocketDimensionMap4D::from(&example_1_initial());
        assert_eq!(format!("{}", initial), EXAMPLE2_INITIAL);
    }

    #[test]
    fn part1_ex_map_4d_execute_1_cycle() {
        let initial = PocketDimensionMap4D::from(&example_1_initial());
        let next = initial.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE2_STEP_1);
    }

    #[test]
    fn part1_ex_map_4d_execute_2_cycles() {
        let initial = PocketDimensionMap4D::from(&example_1_initial());
        let next = initial.execute_cycle();
        let next = next.execute_cycle();
        assert_eq!(format!("{}", next), EXAMPLE2_STEP_2);
    }

    #[test]
    fn part1_ex_map_4d_active_cube_count_after_6_cycles() {
        let initial = PocketDimensionMap4D::from(&example_1_initial());
        let next = initial
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle();
        assert_eq!(next.active_cube_count(), 848);
    }

    #[test]
    fn part2() {
        let initial = PocketDimensionMap4D::from(&day17_input());
        let next = initial
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle()
            .execute_cycle();
        assert_eq!(next.active_cube_count(), 1524);
    }
}
