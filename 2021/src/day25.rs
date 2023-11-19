use std::fmt::{Display, Formatter};
use Tile::*;

const INPUT: &str = include_str!("../input/day25.txt");

pub(crate) fn day25_part1() -> usize {
    SeaCucumbers::from(INPUT).first_step_no_cucumbers_moved()
}

impl SeaCucumbers {
    fn first_step_no_cucumbers_moved(&mut self) -> usize {
        let mut steps = 1;
        while self.moved_in_step() {
            steps += 1;
        }
        steps
    }
    fn moved_in_step(&mut self) -> bool {
        let moved_east = self.move_east();
        let moved_south = self.move_south();
        moved_east || moved_south
    }
    fn move_east(&mut self) -> bool {
        let width = self.width();
        let mut moved = false;

        for y in 0..self.height() {
            let mut x = 0;
            let is_first_empty = self.grid[y][x] == Empty;
            while x < width {
                let next_x = (x + 1) % width;
                if self.grid[y][x] == East
                    && (next_x > 0 && self.grid[y][next_x] == Empty
                    // Last cucumber may not move if first position wasn't empty before
                    || next_x == 0 && is_first_empty)
                {
                    self.grid[y].swap(x, next_x);
                    moved = true;
                    x += 1; // Do not move this cucumber again right away
                }
                x += 1;
            }
        }
        moved
    }
    fn move_south(&mut self) -> bool {
        let height = self.height();
        let mut moved = false;

        for x in 0..self.width() {
            let mut y = 0;
            let is_first_empty = self.grid[y][x] == Empty;
            while y < height {
                let next_y = (y + 1) % height;
                if self.grid[y][x] == South
                    && (next_y > 0 && self.grid[next_y][x] == Empty
                    // Last cucumber may not move if first position wasn't empty before
                    || next_y == 0 && is_first_empty)
                {
                    self.grid[y][x] = Empty;
                    self.grid[next_y][x] = South;
                    moved = true;
                    y += 1; // Do not move this cucumber again right away
                }
                y += 1;
            }
        }
        moved
    }
    fn width(&mut self) -> usize {
        self.grid[0].len()
    }
    fn height(&mut self) -> usize {
        self.grid.len()
    }
}
struct SeaCucumbers {
    grid: Vec<Vec<Tile>>,
}
impl From<&str> for SeaCucumbers {
    fn from(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        SeaCucumbers { grid }
    }
}
impl Display for SeaCucumbers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(Tile::to_string).collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(PartialEq, Clone)]
enum Tile {
    East,
    South,
    Empty,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '>' => East,
            'v' => South,
            '.' => Empty,
            _ => unreachable!("Illegal char '{}'", c),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                East => '>',
                South => 'v',
                Empty => '.',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            58,
            SeaCucumbers::from(EXAMPLE).first_step_no_cucumbers_moved()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(598, day25_part1());
    }

    const EXAMPLE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn part1_example_step_by_step() {
        let mut sea_cucumbers = SeaCucumbers::from(EXAMPLE);

        sea_cucumbers.moved_in_step(); // 1
        assert_eq!(
            sea_cucumbers.to_string(),
            "\
....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"
        );

        sea_cucumbers.moved_in_step(); // 2
        assert_eq!(
            sea_cucumbers.to_string(),
            "\
>.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv."
        );

        sea_cucumbers.moved_in_step();
        assert_eq!(
            sea_cucumbers.to_string(),
            "\
v>v.v>.>v.
v...>>.v.v
>vv>.>v>..
>>v>v.>.v>
..>....v..
.>.>v>v..v
..v..v>vv>
v.v..>>v..
.v>....v.."
        );

        sea_cucumbers.moved_in_step();
        assert_eq!(
            sea_cucumbers.to_string(),
            "\
v>..v.>>..
v.v.>.>.v.
>vv.>>.v>v
>>.>..v>.>
..v>v...v.
..>>.>vv..
>.v.vv>v.v
.....>>vv.
vvv>...v.."
        );

        sea_cucumbers.moved_in_step(); // 5
        assert_eq!(
            sea_cucumbers.to_string(),
            "\
vv>...>v>.
v.v.v>.>v.
>.v.>.>.>v
>v>.>..v>>
..v>v.v...
..>.>>vvv.
.>...v>v..
..v.v>>v.v
v.v.>...v."
        );

        sea_cucumbers.moved_in_step(); // 6
        sea_cucumbers.moved_in_step();
        sea_cucumbers.moved_in_step();
        sea_cucumbers.moved_in_step();
        sea_cucumbers.moved_in_step(); // 10
        assert_eq!(
            sea_cucumbers.to_string(),
            "\
..>..>>vv.
v.....>>.v
..v.v>>>v>
v>.>v.>>>.
..v>v.vv.v
.v.>>>.v..
v.v..>v>..
..v...>v.>
.vv..v>vv."
        );
    }
}
