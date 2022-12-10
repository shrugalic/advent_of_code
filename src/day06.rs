use crate::parse;
use std::ops::RangeInclusive;
const INPUT: &str = include_str!("../input/day06.txt");

pub(crate) fn day06_part1() -> usize {
    count_turned_on_lights(parse(INPUT))
}

pub(crate) fn day06_part2() -> usize {
    total_brightness(parse(INPUT))
}

fn count_turned_on_lights(strings: Vec<&str>) -> usize {
    let mut grid = [[false; 1000]; 1000];
    let instructions: Vec<_> = strings.into_iter().map(Instruction::from).collect();
    for instr in instructions {
        for y in instr.y_range() {
            for x in instr.x_range() {
                match instr.action {
                    Action::On => grid[y][x] = true,
                    Action::Off => grid[y][x] = false,
                    Action::Toggle => grid[y][x] = !grid[y][x],
                }
            }
        }
    }
    grid.iter()
        .map(|row| row.iter().filter(|&on| *on).count())
        .sum()
}

fn total_brightness(strings: Vec<&str>) -> usize {
    let mut grid = [[0u8; 1000]; 1000];
    let instructions: Vec<_> = strings.into_iter().map(Instruction::from).collect();
    for instr in instructions {
        for y in instr.y_range() {
            for x in instr.x_range() {
                match instr.action {
                    Action::On => grid[y][x] += 1,
                    Action::Off => {
                        if grid[y][x] > 0 {
                            grid[y][x] -= 1;
                        }
                    }
                    Action::Toggle => grid[y][x] += 2,
                }
            }
        }
    }
    grid.iter()
        .map(|row| row.iter().map(|v| *v as usize).sum::<usize>() as usize)
        .sum()
}

struct Instruction {
    action: Action,
    top_left: Corner,
    bottom_right: Corner,
}
impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (prefix, bottom_right) = s.split_once(" through ").unwrap();
        let (action, top_left) = prefix.rsplit_once(' ').unwrap();
        Instruction {
            action: Action::from(action),
            top_left: Corner::from(top_left),
            bottom_right: Corner::from(bottom_right),
        }
    }
}
impl Instruction {
    fn x_range(&self) -> RangeInclusive<usize> {
        self.top_left.x..=self.bottom_right.x
    }
    fn y_range(&self) -> RangeInclusive<usize> {
        self.top_left.y..=self.bottom_right.y
    }
}

struct Corner {
    x: usize,
    y: usize,
}
impl From<&str> for Corner {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Corner {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

enum Action {
    On,
    Off,
    Toggle,
}
impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "turn on" => Action::On,
            "turn off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(400410, day06_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(15343601, day06_part2());
    }
}
