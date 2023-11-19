use crate::parse;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day19.txt");

pub(crate) fn day19_part1() -> String {
    follow_path_and_return_letters_and_step_count(parse(INPUT)).0
}

pub(crate) fn day19_part2() -> usize {
    follow_path_and_return_letters_and_step_count(parse(INPUT)).1
}

fn follow_path_and_return_letters_and_step_count(input: Vec<&str>) -> (String, usize) {
    let grid = parse_input(input);
    let mut y = 0;
    let mut x = grid[y].iter().position(|c| c == &'|').unwrap();
    let mut dir = Dir::South;
    let mut collected = vec![];
    let mut step_count = 0;
    let tile_at = |x, y| grid.get(y).and_then(|line: &Vec<char>| line.get(x));
    while let Some(tile) = tile_at(x, y) {
        // println!("({}, {}) {} {}", x, y, dir, tile);
        match tile {
            '+' => match dir {
                Dir::South | Dir::North => {
                    if let Some(' ' | '|') = tile_at(x + 1, y) {
                        x -= 1;
                        dir = Dir::West
                    } else {
                        x += 1;
                        dir = Dir::East
                    }
                }
                Dir::East | Dir::West => {
                    if let Some(' ' | '-') = tile_at(x, y + 1) {
                        y -= 1;
                        dir = Dir::North
                    } else {
                        y += 1;
                        dir = Dir::South
                    }
                }
            },
            ' ' => break,
            c => {
                if c != &'-' && c != &'|' {
                    collected.push(*c);
                }
                match dir {
                    Dir::South => y += 1,
                    Dir::North => y -= 1,
                    Dir::East => x += 1,
                    Dir::West => x -= 1,
                }
            }
        }
        step_count += 1;
    }

    (collected.into_iter().collect::<String>(), step_count)
}

enum Dir {
    South,
    East,
    North,
    West,
}
impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::South => "v",
                Dir::East => ">",
                Dir::North => "^",
                Dir::West => "<",
            }
        )
    }
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "     |          \
                         \n     |  +--+    \
                         \n     A  |  C    \
                         \n F---|----E|--+ \
                         \n     |  |  |  D \
                         \n     +B-+  +--+ \
                         \n                ";

    #[test]
    fn part1_example() {
        assert_eq!(
            "ABCDEF",
            follow_path_and_return_letters_and_step_count(parse(EXAMPLE)).0
        );
    }
    #[test]
    fn part1() {
        assert_eq!("MKXOIHZNBL", day19_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            38,
            follow_path_and_return_letters_and_step_count(parse(EXAMPLE)).1
        );
    }

    #[test]
    fn part2() {
        assert_eq!(17872, day19_part2());
    }
}
