use std::collections::HashSet;
use std::ops::RangeInclusive;
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

struct Floor {
    y: Y,
    count: usize,
    desc: String,
}

type X = usize;
type Y = usize;
fn tower_height(directions: Vec<Direction>, rock_count: usize) -> usize {
    // Our chamber looks like this:
    // …       …
    // |       | 2
    // |       | 1
    // +-------+ 0
    // 012345678
    let initial_y = 3 + 1;
    let mut tower_height = 0;
    let mut wall_height = tower_height + initial_y - 1;
    let mut shapes = [HBar, Cross, LeftL, VBar, Square].iter().cycle();
    let mut directions = directions.iter().cycle();
    let mut extra_height = 0;

    // Init floor
    let mut floors = vec![Floor {
        y: 0,
        count: 0,
        desc: "+-------+".to_string(),
    }];

    // Init occupied positions with walls
    let mut occupied_positions: HashSet<(X, Y)> = (0..=9).into_iter().map(|x| (x, 0)).collect();
    (1..=wall_height).into_iter().for_each(|y| {
        occupied_positions.insert((0, y));
        occupied_positions.insert((8, y));
    });

    // Simulate all the rocks
    let mut count = 0;
    while count < rock_count {
        count += 1;
        let mut rock = Rock {
            shape: shapes.next().unwrap(),
            left: 2 + 1,
            bottom: tower_height + initial_y,
        };

        // Extend walls to cover the rock height for collision tests
        let old_wall_height = wall_height;
        wall_height = tower_height + initial_y - 1 + rock.shape.height();
        (old_wall_height + 1..=wall_height)
            .into_iter()
            .for_each(|y| {
                occupied_positions.insert((0, y));
                occupied_positions.insert((8, y));
            });

        // Let the rock do it's thing until it can't drop any more
        loop {
            // draw(&rock, &occupied_positions);
            let direction = directions.next().unwrap();

            let can_move = !rock.offset_by(direction).overlaps(&occupied_positions);
            if can_move {
                rock.move_in(direction);
            }

            let can_fall = !rock.dropped_by_1().overlaps(&occupied_positions);
            if can_fall {
                rock.drop_1_unit();
            } else {
                break;
            }
        }

        // Store the landed rock in occupied positions
        for pos in rock.occupied_positions() {
            occupied_positions.insert(pos);
        }

        // Recalculate new tower height
        tower_height = occupied_positions
            .iter()
            .filter(|(x, _)| (1..8).contains(x))
            .map(|&(_, y)| y)
            .max()
            .unwrap();

        // Check for new floors, where rocks cover the full width of the chamber
        let last_floor_y = *floors.last().map(|Floor { y, .. }| y).unwrap();
        if let Some(y) = (last_floor_y + 1..=tower_height).into_iter().find(|y| {
            (1..8)
                .into_iter()
                .all(|x| occupied_positions.contains(&(x, *y)))
        }) {
            let desc = gen_floor_desc(&occupied_positions, last_floor_y..=y);
            if extra_height == 0 {
                if let Some(prev) = floors.iter().find(|Floor { desc: d, .. }| desc.eq(d)) {
                    let period = count - prev.count;
                    let floor_height = y - prev.y;

                    let mut multi = 0;
                    let old_count = count;
                    while count + period < rock_count {
                        count += period;
                        multi += 1;
                    }
                    extra_height = multi * floor_height;
                    println!(
                        "Floor @ {y} with count {old_count} is a repeat of floor @ {} with count {}",
                        prev.y, prev.count
                    );
                    println!(
                        "Period {period}, floor_height {floor_height}, extra_height {extra_height}"
                    );

                    // println!("floor:\n{}\n", desc);
                    // draw(&rock, &occupied_positions);
                }
                floors.push(Floor { y, desc, count });
            }
        }
    }

    // dbg!(floors.iter().map(|Floor { y, .. }| y).collect::<Vec<_>>());
    // if !floors.is_empty() {
    //     let diffs: Vec<_> = floors.windows(2).map(|a| a[1].y - a[0].y).collect();
    //     for Floor { y, desc, count } in floors {
    //         println!("floor at {y} with {count}:\n{desc}\n");
    //     }
    // }
    tower_height + extra_height
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
            .rev()
            .map(|y| {
                (0..9usize)
                    .into_iter()
                    .map(|x| {
                        if rock.occupied_positions().any(|pos| pos == (x, y)) {
                            '@'
                        } else if occupied_positions.contains(&(x, y)) {
                            match (x, y) {
                                (0, 0) | (8, 0) => '+',
                                (0, _) | (8, _) => '|',
                                (_, 0) => '-',
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

fn gen_floor_desc(occupied_positions: &HashSet<(X, Y)>, y_range: RangeInclusive<Y>) -> String {
    y_range
        .into_iter()
        .rev()
        .map(|y| {
            (0..9usize)
                .into_iter()
                .map(|x| {
                    if occupied_positions.contains(&(x, y)) {
                        match (x, y) {
                            (0, 0) | (8, 0) => '+',
                            (0, _) | (8, _) => '|',
                            (_, 0) => '-',
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

    #[test]
    fn part2_example() {
        let directions = parse(EXAMPLE);
        assert_eq!(1514285714288, tower_height(directions, P2_ROUNDS));
    }

    #[test]
    fn part2() {
        assert_eq!(1_523_615_160_362, day17_part2());
    }
}
