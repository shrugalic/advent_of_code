use super::pocket_dimension_map::PocketDimensionMap;
use super::pocket_dimension_vec::PocketDimensionVec;

#[cfg(test)]
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
