type Coord = isize;
type Radius = usize;

#[derive(Debug)]
struct Loc {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Loc {
    fn distance_to(&self, other: &Loc) -> usize {
        Loc::diff(self.x, other.x) + Loc::diff(self.y, other.y) + Loc::diff(self.z, other.z)
    }
    fn diff(a: Coord, b: Coord) -> usize {
        (a - b).abs() as usize
    }
}

#[derive(Debug)]
struct Nanobot {
    center: Loc,
    radius: Radius,
}

impl Nanobot {
    fn is_in_range_of(&self, signal: &Nanobot) -> bool {
        self.center.distance_to(&signal.center) <= signal.radius
    }
}

impl From<String> for Nanobot {
    fn from(s: String) -> Self {
        let (pos, r) = s.split_once(">, r=").unwrap();
        let pos: Vec<Coord> = pos
            .trim_start_matches("pos=<")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let center = Loc {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        };
        let radius = r.parse().unwrap();
        Nanobot { center, radius }
    }
}

pub(crate) fn count_nanobots_in_signal_range(input: Vec<String>) -> usize {
    let nanobots: Vec<Nanobot> = input.into_iter().map(Nanobot::from).collect();
    let signal = nanobots.iter().max_by_key(|n| n.radius).unwrap();

    nanobots
        .iter()
        .filter(|bot| bot.is_in_range_of(signal))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_1: &str = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    #[test]
    fn example_count_nanobots_in_signal_range() {
        assert_eq!(
            7,
            count_nanobots_in_signal_range(read_str_to_lines(EXAMPLE_1))
        );
    }

    #[test]
    fn part1_count_nanobots_in_signal_range() {
        assert_eq!(
            417,
            count_nanobots_in_signal_range(read_file_to_lines("input/day23.txt"))
        );
    }
}
