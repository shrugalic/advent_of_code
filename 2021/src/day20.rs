use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day20.txt");

pub(crate) fn day20_part1() -> usize {
    ImageEnhancementSystem::from(INPUT).number_of_lit_pixels_after(2)
}

pub(crate) fn day20_part2() -> usize {
    ImageEnhancementSystem::from(INPUT).number_of_lit_pixels_after(50)
}

type Coord = isize;
type Pos = (Coord, Coord);

#[derive(Clone, Copy)]
struct Pixel {
    is_lit: bool,
}
impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Pixel { is_lit: true },
            '.' => Pixel { is_lit: false },
            _ => unreachable!(),
        }
    }
}
impl Pixel {
    fn is_lit(&self) -> bool {
        self.is_lit
    }
    fn to_char(self) -> char {
        match self.is_lit {
            true => '#',
            false => '.',
        }
    }
    fn to_num(self) -> usize {
        match self.is_lit {
            true => 1,
            false => 0,
        }
    }
}

struct ImageEnhancementSystem {
    algorithm: Vec<Pixel>,
    outside: Pixel,             // State of all pixels _outside_ the tracked area
    image: HashMap<Pos, Pixel>, // The tracked area has a border that is one pixel wide
}
impl From<&str> for ImageEnhancementSystem {
    fn from(input: &str) -> Self {
        let (algorithm, image_lines) = input.trim().split_once("\n\n").unwrap();
        let algorithm: Vec<Pixel> = algorithm.chars().map(Pixel::from).collect();
        assert_eq!(512, algorithm.len());
        let mut image = HashMap::new();
        for (y, line) in image_lines.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                image.insert((x as Coord, y as Coord), Pixel::from(c));
            }
        }
        ImageEnhancementSystem {
            algorithm,
            outside: Pixel::from('.'),
            image,
        }
    }
}
impl Display for ImageEnhancementSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = self.dimensions();
        let mut image = String::new();
        for y in y_range {
            for x in x_range.clone() {
                image.push(self.get_pixel(&(x, y)).to_char());
            }
            image.push('\n');
        }
        write!(f, "{}", image.trim_end())
    }
}
impl ImageEnhancementSystem {
    fn number_of_lit_pixels_after(mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self = self.enhance();
        }
        // println!("{}", self);
        self.number_of_lit_pixels()
    }
    fn number_of_lit_pixels(&self) -> usize {
        self.image.values().filter(|p| p.is_lit()).count()
    }
    fn enhance(mut self) -> Self {
        let (x_range, y_range) = self.dimensions();
        let mut image = HashMap::new();
        for y in y_range {
            for x in x_range.clone() {
                let p = self.enhance_pixel_at((x, y));
                image.insert((x as Coord, y as Coord), p);
            }
        }
        self.image = image;
        self.outside = match self.outside.is_lit() {
            false => self.algorithm[0],
            true => self.algorithm[511],
        };
        self
    }
    fn get_pixel(&self, at: &Pos) -> Pixel {
        self.image.get(at).cloned().unwrap_or(self.outside)
    }
    fn dimensions(&self) -> (RangeInclusive<Coord>, RangeInclusive<Coord>) {
        let min_x = *self.image.keys().map(|(x, _)| x).min().unwrap() - 1;
        let max_x = *self.image.keys().map(|(x, _)| x).max().unwrap() + 1;
        let min_y = *self.image.keys().map(|(_, y)| y).min().unwrap() - 1;
        let max_y = *self.image.keys().map(|(_, y)| y).max().unwrap() + 1;
        (min_x..=max_x, min_y..=max_y)
    }
    fn enhance_pixel_at(&self, pos: Pos) -> Pixel {
        let pixels = self.neighbors_of(pos);
        let index = pixels.to_index();
        self.algorithm[index]
    }

    fn neighbors_of(&self, pos: Pos) -> Vec<Pixel> {
        pos.neighbors()
            .iter()
            .map(|at| self.get_pixel(at))
            .collect()
    }
}

trait Neighbors {
    fn neighbors(&self) -> Vec<Pos>;
}
impl Neighbors for Pos {
    fn neighbors(&self) -> Vec<Pos> {
        let mut neighbors = vec![];
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                neighbors.push((self.0 + dx, self.1 + dy));
            }
        }
        neighbors
    }
}

trait ToIndex {
    fn to_index(&self) -> usize;
}
impl ToIndex for Vec<Pixel> {
    fn to_index(&self) -> usize {
        self.iter().map(|p| p.to_num()).fold(0, |a, x| (a << 1) + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_display() {
        let system = ImageEnhancementSystem::from(EXAMPLE);
        assert_eq!(system.algorithm.len(), 512);
        let expected = "\
.......
.#..#..
.#.....
.##..#.
...#...
...###.
.......";
        assert_eq!(expected, system.to_string());
    }

    #[test]
    fn test_part1_enhance_once() {
        let mut system = ImageEnhancementSystem::from(EXAMPLE);
        system = system.enhance();
        let expected = "\
.........
..##.##..
.#..#.#..
.##.#..#.
.####..#.
..#..##..
...##..#.
....#.#..
.........";
        assert_eq!(expected, system.to_string());
    }

    #[test]
    fn test_part1_enhance_twice() {
        let mut system = ImageEnhancementSystem::from(EXAMPLE);
        system = system.enhance().enhance();
        let expected = "\
...........
........#..
..#..#.#...
.#.#...###.
.#...##.#..
.#.....#.#.
..#.#####..
...#.#####.
....##.##..
.....###...
...........";
        assert_eq!(expected, system.to_string());
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            (2, 5).neighbors(),
            vec![
                (1, 4),
                (2, 4),
                (3, 4),
                (1, 5),
                (2, 5),
                (3, 5),
                (1, 6),
                (2, 6),
                (3, 6)
            ]
        );
    }

    #[test]
    fn test_to_index() {
        let pixels = "...#...#.".chars().map(Pixel::from).collect::<Vec<_>>();
        assert_eq!(34, pixels.to_index());
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            35,
            ImageEnhancementSystem::from(EXAMPLE).number_of_lit_pixels_after(2)
        );
    }

    #[test]
    fn part1() {
        // 5577 is too low (that's without considering outside after 1 step)
        // 5705 is too high (that's without considering outside after 2 steps)
        assert_eq!(5663, day20_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            3351,
            ImageEnhancementSystem::from(EXAMPLE).number_of_lit_pixels_after(50)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(19_638, day20_part2());
    }

    const EXAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";
}
