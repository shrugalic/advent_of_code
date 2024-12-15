use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use crate::vec_tile_grid::VecTileGrid;
use std::cmp::PartialEq;
use std::collections::HashSet;
use Direction::{Down, Left, Right, Up};

const INPUT: &str = include_str!("../../2024/input/day15.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let (grid, movements) = parse(input);
    sum_of_gps_coordinates_of_moved_boxes(grid, movements)
}

fn solve_part2(input: &str) -> usize {
    let (orig, movements) = parse(input);

    // Make the grid double-wide
    let chars = vec![vec!['.'; 2 * orig.width()]; orig.height()];
    let mut grid: VecTileGrid<char> = VecTileGrid { chars };
    for (y, line) in orig.chars.into_iter().enumerate() {
        for (x_orig, c) in line.into_iter().enumerate() {
            let x_new = 2 * x_orig;
            let (l, r) = match c {
                '#' => ('#', '#'),
                'O' => ('[', ']'),
                '.' => ('.', '.'),
                '@' => ('@', '.'),
                _ => unreachable!(),
            };
            grid.chars[y][x_new] = l;
            grid.chars[y][x_new + 1] = r;
        }
    }

    sum_of_gps_coordinates_of_moved_boxes(grid, movements)
}

fn sum_of_gps_coordinates_of_moved_boxes(
    mut grid: VecTileGrid<char>,
    directions: Vec<Direction>,
) -> usize {
    let mut robot = *grid.positions(|c| c == &'@').first().unwrap();
    let mut forces: HashSet<Vec2D> = HashSet::new();
    let boxes = ['[', ']', 'O'];

    'following_directions: for dir in directions {
        // In part 2, a single robot (or box-half) can push a double-wide box above or below
        // This is modeled by adding an extra force pushing the other half of the box
        let determine_extra_force = |curr_pos: &Vec2D, next_pos: &Vec2D| -> Option<Vec2D> {
            let curr_tile = grid.char_at(curr_pos).unwrap();
            let next_tile = grid.char_at(next_pos).unwrap();
            if curr_tile != next_tile && (dir == Up || dir == Down) {
                if next_tile == &'[' {
                    return Some(curr_pos.right_neighbor());
                } else if next_tile == &']' {
                    return Some(curr_pos.left_neighbor());
                }
            }
            None
        };
        let offset = dir.to_offset();
        let next_pos = robot + offset;
        let mut moves = vec![(robot, next_pos)];

        forces.insert(robot);
        if let Some(extra_force) = determine_extra_force(&robot, &next_pos) {
            forces.insert(extra_force);
        }
        while !forces.is_empty() {
            let targets: Vec<_> = forces
                .drain()
                .map(|src| src + offset)
                .map(|target| (target, grid.char_at(&target).unwrap()))
                .collect();
            if targets.iter().any(|(_, c)| c == &&'#') {
                // The movement is blocked by a wall
                continue 'following_directions;
            }
            for (curr_pos, _) in targets.into_iter().filter(|(_, c)| boxes.contains(c)) {
                // There are boxes in the way, which act as new push forces
                let next_pos = curr_pos + offset;
                moves.push((curr_pos, next_pos));

                forces.insert(curr_pos);
                if let Some(extra_force) = determine_extra_force(&curr_pos, &next_pos) {
                    forces.insert(extra_force);
                }
            }
        }
        // Movement was possible
        robot += offset;
        while let Some((from_pos, to_pos)) = moves.pop() {
            let from_tile = *grid.char_at(&from_pos).unwrap();
            *grid.mut_char_at(&from_pos).unwrap() = '.';
            *grid.mut_char_at(&to_pos).unwrap() = from_tile;
        }
    }
    let box_positions = grid.positions(|&c| c == '[' || c == 'O');
    box_positions
        .into_iter()
        .map(|pos| (pos.x + 100 * pos.y) as usize)
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(&self) -> Vec2D {
        match self {
            Up => Vec2D::up(),
            Down => Vec2D::down(),
            Left => Vec2D::left(),
            Right => Vec2D::right(),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> (VecTileGrid<char>, Vec<Direction>) {
    let (grid, movements) = input.trim().split_once("\n\n").unwrap();
    let grid = VecTileGrid::from(grid);
    let movements = movements
        .lines()
        .flat_map(str::chars)
        .map(Direction::from)
        .collect();
    (grid, movements)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_SMALL_EXAMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const LARGE_EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const PART2_SMALL_EXAMPLE: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn test_part1_smaller_example() {
        assert_eq!(2_028, solve_part1(PART1_SMALL_EXAMPLE));
    }

    #[test]
    fn test_part1_larger_example() {
        assert_eq!(10_092, solve_part1(LARGE_EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_463_715, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_small_example() {
        /*
        ##############
        ##...[].##..## 5,1 -> 100 + 5
        ##...@.[]...## 7,2 -> 200 + 7
        ##....[]....## 6,3 -> 300 + 6
        ##..........## -> total 618
        ##..........##
        ##############
        */
        assert_eq!(618, solve_part2(PART2_SMALL_EXAMPLE));
    }

    #[test]
    fn test_part2_large_example() {
        assert_eq!(9_021, solve_part2(LARGE_EXAMPLE));
    }

    #[test]
    fn test_part2_triple_boxes() {
        let custom_example = "\
########
#......#
#OOO...#
#.OO@..#
#..O...#
#......#
########

<>v<<v<^^
";
        /* This will turn into the following interesting case:
        ################
        ##............##
        ##[][][]......##
        ##.[][].......##
        ##..[]........##
        ##...@........##
        ################

        ^^
        */
        assert_eq!(1_024, solve_part2(custom_example));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1_481_392, solve_part2(INPUT));
    }
}
