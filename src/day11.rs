use line_reader::read_file_to_lines;

pub(crate) fn day11_part1() -> usize {
    distance_to_origin(read_file_to_lines("input/day11.txt"))
}

fn distance_to_origin(input: Vec<String>) -> usize {
    curr_and_max_distances_to_origin(input).0
}

fn max_distance_to_origin(input: Vec<String>) -> usize {
    curr_and_max_distances_to_origin(input).1
}

fn curr_and_max_distances_to_origin(input: Vec<String>) -> (usize, usize) {
    let steps: Vec<Dir> = input[0].split(',').map(Dir::from).collect();
    let mut max_dist = 0;

    let mut pos = Hex::default();
    for dir in steps {
        pos.move_in(dir);
        max_dist = max_dist.max(pos.distance_to_origin());
    }

    (pos.distance_to_origin(), max_dist)
}

pub(crate) fn day11_part2() -> usize {
    max_distance_to_origin(read_file_to_lines("input/day11.txt"))
}

enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl From<&str> for Dir {
    fn from(dir: &str) -> Self {
        match dir {
            "n" => Dir::N,
            "ne" => Dir::NE,
            "se" => Dir::SE,
            "s" => Dir::S,
            "sw" => Dir::SW,
            "nw" => Dir::NW,
            _ => panic!("Illegal direction {}", dir),
        }
    }
}

struct Hex {
    x: isize,
    y: isize,
    z: isize,
}

impl Default for Hex {
    fn default() -> Self {
        Hex { x: 0, y: 0, z: 0 }
    }
}

impl Hex {
    fn move_in(&mut self, dir: Dir) {
        match dir {
            Dir::N => {
                self.y += 1;
                self.z -= 1;
            }
            Dir::NE => {
                self.x += 1;
                self.z -= 1;
            }
            Dir::SE => {
                self.x += 1;
                self.y -= 1;
            }
            Dir::S => {
                self.y -= 1;
                self.z += 1;
            }
            Dir::SW => {
                self.x -= 1;
                self.z += 1;
            }
            Dir::NW => {
                self.x -= 1;
                self.y += 1;
            }
        }
    }
    fn distance_to_origin(&self) -> usize {
        self.distance_to(&Hex::default())
    }
    fn distance_to(&self, other: &Hex) -> usize {
        (((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2)
            as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    #[test]
    fn part1_examples() {
        assert_eq!(3, distance_to_origin(read_str_to_lines("ne,ne,ne")));
        assert_eq!(0, distance_to_origin(read_str_to_lines("ne,ne,sw,sw")));
        assert_eq!(2, distance_to_origin(read_str_to_lines("ne,ne,s,s")));
        assert_eq!(3, distance_to_origin(read_str_to_lines("se,sw,se,sw,sw")));
    }

    #[test]
    fn part1_full() {
        assert_eq!(722, day11_part1());
    }

    #[test]
    fn part2_full() {
        assert_eq!(1551, day11_part2());
    }
}
