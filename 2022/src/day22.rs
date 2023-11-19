use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use Facing::*;
use Instruction::*;
use Tile::*;

const INPUT: &str = include_str!("../input/day22.txt");

pub(crate) fn day22_part1() -> usize {
    let map = Map::from(INPUT);
    map.follow_path_on_plane()
}

pub(crate) fn day22_part2() -> usize {
    let map = Map::from(INPUT);
    map.follow_path_on_cube()
}

impl Map {
    fn follow_path_on_plane(&self) -> usize {
        let x_ranges_by_y = self.x_ranges_by_y();
        let y_ranges_by_x = self.y_ranges_by_x();

        // Start in left-most open tile in first row
        let mut y = 1;
        let mut x = *self
            .board
            .iter()
            .filter(|&((_, iy), tile)| iy == &y && matches!(tile, &Open))
            .map(|((x, _y), _tile)| x)
            .min()
            .unwrap();
        let mut facing = Right;
        for instruction in &self.path_description {
            // println!("At ({x}, {y}): {instruction}");
            match instruction {
                GoForward(step_count) => {
                    match facing {
                        Right => {
                            let x_range = x_ranges_by_y[y as usize].clone().unwrap();
                            // println!("x_range at {y} = {x_range:?}");
                            if let Some(next_x) = x_range
                                .cycle()
                                .skip_while(|ix| ix != &x)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|x| self.is_open(*x, y))
                                .last()
                            {
                                x = next_x;
                            }
                        }
                        Down => {
                            let y_range = y_ranges_by_x[x as usize].clone().unwrap();
                            // println!("y_range at {x} = {y_range:?}");
                            if let Some(next_y) = y_range
                                .cycle()
                                .skip_while(|iy| iy != &y)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|y| self.is_open(x, *y))
                                .last()
                            {
                                y = next_y;
                            }
                        }
                        Left => {
                            let x_range = x_ranges_by_y[y as usize].clone().unwrap();
                            // println!("x_range at {y} = {x_range:?}");
                            if let Some(next_x) = x_range
                                .rev()
                                .cycle()
                                .skip_while(|ix| ix != &x)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|x| self.is_open(*x, y))
                                .last()
                            {
                                x = next_x;
                            }
                        }
                        Up => {
                            let y_range = y_ranges_by_x[x as usize].clone().unwrap();
                            // println!("y_range at {x} = {y_range:?}");
                            if let Some(next_y) = y_range
                                .rev()
                                .cycle()
                                .skip_while(|iy| iy != &y)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|y| self.is_open(x, *y))
                                .last()
                            {
                                y = next_y;
                            }
                        }
                    }
                }
                TurnLeft | TurnRight => facing.turn_according_to(instruction),
            }
        }
        (1000 * y + 4 * x + facing as isize) as usize
    }

    fn follow_path_on_cube(&self) -> usize {
        let transition_areas = self.transition_area_by_facing();

        // Start in left-most open tile in first row
        let mut y = 1;
        let mut x = *self
            .board
            .iter()
            .filter(|&((_, iy), tile)| iy == &y && matches!(tile, &Open))
            .map(|((x, _y), _tile)| x)
            .min()
            .unwrap();
        let mut facing = Right;
        for instruction in &self.path_description {
            // println!("At ({x}, {y}): {instruction}");
            match instruction {
                GoForward(step_count) => {
                    let mut steps_left = *step_count;
                    while steps_left > 0 {
                        steps_left -= 1;
                        let (dx, dy) = match facing {
                            Right => (1, 0),
                            Down => (0, 1),
                            Left => (-1, 0),
                            Up => (0, -1),
                        };
                        if self.exists(x + dx, y + dy) {
                            if self.is_open(x + dx, y + dy) {
                                x += dx;
                                y += dy;
                            } else {
                                // println!("stopping at wall on own square");
                                steps_left = 0;
                            }
                        } else if let Some(((nx, ny), next_facing)) =
                            transition_areas[facing as usize].get(&(x, y))
                        {
                            if self.is_open(*nx, *ny) {
                                // println!("({x}, {y}) transitioning to next square ({nx}, {ny}), steps_left {steps_left}");
                                x = *nx;
                                y = *ny;
                                facing = *next_facing;
                            } else {
                                // println!("stopping at wall on next square");
                                steps_left = 0;
                            }
                        }
                    }
                }
                TurnLeft | TurnRight => facing.turn_according_to(instruction),
            }
        }
        (1000 * y + 4 * x + facing as isize) as usize
    }

    fn transition_area_by_facing(&self) -> Vec<BTreeMap<Pos, (Pos, Facing)>> {
        let size = self.cube_size();
        // println!("cube_size = {size}");
        let first = 1;
        let second = first + size;
        let third = second + size;
        let fourth = third + size;
        let fifth = fourth + size;
        let sixth = fifth + size;
        // A transition area is one side of a square going in a particular direction,
        // and defines a mapping from a point on this edge to a point on the edge
        // of another square, and a new facing direction
        let mut transition_areas: Vec<BTreeMap<Pos, (Pos, Facing)>> = vec![];
        if size == 4 {
            // example

            // right
            let mut to_right = BTreeMap::new();
            let x = fourth - 1;
            (first..second).into_iter().for_each(|y| {
                to_right.insert((x, y), ((x + size, fourth - y), Left));
            });
            (second..third).into_iter().for_each(|y| {
                to_right.insert((x, y), ((sixth - y, third), Down));
            });
            let x = fifth - 1;
            (third..fourth).into_iter().for_each(|y| {
                to_right.insert((x, y), ((x - size, fourth - y), Left));
            });
            transition_areas.push(to_right); // 0 right, 1 down, 2 left, 3 up

            // down
            let mut down = BTreeMap::new();
            let y = third - 1;
            (first..second).into_iter().for_each(|x| {
                down.insert((x, y), ((fourth - x, y + size), Up));
            });
            (second..third).into_iter().for_each(|x| {
                down.insert((x, y), ((third, fifth - x), Right));
            });
            let y = fourth - 1;
            (third..fourth).into_iter().for_each(|x| {
                down.insert((x, y), ((fourth - x, y - size), Up));
            });
            (fourth..fifth).into_iter().for_each(|x| {
                down.insert((x, y), ((first, sixth - x), Right));
            });
            transition_areas.push(down);

            // left
            let mut to_left = BTreeMap::new();
            let x = third;
            (first..second).into_iter().for_each(|y| {
                to_left.insert((x, y), ((y + size, second), Down));
            });
            let x = first;
            (second..third).into_iter().for_each(|y| {
                to_left.insert((x, y), ((sixth - y, fourth - 1), Up));
            });
            let x = third;
            (third..fourth).into_iter().for_each(|y| {
                to_left.insert((x, y), ((fifth - y, third - 1), Up));
            });
            transition_areas.push(to_left);

            // up
            let mut up = BTreeMap::new();
            let y = second;
            (first..second).into_iter().for_each(|x| {
                up.insert((x, y), ((fourth - x, y - size), Down));
            });
            (second..third).into_iter().for_each(|x| {
                up.insert((x, y), ((third, x - size), Right));
            });
            let y = first;
            (third..fourth).into_iter().for_each(|x| {
                up.insert((x, y), ((fourth - x, y + size), Down));
            });
            let y = third;
            (fourth..fifth).into_iter().for_each(|x| {
                up.insert((x, y), ((fourth - 1, sixth - x), Left));
            });
            transition_areas.push(up);
        } else if size == 50 {
            // part 2

            // right
            let mut to_right = BTreeMap::new();
            let x = fourth - 1;
            (first..second).into_iter().for_each(|y| {
                to_right.insert((x, y), ((x - size, fourth - y), Left));
            });
            let x = third - 1;
            (second..third).into_iter().for_each(|y| {
                to_right.insert((x, y), ((y + size, second - 1), Up));
            });
            (third..fourth).into_iter().for_each(|y| {
                to_right.insert((x, y), ((x + size, fourth - y), Left));
            });
            let x = second - 1;
            (fourth..fifth).into_iter().for_each(|y| {
                to_right.insert((x, y), ((y - 2 * size, fourth - 1), Up));
            });
            transition_areas.push(to_right); // 0 right, 1 down, 2 left, 3 up

            // down
            let mut down = BTreeMap::new();
            let y = fifth - 1;
            (first..second).into_iter().for_each(|x| {
                down.insert((x, y), ((x + 2 * size, 1), Down));
            });
            let y = fourth - 1;
            (second..third).into_iter().for_each(|x| {
                down.insert((x, y), ((second - 1, x + 2 * size), Left));
            });
            let y = second - 1;
            (third..fourth).into_iter().for_each(|x| {
                down.insert((x, y), ((third - 1, x - size), Left));
            });
            transition_areas.push(down);

            // left
            let mut to_left = BTreeMap::new();
            let x = second;
            (first..second).into_iter().for_each(|y| {
                to_left.insert((x, y), ((first, fourth - y), Right));
            });
            (second..third).into_iter().for_each(|y| {
                to_left.insert((x, y), ((y - size, third), Down));
            });
            let x = first;
            (third..fourth).into_iter().for_each(|y| {
                to_left.insert((x, y), ((x + size, fourth - y), Right));
            });
            (fourth..fifth).into_iter().for_each(|y| {
                to_left.insert((x, y), ((y - 2 * size, 1), Down));
            });
            transition_areas.push(to_left);

            // up
            let mut up = BTreeMap::new();
            let y = third;
            (first..second).into_iter().for_each(|x| {
                up.insert((x, y), ((second, x + size), Right));
            });
            let y = first;
            (second..third).into_iter().for_each(|x| {
                up.insert((x, y), ((first, 2 * size + x), Right));
            });
            (third..fourth).into_iter().for_each(|x| {
                up.insert((x, y), ((x - 2 * size, fifth - 1), Up));
            });
            transition_areas.push(up);
        }
        transition_areas
    }
    fn cube_size(&self) -> isize {
        (1..=self.height)
            .into_iter()
            .map(|y| {
                (1..=self.width)
                    .into_iter()
                    .filter(move |x| self.board.contains_key(&(*x, y)))
                    .map(|x| (x, x))
                    .reduce(|(a_min, a_max), (b_min, b_max)| (a_min.min(b_min), a_max.max(b_max)))
                    .unwrap()
            })
            .map(|(min, max)| max - min + 1)
            .min()
            .unwrap()
    }
    fn y_ranges_by_x(&self) -> Vec<Option<RangeInclusive<Coord>>> {
        let y_ranges_by_x: Vec<Option<RangeInclusive<Coord>>> = (0..=self.width)
            .into_iter()
            .map(|x| {
                (1..=self.height)
                    .into_iter()
                    .filter(move |y| self.board.contains_key(&(x, *y)))
                    .map(|y| y..=y)
                    .reduce(|a, b| *a.start().min(b.start())..=*a.end().max(b.end()))
            })
            .collect();
        y_ranges_by_x
    }
    fn x_ranges_by_y(&self) -> Vec<Option<RangeInclusive<Coord>>> {
        let x_ranges_by_y: Vec<Option<RangeInclusive<Coord>>> = (0..=self.height)
            .into_iter()
            .map(|y| {
                (1..=self.width)
                    .into_iter()
                    .filter(move |x| self.board.contains_key(&(*x, y)))
                    .map(|x| x..=x)
                    .reduce(|a, b| *a.start().min(b.start())..=*a.end().max(b.end()))
            })
            .collect();
        x_ranges_by_y
    }
    fn exists(&self, x: Coord, y: Coord) -> bool {
        self.board.get(&(x, y)).is_some()
    }
    fn is_open(&self, x: Coord, y: Coord) -> bool {
        matches!(self.board.get(&(x, y)), Some(Open))
    }
}

