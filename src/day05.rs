use line_reader::read_file_to_lines;
use std::collections::HashMap;

pub(crate) fn day05_part1() -> usize {
    let lines = parse(read_file_to_lines("input/day05.txt"));
    count_overlaps(lines, false)
}

pub(crate) fn day05_part2() -> usize {
    let lines = parse(read_file_to_lines("input/day05.txt"));
    count_overlaps(lines, true)
}

fn parse(input: Vec<String>) -> Vec<Line> {
    input.into_iter().map(Line::from).collect()
}

fn count_overlaps(lines: Vec<Line>, include_diagonals: bool) -> usize {
    let mut overlaps: HashMap<Point, usize> = HashMap::new();
    for line in lines.into_iter() {
        if line.is_horizontal() || line.is_vertical() || include_diagonals {
            for point in line {
                *overlaps.entry(point).or_default() += 1;
            }
        }
    }
    overlaps.values().filter(|&&count| count > 1).count()
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}
impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
impl Point {
    fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }
    fn offset_by(&self, dx: i16, dy: i16) -> Self {
        Point::new((self.x + dx) as i16, (self.y + dy) as i16)
    }
}

struct Line {
    start: Point,
    end: Point,
    /// The current Point during iteration
    it: Option<Point>,
}
impl From<String> for Line {
    fn from(s: String) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();
        Line {
            start: Point::from(start),
            end: Point::from(end),
            it: None,
        }
    }
}
impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.x.eq(&self.end.x)
    }
    fn is_vertical(&self) -> bool {
        self.start.y.eq(&self.end.y)
    }
}
impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.it = match self.it {
            None => Some(self.start),
            Some(prev) if prev == self.end => None,
            Some(prev) => Some(prev.offset_by(
                (self.end.x - self.start.x).signum(),
                (self.end.y - self.start.y).signum(),
            )),
        };
        self.it
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_example() {
        let lines = parse(read_str_to_lines(EXAMPLE));
        assert_eq!(5, count_overlaps(lines, false));
    }
    #[test]
    fn part2_example() {
        let lines = parse(read_str_to_lines(EXAMPLE));
        assert_eq!(12, count_overlaps(lines, true));
    }

    #[test]
    fn part1() {
        assert_eq!(day05_part1(), 5197);
    }

    #[test]
    fn part2() {
        assert_eq!(day05_part2(), 18605);
    }
}
