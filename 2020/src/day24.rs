use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::East => "E",
            Direction::SouthEast => "SE",
            Direction::SouthWest => "SW",
            Direction::West => "W",
            Direction::NorthWest => "NW",
            Direction::NorthEast => "NE",
        };
        write!(f, "{}", s)
    }
}
impl Direction {
    fn to_coord(&self) -> Coordinate {
        match self {
            Direction::East => Coordinate { x: 1, y: -1, z: 0 },
            Direction::SouthEast => Coordinate { x: 0, y: -1, z: 1 },
            Direction::SouthWest => Coordinate { x: -1, y: 0, z: 1 },
            Direction::West => Coordinate { x: -1, y: 1, z: 0 },
            Direction::NorthWest => Coordinate { x: 0, y: 1, z: -1 },
            Direction::NorthEast => Coordinate { x: 1, y: 0, z: -1 },
        }
    }

    fn all_direction_coordinates() -> [Coordinate; 6] {
        [
            Direction::East.to_coord(),
            Direction::SouthEast.to_coord(),
            Direction::SouthWest.to_coord(),
            Direction::West.to_coord(),
            Direction::NorthWest.to_coord(),
            Direction::NorthEast.to_coord(),
        ]
    }
}

struct Path {
    path: Vec<Direction>,
}
impl<T: AsRef<str>> From<T> for Path {
    fn from(line: T) -> Self {
        let path = line
            .as_ref()
            .split_inclusive(|c| c == 'e' || c == 'w')
            .map(Direction::from)
            .collect();
        Path { path }
    }
}
impl Debug for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

#[derive(PartialEq, Clone)]
enum Color {
    Black,
    White,
}
impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}
impl Color {
    fn flip(&mut self) {
        *self = match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
    }
    fn is_black(&self) -> bool {
        self == &Color::Black
    }
}
#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}
impl Default for Coordinate {
    fn default() -> Self {
        Coordinate { x: 0, y: 0, z: 0 }
    }
}
impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl From<Path> for Coordinate {
    fn from(path: Path) -> Self {
        path.path
            .iter()
            .map(|dir| dir.to_coord())
            .fold(Coordinate::default(), |pos, coord| pos + coord)
    }
}
impl Coordinate {
    fn neighbors(&self) -> Vec<Coordinate> {
        Direction::all_direction_coordinates()
            .iter()
            .map(|dir| *self + *dir)
            .collect()
    }
}

impl<T: AsRef<str> + Display> From<T> for Direction {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "e" => Direction::East,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            "w" => Direction::West,
            "nw" => Direction::NorthWest,
            "ne" => Direction::NorthEast,
            _ => panic!("Invalid Direction {}", s),
        }
    }
}

pub(crate) fn black_tile_count(input: &[String]) -> usize {
    let floor = floor_from_input(input);
    count_black_tiles(&floor)
}

type Floor = HashMap<Coordinate, Color>;
fn count_black_tiles(floor: &Floor) -> usize {
    floor
        .values()
        .filter(|&color| color == &Color::Black)
        .count()
}

pub(crate) fn iterate_for_given_number_of_days(input: &[String], days: usize) -> usize {
    let mut floor = floor_from_input(input);

    for _ in 0..days {
        floor = iterate(floor);
    }
    count_black_tiles(&floor)
}

fn iterate(floor: Floor) -> Floor {
    let floor = extend_with_missing_neighbors(&floor);
    apply_iteration_rules(&floor)
}

fn extend_with_missing_neighbors(prev: &Floor) -> Floor {
    let mut new = prev.clone();
    prev.iter().for_each(|(pos, _)| {
        pos.neighbors().into_iter().for_each(|neighbor| {
            new.entry(neighbor).or_insert_with(Color::default);
        })
    });
    // println!(
    //     "Extended: {}/{}, prev: {}/{}",
    //     count_black_tiles(&new),
    //     new.len(),
    //     count_black_tiles(&prev),
    //     prev.len()
    // );
    new
}

fn apply_iteration_rules(prev: &Floor) -> Floor {
    let mut new = prev.clone();
    prev.iter().for_each(|(pos, my_color)| {
        let black_count = pos
            .neighbors()
            .iter()
            .filter_map(|neighbor| prev.get(neighbor))
            .filter(|c| c.is_black())
            .count();
        match (my_color, black_count) {
            (Color::Black, 0 | 3..=6) => new.get_mut(pos).unwrap().flip(),
            (Color::White, 2) => new.get_mut(pos).unwrap().flip(),
            (_, _) => {}
        }
    });
    // println!(
    //     "Applied: {}/{}, prev: {}/{}",
    //     count_black_tiles(&new),
    //     new.len(),
    //     count_black_tiles(&prev),
    //     prev.len()
    // );
    new
}

fn floor_from_input(input: &[String]) -> Floor {
    let mut floor: Floor = HashMap::new();
    input
        .iter()
        .map(Path::from)
        .map(Coordinate::from)
        .for_each(|coord| {
            let tile = floor.entry(coord).or_insert_with(Color::default);
            tile.flip();
        });
    floor
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn coordinate_of_path_esenee() {
        let expected = Coordinate { x: 3, y: -3, z: 0 };
        let actual = Coordinate::from(Path::from("esenee"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn coordinate_of_path_esew() {
        let expected = Coordinate { x: 0, y: -1, z: 1 };
        let actual = Coordinate::from(Path::from("esew"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn coordinate_of_path_nwwswee() {
        let expected = Coordinate { x: 0, y: 0, z: 0 };
        let actual = Coordinate::from(Path::from("nwwswee"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_1() {
        assert_eq!(black_tile_count(&read_str_to_lines(EXAMPLE)), 10);
    }

    #[test]
    fn part_1() {
        assert_eq!(
            black_tile_count(&read_file_to_lines("input/day24.txt")),
            287
        );
    }

    #[test]
    fn part_2_example_1_day_1() {
        assert_eq!(
            iterate_for_given_number_of_days(&read_str_to_lines(EXAMPLE), 1),
            15
        );
    }

    #[test]
    fn part_2_example_1_day_2() {
        assert_eq!(
            iterate_for_given_number_of_days(&read_str_to_lines(EXAMPLE), 2),
            12
        );
    }

    #[test]
    fn part_2_example_1_day_100() {
        assert_eq!(
            iterate_for_given_number_of_days(&read_str_to_lines(EXAMPLE), 100),
            2208
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            iterate_for_given_number_of_days(&read_file_to_lines("input/day24.txt"), 100),
            3636
        );
    }
}