type Coord = isize;
#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Open,
            '#' => Wall,
            _ => unreachable!("Invalid tile '{c}'"),
        }
    }
}
impl Tile {
    fn to_char(&self) -> char {
        match self {
            Open => '.',
            Wall => '#',
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Facing {
    fn turn_according_to(&mut self, instruction: &Instruction) {
        *self = match instruction {
            GoForward(_) => *self,
            TurnLeft => match self {
                Right => Up,
                Down => Right,
                Left => Down,
                Up => Left,
            },
            TurnRight => match self {
                Right => Down,
                Down => Left,
                Left => Up,
                Up => Right,
            },
        };
    }
}
#[derive(Debug)]
enum Instruction {
    GoForward(usize),
    TurnLeft,
    TurnRight,
}
impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s.trim() {
            "L" => TurnLeft,
            "R" => TurnRight,
            s => GoForward(
                s.parse()
                    .unwrap_or_else(|_| panic!("Got invalid number '{s}'")),
            ),
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GoForward(step_count) => format!("Going forward (max {step_count})"),
                TurnLeft => "turning left".to_string(),
                TurnRight => "turning right".to_string(),
            }
        )
    }
}
type Pos = (Coord, Coord);
#[derive(Debug)]
struct Map {
    board: BTreeMap<Pos, Tile>,
    width: Coord,
    height: Coord,
    path_description: Vec<Instruction>,
}
impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let (map, path) = s.split_once("\n\n").unwrap();
        let board: BTreeMap<(Coord, Coord), Tile> = map
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, tile)| ['.', '#'].contains(tile))
                    .map(move |(x, tile)| (((x + 1) as isize, (y + 1) as isize), Tile::from(tile)))
            })
            .collect();

        let mut path_description = vec![];
        let mut num_i = 0;
        while num_i < path.len() {
            let mut turn_i = num_i + 1;
            while turn_i < path.len() && !path[num_i..=turn_i].ends_with(['L', 'R']) {
                turn_i += 1;
            }
            path_description.push(Instruction::from(&path[num_i..turn_i]));
            if turn_i < path.len() {
                path_description.push(Instruction::from(&path[turn_i..=turn_i]));
            }
            num_i = turn_i + 1;
        }

        let width = *board.keys().map(|(x, _y)| x).max().unwrap();
        let height = *board.keys().map(|(_x, y)| y).max().unwrap();
        Map {
            board,
            width,
            height,
            path_description,
        }
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (1..=self.height)
                .map(|y| {
                    (1..=self.width)
                        .into_iter()
                        .map(|x| self.board.get(&(x, y)).map(Tile::to_char).unwrap_or(' '))
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_example() {
        let map = Map::from(EXAMPLE);
        assert_eq!(6_032, map.follow_path_on_plane());
    }

    #[test]
    fn part1() {
        assert_eq!(31_568, day22_part1());
    }

    #[test]
    fn part2_example_right_transition_area() {
        let map = Map::from(EXAMPLE);
        let transition_areas = map.transition_area_by_facing();
        let right = &transition_areas[Right as usize];

        // top square
        assert_eq!(right.get(&(12, 1)), Some(&((16, 12), Left)));
        assert_eq!(right.get(&(12, 4)), Some(&((16, 9), Left)));
        // middle square
        assert_eq!(right.get(&(12, 5)), Some(&((16, 9), Down)));
        assert_eq!(right.get(&(12, 8)), Some(&((13, 9), Down)));
        // bottom square
        assert_eq!(right.get(&(16, 9)), Some(&((12, 4), Left)));
        assert_eq!(right.get(&(16, 12)), Some(&((12, 1), Left)));
    }

    #[test]
    fn part2_example_down_transition_area() {
        let map = Map::from(EXAMPLE);
        let transition_areas = map.transition_area_by_facing();
        let down = &transition_areas[Down as usize];

        // first middle square
        assert_eq!(down.get(&(1, 8)), Some(&((12, 12), Up)));
        assert_eq!(down.get(&(4, 8)), Some(&((9, 12), Up)));
        // second middle square
        assert_eq!(down.get(&(5, 8)), Some(&((9, 12), Right)));
        assert_eq!(down.get(&(8, 8)), Some(&((9, 9), Right)));
        // first middle square
        assert_eq!(down.get(&(9, 12)), Some(&((4, 8), Up)));
        assert_eq!(down.get(&(12, 12)), Some(&((1, 8), Up)));
        // last bottom square
        assert_eq!(down.get(&(13, 12)), Some(&((1, 8), Right)));
        assert_eq!(down.get(&(16, 12)), Some(&((1, 5), Right)));
    }

    #[test]
    fn part2_example_left_transition_area() {
        let map = Map::from(EXAMPLE);
        let transition_areas = map.transition_area_by_facing();
        let left = &transition_areas[Left as usize];

        // top square
        assert_eq!(left.get(&(9, 1)), Some(&((5, 5), Down)));
        assert_eq!(left.get(&(9, 4)), Some(&((8, 5), Down)));
        // middle square
        assert_eq!(left.get(&(1, 5)), Some(&((16, 12), Up)));
        assert_eq!(left.get(&(1, 8)), Some(&((13, 12), Up)));
        // bottom square
        assert_eq!(left.get(&(9, 9)), Some(&((8, 8), Up)));
        assert_eq!(left.get(&(9, 12)), Some(&((5, 8), Up)));
    }

    #[test]
    fn part2_example_up_transition_area() {
        let map = Map::from(EXAMPLE);
        let transition_areas = map.transition_area_by_facing();
        let up = &transition_areas[Up as usize];

        // first middle square
        assert_eq!(up.get(&(1, 5)), Some(&((12, 1), Down)));
        assert_eq!(up.get(&(4, 5)), Some(&((9, 1), Down)));
        // second middle square
        assert_eq!(up.get(&(5, 5)), Some(&((9, 1), Right)));
        assert_eq!(up.get(&(8, 5)), Some(&((9, 4), Right)));
        // top square
        assert_eq!(up.get(&(9, 1)), Some(&((4, 5), Down)));
        assert_eq!(up.get(&(12, 1)), Some(&((1, 5), Down)));
        // bottom square
        assert_eq!(up.get(&(13, 9)), Some(&((12, 8), Left)));
        assert_eq!(up.get(&(16, 9)), Some(&((12, 5), Left)));
    }

    #[test]
    fn part2_right_transition_area() {
        let map = Map::from(INPUT);
        let transition_areas = map.transition_area_by_facing();
        let right = &transition_areas[Right as usize];
        //
        assert_eq!(right.get(&(150, 1)), Some(&((100, 150), Left)));
        assert_eq!(right.get(&(150, 50)), Some(&((100, 101), Left)));
        //
        assert_eq!(right.get(&(100, 51)), Some(&((101, 50), Up)));
        assert_eq!(right.get(&(100, 100)), Some(&((150, 50), Up)));
        //
        assert_eq!(right.get(&(100, 101)), Some(&((150, 50), Left)));
        assert_eq!(right.get(&(100, 150)), Some(&((150, 1), Left)));
        //
        assert_eq!(right.get(&(50, 151)), Some(&((51, 150), Up)));
        assert_eq!(right.get(&(50, 200)), Some(&((100, 150), Up)));
    }

    #[test]
    fn part2_down_transition_area() {
        let map = Map::from(INPUT);
        let transition_areas = map.transition_area_by_facing();
        let down = &transition_areas[Down as usize];

        // left square
        assert_eq!(down.get(&(1, 200)), Some(&((101, 1), Down)));
        assert_eq!(down.get(&(50, 200)), Some(&((150, 1), Down)));
        // middle square
        assert_eq!(down.get(&(51, 150)), Some(&((50, 151), Left)));
        assert_eq!(down.get(&(100, 150)), Some(&((50, 200), Left)));
        // bottom square
        assert_eq!(down.get(&(101, 50)), Some(&((100, 51), Left)));
        assert_eq!(down.get(&(150, 50)), Some(&((100, 100), Left)));
    }

    #[test]
    fn part2_left_transition_area() {
        let map = Map::from(INPUT);
        let transition_areas = map.transition_area_by_facing();
        let left = &transition_areas[Left as usize];

        //
        assert_eq!(left.get(&(51, 1)), Some(&((1, 150), Right)));
        assert_eq!(left.get(&(51, 50)), Some(&((1, 101), Right)));
        //
        assert_eq!(left.get(&(51, 51)), Some(&((1, 101), Down)));
        assert_eq!(left.get(&(51, 100)), Some(&((50, 101), Down)));
        //
        assert_eq!(left.get(&(1, 101)), Some(&((51, 50), Right)));
        assert_eq!(left.get(&(1, 150)), Some(&((51, 1), Right)));
        //
        assert_eq!(left.get(&(1, 151)), Some(&((51, 1), Down)));
        assert_eq!(left.get(&(1, 200)), Some(&((100, 1), Down)));
    }

    #[test]
    fn part2_up_transition_area() {
        let map = Map::from(INPUT);
        let transition_areas = map.transition_area_by_facing();
        let up = &transition_areas[Up as usize];

        // left
        assert_eq!(up.get(&(1, 101)), Some(&((51, 51), Right)));
        assert_eq!(up.get(&(50, 101)), Some(&((51, 100), Right)));
        // middle
        assert_eq!(up.get(&(51, 1)), Some(&((1, 151), Right)));
        assert_eq!(up.get(&(100, 1)), Some(&((1, 200), Right)));
        // right
        assert_eq!(up.get(&(101, 1)), Some(&((1, 200), Up)));
        assert_eq!(up.get(&(150, 1)), Some(&((50, 200), Up)));
    }

    #[test]
    fn part2_example() {
        let map = Map::from(EXAMPLE);
        assert_eq!(5_031, map.follow_path_on_cube());
    }

    #[test]
    fn part2() {
        assert_eq!(36_540, day22_part2());
    }
}
