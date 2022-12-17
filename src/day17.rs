use std::collections::HashSet;
use Direction::*;
use Shape::*;

const INPUT: &str = include_str!("../input/day17.txt");
const P1_ROUNDS: usize = 2022;
const P2_ROUNDS: usize = 1_000_000_000_000;

pub(crate) fn day17_part1() -> usize {
    let directions = parse(INPUT);
    tower_height(directions, P1_ROUNDS)
}

pub(crate) fn day17_part2() -> usize {
    let directions = parse(INPUT);
    tower_height(directions, P2_ROUNDS)
}

type X = usize;
type Y = usize;
fn tower_height(directions: Vec<Direction>, rock_count: usize) -> usize {
    let initial_x = 2 + 1;
    let initial_y = 3 + 1;
    let mut tower_top = 0;
    let mut wall_height = tower_top + initial_y - 1;
    let mut shapes = [HBar, Cross, LeftL, VBar, Square].iter().cycle();
    let mut directions = directions.iter().cycle();

    let mut occupied_positions: HashSet<(X, Y)> = (0..=9).into_iter().map(|x| (x, 0)).collect();
    (1..=wall_height).into_iter().for_each(|y| {
        occupied_positions.insert((0, y));
        occupied_positions.insert((8, y));
    });

    // |       |
    // +-------+
    // 012345678
    for _ in 0..rock_count {
        let mut rock = Rock {
            shape: shapes.next().unwrap(),
            left: initial_x,
            bottom: tower_top + initial_y,
        };

        // Extend walls for collision tests
        let old_wall_height = wall_height;
        wall_height = tower_top + initial_y - 1 + rock.shape.height();
        (old_wall_height + 1..=wall_height)
            .into_iter()
            .for_each(|y| {
                occupied_positions.insert((0, y));
                occupied_positions.insert((8, y));
            });

        loop {
            // draw(&rock, &occupied_positions);

            let direction = directions.next().unwrap();
            if !rock.offset_by(direction).overlaps(&occupied_positions) {
                rock.move_in(direction);
            }

            let can_drop = rock.bottom > 0 && !rock.dropped_by_1().overlaps(&occupied_positions);
            if can_drop {
                rock.drop_1_unit();
            } else {
                break;
            }
        }
        // Store shape
        for pos in rock.occupied_positions() {
            occupied_positions.insert(pos);
        }
        // Recalculate tower height
        tower_top = occupied_positions
            .iter()
            .filter(|(x, _)| (1..8).contains(x))
            .map(|&(_, y)| y)
            .max()
            .unwrap();
    }
    tower_top
}

#[allow(unused)]
fn draw(rock: &Rock, occupied_positions: &HashSet<(X, Y)>) {
    let max_y = occupied_positions
        .iter()
        .map(|(_, y)| *y)
        .chain(rock.occupied_positions().map(|(_, y)| y))
        .max()
        .unwrap_or(0);

    println!(
        "\n{}",
        (0..=max_y)
            .into_iter()
            .map(|top| {
                let y = max_y - top;
                (0..9usize)
                    .into_iter()
                    .map(|x| {
                        if rock.occupied_positions().any(|pos| pos == (x, y)) {
                            '@'
                        } else if occupied_positions.contains(&(x, y)) {
                            match (x, y) {
                                (0, 0) | (8, 0) => '+',
                                (_, 0) => '-',
                                (0, _) | (8, _) => '|',
                                _ => '#',
                            }
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct Rock<'a> {
    shape: &'a Shape,
    // Position
    left: usize,
    bottom: usize,
}
impl<'a> Rock<'a> {
    fn right(&self) -> usize {
        self.left + self.shape.width() - 1
    }
    fn move_in(&mut self, direction: &Direction) {
        match direction {
            Left => self.left -= 1,
            Right => self.left += 1,
        }
    }
    fn drop_1_unit(&mut self) {
        self.bottom -= 1;
    }
    fn dropped_by_1(&self) -> Self {
        let mut clone = self.clone();
        clone.drop_1_unit();
        clone
    }
    fn offset_by(&self, direction: &Direction) -> Self {
        let mut clone = self.clone();
        clone.move_in(direction);
        clone
    }

    fn offsets(&self) -> Vec<(X, Y)> {
        // Offsets of its pixels compared to its bottom left corner (considered (0,0)
        match self.shape {
            HBar => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Cross => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            LeftL => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            VBar => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
    fn occupied_positions(&self) -> impl Iterator<Item = (X, Y)> + '_ {
        self.offsets()
            .into_iter()
            .map(|(x, y)| (self.left + x, self.bottom + y))
    }
    fn overlaps(&self, occupied_positions: &HashSet<(X, Y)>) -> bool {
        self.occupied_positions()
            .any(|pos| occupied_positions.contains(&pos))
    }
}

enum Shape {
    HBar,
    Cross,
    LeftL,
    VBar,
    Square,
}
impl Shape {
    fn width(&self) -> usize {
        match self {
            HBar => 4,
            Cross => 3,
            LeftL => 3,
            VBar => 1,
            Square => 2,
        }
    }
    fn height(&self) -> usize {
        match self {
            HBar => 1,
            Cross => 3,
            LeftL => 3,
            VBar => 4,
            Square => 2,
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .trim()
        .chars()
        .map(|c| if c == '<' { Left } else { Right })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        let directions = parse(EXAMPLE);
        assert_eq!(3_068, tower_height(directions, P1_ROUNDS));
    }

    #[test]
    fn part1() {
        assert_eq!(3_071, day17_part1());
    }

    #[ignore]
    #[test]
    fn part2_example() {
        let directions = parse(EXAMPLE);
        assert_eq!(1514285714288, tower_height(directions, P2_ROUNDS));
    }

    #[ignore]
    #[test]
    fn part2() {
        assert_eq!(1, day17_part2());
    }
}
