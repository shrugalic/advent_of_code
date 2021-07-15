use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

#[cfg(test)]
mod tests;

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

#[derive(PartialEq)]
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

pub fn black_tile_count(input: &[String]) -> usize {
    let mut floor: HashMap<Coordinate, Color> = HashMap::new();
    input
        .iter()
        .map(Path::from)
        .map(Coordinate::from)
        .for_each(|coord| {
            let tile = floor.entry(coord).or_insert_with(Color::default);
            tile.flip();
        });

    let (black, white): (Vec<&Color>, Vec<&Color>) =
        floor.values().partition(|&color| color == &Color::Black);
    println!("{} black and {} white", black.len(), white.len());
    black.len()
}
