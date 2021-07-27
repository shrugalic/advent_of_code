use std::cmp::Ordering;
use std::convert::TryFrom;

type Coord = isize;
type Location = (Coord, Coord);

enum Track {
    Intersection,
    Slash,
    Backslash,
    Vertical,
    Horizontal,
}
impl From<char> for Track {
    fn from(ch: char) -> Self {
        match ch {
            '+' => Track::Intersection,
            '/' => Track::Slash,
            '\\' => Track::Backslash,
            '|' | '^' | 'v' => Track::Vertical,
            '-' | '<' | '>' => Track::Horizontal,
            _ => panic!("Illegal track {}", ch),
        }
    }
}
impl Track {
    fn from_location(loc: &Location, lines: &[String]) -> Track {
        let track_char = lines[loc.1 as usize].chars().nth(loc.0 as usize).unwrap();
        Track::from(track_char)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Cart {
    loc: Location,
    dir: Dir,
    next: Choice,
}
impl PartialOrd<Self> for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Only the location matters: y first, then x.
        let y_ord = self.loc.1.partial_cmp(&other.loc.1).unwrap();
        match y_ord {
            Ordering::Less | Ordering::Greater => Some(y_ord),
            Ordering::Equal => self.loc.0.partial_cmp(&other.loc.0),
        }
    }
}
impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
impl Cart {
    fn move_1(&mut self) {
        let offset = self.dir.offset();
        self.loc.0 += offset.0;
        self.loc.1 += offset.1;
    }
    fn turn(&mut self, track: &Track) {
        let dir = self.dir.clone();
        self.dir = match (&dir, track) {
            (Dir::Right, Track::Slash) | (Dir::Left, Track::Backslash) => Dir::Up,
            (Dir::Right, Track::Backslash) | (Dir::Left, Track::Slash) => Dir::Down,
            (Dir::Up, Track::Slash) | (Dir::Down, Track::Backslash) => Dir::Right,
            (Dir::Up, Track::Backslash) | (Dir::Down, Track::Slash) => Dir::Left,
            (_dir, Track::Intersection) => match (&dir, &self.next) {
                (Dir::Right, Choice::Right) | (Dir::Left, Choice::Left) => Dir::Down,
                (Dir::Right, Choice::Left) | (Dir::Left, Choice::Right) => Dir::Up,
                (Dir::Up, Choice::Right) | (Dir::Down, Choice::Left) => Dir::Right,
                (Dir::Up, Choice::Left) | (Dir::Down, Choice::Right) => Dir::Left,
                (_, Choice::Straight) => dir,
            },
            (_, Track::Vertical | Track::Horizontal) => dir,
        };
        if matches!(track, Track::Intersection) {
            self.next.advance();
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}
impl TryFrom<char> for Dir {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Dir::Right),
            '<' => Ok(Dir::Left),
            '^' => Ok(Dir::Up),
            'v' => Ok(Dir::Down),
            _ => Err(()),
        }
    }
}
impl Dir {
    fn offset(&self) -> Location {
        match self {
            Dir::Right => (1, 0),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Choice {
    Right,
    Left,
    Straight,
}
impl Choice {
    fn advance(&mut self) {
        *self = match self {
            Choice::Left => Choice::Straight,
            Choice::Straight => Choice::Right,
            Choice::Right => Choice::Left,
        };
    }
}

pub(crate) fn location_of_first_crash(lines: &[String]) -> Location {
    let mut carts = initial_cart_locations(lines);
    loop {
        let mut handled_carts: Vec<Cart> = Vec::new();
        carts.sort();
        // println!("Carts: {:?}", carts);
        while !carts.is_empty() {
            let mut cart = carts.remove(0);

            // Move to new location
            cart.move_1();

            // check for collisions
            if carts.iter().any(|other| other.loc == cart.loc)
                || handled_carts.iter().any(|other| other.loc == cart.loc)
            {
                println!("{:?}", cart);
                return cart.loc;
            }

            // Turn into new direction as needed
            let track = Track::from_location(&cart.loc, &lines);
            cart.turn(&track);

            handled_carts.push(cart);
        }
        carts = handled_carts;
    }
}

fn initial_cart_locations(lines: &[String]) -> Vec<Cart> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                Dir::try_from(ch)
                    .map(|dir| Cart {
                        loc: (x as isize, y as isize),
                        dir,
                        next: Choice::Left,
                    })
                    .ok()
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    // The double backslashes are escaped single backslashes,
    // and the \n\ prevents IntelliJ from trimming the end of the line
    const EXAMPLE: &str = "\
/->-\\        \n\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ";

    #[test]
    fn example_joined_lines_from_file_match_const_str() {
        let lines = read_file_to_lines("input/day13example.txt").join("\n");
        assert_eq!(EXAMPLE, lines);
    }

    #[test]
    fn straight_track() {
        let track = "\
|
v
|
|
|
^
|";
        assert_eq!((0, 3), location_of_first_crash(&read_str_to_lines(track)));
    }

    #[test]
    fn backslash_curve() {
        let track = "\
->--\\
    |
    ^
    |";
        assert_eq!((4, 0), location_of_first_crash(&read_str_to_lines(track)));
    }

    #[test]
    fn example() {
        assert_eq!((7, 3), location_of_first_crash(&read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part1() {
        assert_eq!(
            (102, 114),
            location_of_first_crash(&read_file_to_lines("input/day13.txt"))
        );
    }
}
