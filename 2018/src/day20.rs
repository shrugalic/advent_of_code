use crate::parse;
use std::fmt::{Debug, Formatter};

const INPUT: &str = include_str!("../input/day20.txt");

pub(crate) fn day20_part1() -> usize {
    let base = Base::from(parse(INPUT)[0]);
    base.furthest_room_from_start()
}

pub(crate) fn day20_part2() -> usize {
    let base = Base::from(parse(INPUT)[0]);
    base.number_of_rooms_at_least_1000_doors_away()
}

type Coord = isize;
type X = Coord;
type Y = Coord;

#[derive(Clone, Copy)]
struct Loc {
    x: X,
    y: Y,
}
impl Loc {
    fn new(x: X, y: Y) -> Self {
        Loc { x, y }
    }
    fn offset_by(&self, other: &Loc) -> Self {
        Loc::new(self.x + other.x, self.y + other.y)
    }
    fn move_to(&self, dir: &Direction) -> Self {
        self.offset_by(&dir.to_offset())
    }
}
impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Room,
    VDoor,
    HDoor,
    Start,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Wall => "#",
            Tile::Room => ".",
            Tile::VDoor => "|",
            Tile::HDoor => "-",
            Tile::Start => "X",
        }
        .to_string()
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl Tile {
    fn matches(&self, filter: &dyn Fn(&Tile) -> bool) -> bool {
        filter(self)
    }
}

struct Grid<T: ToString> {
    grid: Vec<Vec<T>>,
}

impl<T: ToString> ToString for Grid<T> {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
impl<T: ToString> Grid<T> {
    fn get(&self, loc: &Loc) -> Option<&T> {
        self.grid
            .get(loc.y as usize)
            .and_then(|line| line.get(loc.x as usize))
    }
    fn mut_get(&mut self, loc: &Loc) -> Option<&mut T> {
        self.grid
            .get_mut(loc.y as usize)
            .and_then(|line| line.get_mut(loc.x as usize))
    }
    fn set(&mut self, loc: &Loc, t: T) {
        self.grid[loc.y as usize][loc.x as usize] = t;
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
}
impl<T: ToString + PartialEq> Grid<T> {
    fn locations_matching(&self, wanted: &T) -> Vec<Loc> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, tile)| tile == &wanted)
                    .map(move |(x, _)| Loc::new(x as isize, y as isize))
            })
            .collect::<Vec<_>>()
    }
}

enum Direction {
    N,
    E,
    S,
    W,
}
impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            'W' => Direction::W,
            _ => panic!("Illegal direction {}", ch),
        }
    }
}
impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::N => "N",
            Direction::E => "E",
            Direction::S => "S",
            Direction::W => "W",
        }
        .to_string()
    }
}
impl Direction {
    fn to_offset(&self) -> Loc {
        match self {
            Direction::N => Loc::new(0, -1),
            Direction::E => Loc::new(1, 0),
            Direction::S => Loc::new(0, 1),
            Direction::W => Loc::new(-1, 0),
        }
    }
    fn all() -> [Direction; 4] {
        [Direction::N, Direction::E, Direction::S, Direction::W]
    }
}

