use crate::parse;
use reformation::Reformation;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day10.txt");

type Coord = isize;

#[derive(Reformation, Debug, PartialEq)]
#[reformation("<\\s?{x}, \\s?{y}>")]
struct Coord2D {
    x: Coord,
    y: Coord,
}
impl Coord2D {
    fn equals(&self, x: Coord, y: Coord) -> bool {
        self.x == x && self.y == y
    }
}

#[derive(Reformation, Debug, PartialEq)]
#[reformation(r"position={position} velocity={velocity}")]
struct Point {
    position: Coord2D,
    velocity: Coord2D,
}
impl Point {
    fn is_at_pos(&self, x: Coord, y: Coord) -> bool {
        self.position.equals(x, y)
    }
}

pub(crate) fn day10_part1() -> usize {
    message(&parse(INPUT)).1
}

fn message(input: &[&str]) -> (String, usize) {
    let mut points = to_points(input);
    let mut seconds = 0;
    while !has_many_vertically_aligned_points(&points) {
        points = iterate(points);
        seconds += 1;
    }
    (to_string(&points), seconds)
}

fn has_many_vertically_aligned_points(points: &[Point]) -> bool {
    let min_x = points.iter().map(|p| p.position.x).min().unwrap();
    points.iter().filter(|p| p.position.x == min_x).count() >= 8
}

fn iterate(mut points: Vec<Point>) -> Vec<Point> {
    points.iter_mut().for_each(|p| {
        p.position.x += p.velocity.x;
        p.position.y += p.velocity.y;
    });
    points
}

fn to_points(input: &[&str]) -> Vec<Point> {
    input
        .iter()
        .map(|line| Point::parse(line).unwrap())
        .collect()
}

fn to_string(points: &[Point]) -> String {
    let x_range = get_x_range(&points);
    let y_range = get_y_range(&points);
    y_range
        .into_iter()
        .map(|y| {
            x_range
                .clone()
                .into_iter()
                .map(|x| {
                    if points.iter().any(|p| p.is_at_pos(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_x_range(points: &[Point]) -> RangeInclusive<isize> {
    points.iter().map(|p| p.position.x).min().unwrap()
        ..=points.iter().map(|p| p.position.x).max().unwrap()
}

fn get_y_range(points: &[Point]) -> RangeInclusive<isize> {
    points.iter().map(|p| p.position.y).min().unwrap()
        ..=points.iter().map(|p| p.position.y).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn parse_lines() {
        assert_eq!(
            Point::parse("position=< 9,  1> velocity=< 0,  2>").unwrap(),
            Point {
                position: Coord2D { x: 9, y: 1 },
                velocity: Coord2D { x: 0, y: 2 }
            }
        );
        assert_eq!(
            Point::parse("position=< 3, -2> velocity=<-1,  1>").unwrap(),
            Point {
                position: Coord2D { x: 3, y: -2 },
                velocity: Coord2D { x: -1, y: 1 }
            }
        );
    }

    #[test]
    fn part_1() {
        let input = &parse(INPUT);
        assert_eq!((PART_1_MESSAGE.to_string(), 10511), message(input));
    }

    #[test]
    fn example_message() {
        let input = &parse(EXAMPLE_POINTS);
        assert_eq!((EXAMPLE_MESSAGE.to_string(), 3), message(input));
    }

    #[test]
    fn before_and_after_iterations() {
        let mut points = to_points(&parse(EXAMPLE_POINTS));
        assert_eq!(INITIAL, to_string(&points));
        points = iterate(points);
        assert_eq!(AFTER_1_ITERATION, to_string(&points));
        points = iterate(points);
        assert_eq!(AFTER_2_ITERATIONS, to_string(&points));
        points = iterate(points);
        points = iterate(points);
        assert_eq!(AFTER_4_ITERATIONS, to_string(&points));
    }

    const INITIAL: &str = "........#.............
................#.....
.........#.#..#.......
......................
#..........#.#.......#
...............#......
....#.................
..#.#....#............
.......#..............
......#...............
...#...#.#...#........
....#..#..#.........#.
.......#..............
...........#..#.......
#...........#.........
...#.......#..........";

    const AFTER_1_ITERATION: &str = "........#....#....
......#.....#.....
#.........#......#
..................
....#.............
..##.........#....
....#.#...........
...##.##..#.......
......#.#.........
......#...#.....#.
#...........#.....
..#.....#.#.......";

    const AFTER_2_ITERATIONS: &str = "..........#...
#..#...####..#
..............
....#....#....
..#.#.........
...#...#......
...#..#..#.#..
#....#.#......
.#...#...##.#.
....#.........";
    const EXAMPLE_MESSAGE: &str = "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###";

    // FNRGPBHR
    const PART_1_MESSAGE: &str = "######..#....#..#####....####...#####...#####...#....#..#####.
#.......##...#..#....#..#....#..#....#..#....#..#....#..#....#
#.......##...#..#....#..#.......#....#..#....#..#....#..#....#
#.......#.#..#..#....#..#.......#....#..#....#..#....#..#....#
#####...#.#..#..#####...#.......#####...#####...######..#####.
#.......#..#.#..#..#....#..###..#.......#....#..#....#..#..#..
#.......#..#.#..#...#...#....#..#.......#....#..#....#..#...#.
#.......#...##..#...#...#....#..#.......#....#..#....#..#...#.
#.......#...##..#....#..#...##..#.......#....#..#....#..#....#
#.......#....#..#....#...###.#..#.......#####...#....#..#....#";

    const AFTER_4_ITERATIONS: &str = "........#....
....##...#.#.
..#.....#..#.
.#..##.##.#..
...##.#....#.
.......#....#
..........#..
#......#...#.
.#.....##....
...........#.
...........#.";

    const EXAMPLE_POINTS: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
}