pub(crate) struct Base {
    grid: Grid<Tile>,
}
impl ToString for Base {
    fn to_string(&self) -> String {
        self.grid.to_string()
    }
}
impl From<&str> for Base {
    fn from(instructions: &str) -> Self {
        let mut path: Vec<(Loc, Tile)> = vec![];
        let mut curr = Loc::new(0, 0);
        path.push((curr, Tile::Start));

        let direction_and_door_type_from = |dir| match dir {
            'N' => (Direction::from(dir), Tile::HDoor),
            'S' => (Direction::from(dir), Tile::HDoor),
            'E' => (Direction::from(dir), Tile::VDoor),
            'W' => (Direction::from(dir), Tile::VDoor),
            _ => unreachable!(),
        };

        let mut branch_starts = vec![];
        instructions.chars().for_each(|ch| {
            if let Some(sub_path) = match ch {
                'N' | 'S' | 'E' | 'W' => {
                    let (direction, door_type) = direction_and_door_type_from(ch);
                    let door = curr.offset_by(&direction.to_offset());
                    let room = door.offset_by(&direction.to_offset());
                    Some([(door, door_type), (room, Tile::Room)])
                }
                '(' => {
                    branch_starts.push(curr);
                    None
                }
                ')' => {
                    curr = branch_starts.pop().unwrap();
                    None
                }
                '|' => {
                    curr = *branch_starts.last().unwrap();
                    None
                }
                '^' | '$' => None,
                _ => None,
            } {
                let end = sub_path[1].0;
                curr = end;
                path.extend(sub_path);
            }
        });

        Base::from(path)
    }
}
impl From<Vec<(Loc, Tile)>> for Base {
    fn from(path: Vec<(Loc, Tile)>) -> Base {
        // Determine the grid size from path
        let xs = || path.iter().map(|(loc, _)| loc.x);
        let ys = || path.iter().map(|(loc, _)| loc.y);
        // add -1/+1 extra to account for the outside wall
        let x_range = (xs().min().unwrap() - 1)..=(xs().max().unwrap() + 1);
        let y_range = (ys().min().unwrap() - 1)..=(ys().max().unwrap() + 1);

        // Create a grid of all walls
        let mut grid = Grid {
            grid: vec![
                vec![Tile::Wall; (x_range.end() - x_range.start()) as usize + 1];
                (y_range.end() - y_range.start()) as usize + 1
            ],
        };

        // Carve the path into it
        let start = Loc::new(-*x_range.start(), -*y_range.start());
        path.iter()
            .for_each(|(loc, tile)| grid.set(&loc.offset_by(&start), *tile));
        Base { grid }
    }
}
impl Base {
    pub(crate) fn furthest_room_from_start(&self) -> usize {
        let distances = self.calculate_shortest_paths();
        distances
            .grid
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|dist| dist < &usize::MAX)
            .max()
            .unwrap()
    }
    pub(crate) fn number_of_rooms_at_least_1000_doors_away(&self) -> usize {
        let distances = self.calculate_shortest_paths();
        distances
            .grid
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|dist| dist >= &1000 && dist < &usize::MAX)
            .count()
    }

    fn calculate_shortest_paths(&self) -> Grid<usize> {
        let grid = vec![vec![usize::MAX; self.grid.width()]; self.grid.height()];
        let mut distances = Grid { grid };

        let start = self.grid.locations_matching(&Tile::Start).remove(0);
        distances.set(&start, 0);
        let mut candidates = vec![(start, 0)];
        let a_door = |tile: &Tile| tile == &Tile::VDoor || tile == &Tile::HDoor;
        while let Some((loc, dist)) = candidates.pop() {
            candidates.extend(
                Direction::all()
                    .iter()
                    .filter(|dir| {
                        let next_tile = self.grid.get(&loc.move_to(dir)).unwrap();
                        next_tile.matches(&a_door)
                    })
                    .map(|dir| loc.move_to(dir).move_to(dir))
                    .filter(|next_loc| Some(&Tile::Room) == self.grid.get(next_loc))
                    .filter(|next_loc| {
                        let existing = distances.mut_get(next_loc).unwrap();
                        if &(dist + 2) < existing {
                            *existing = dist + 1;
                            true
                        } else {
                            false
                        }
                    })
                    .map(|next_loc| (next_loc, dist + 1)),
            );
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#####
#.|.#
#-###
#.|X#
#####";

    #[test]
    fn example_base_to_string() {
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Room, Tile::VDoor, Tile::Room, Tile::Wall],
            vec![Tile::Wall, Tile::HDoor, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Room, Tile::VDoor, Tile::Start, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        let base = Base {
            grid: Grid { grid },
        };

        assert_eq!(base.to_string(), EXAMPLE);
    }

    #[test]
    fn example_base_from_instructions() {
        let base = Base::from("^WNE$");
        assert_eq!(base.to_string(), EXAMPLE);
    }

    #[test]
    fn branching_example() {
        let base = Base::from("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(
            base.to_string(),
            "\
#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########"
        );
    }

    #[test]
    fn empty_branch_example() {
        let base = Base::from("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(
            base.to_string(),
            "\
###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########"
        );
    }

    #[test]
    fn basic_example_distance() {
        let base = Base::from("^WNE$");
        assert_eq!(3, base.furthest_room_from_start());
    }

    #[test]
    fn branching_example_distance() {
        let base = Base::from("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(10, base.furthest_room_from_start());
    }

    #[test]
    fn empty_branch_example_distance() {
        let base = Base::from("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(18, base.furthest_room_from_start());
    }

    #[test]
    fn part1() {
        assert_eq!(4_360, day20_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(8_509, day20_part2());
    }
}
