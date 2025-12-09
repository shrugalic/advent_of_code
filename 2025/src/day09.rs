use crate::day09::Rotation::{ClockWise, CounterClockWise};
use crate::vec_2d::Vec2D;
use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use Direction::*;
use Turn::*;

const INPUT: &str = include_str!("../../2025/input/day09.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let positions = parse_red_tile_positions(input);
    let mut max_area = 0;
    for i in 0..positions.len() - 1 {
        let pos1 = &positions[i];
        for pos2 in positions.iter().skip(i) {
            let area = area(pos1, pos2);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn solve_part2(input: &str) -> usize {
    let positions = parse_red_tile_positions(input);
    let xs: HashSet<_> = positions.iter().map(|p| p.x).collect();
    let ys: HashSet<_> = positions.iter().map(|p| p.y).collect();

    // Pair up adjacent positions, wrapping around at the end
    // (so the last pair contains the last and first position)
    let position_pairs: Vec<_> = (0..positions.len())
        .map(|i| {
            let n = (i + 1) % positions.len();
            (positions[i], positions[n])
        })
        .collect();

    // Collect all positions that are on the border of the polygon, including the corners
    let border_positions: HashSet<Vec2D> = position_pairs
        .iter()
        .flat_map(|(pos1, pos2)| {
            if pos1.x == pos2.x {
                (isize::min(pos1.y, pos2.y)..=isize::max(pos1.y, pos2.y))
                    .map(|y| Vec2D { x: pos1.x, y })
                    .collect::<Vec<_>>()
            } else {
                debug_assert_eq!(pos1.y, pos2.y);
                (isize::min(pos1.x, pos2.x)..=isize::max(pos1.x, pos2.x))
                    .map(|x| Vec2D { x, y: pos1.y })
                    .collect()
            }
        })
        .collect();
    // dbg!(border_positions.len()); // 583'688

    // Collect the directions from one position to the next
    let directions: Vec<_> = position_pairs
        .iter()
        .flat_map(|(pos1, pos2)| Direction::try_from((pos1, pos2)))
        .collect();

    // Collect the turns between three adjacent positions
    let turns: Vec<_> = (0..directions.len())
        .map(|i| {
            let p = (i as isize - 1).rem_euclid(directions.len() as isize) as usize;
            (directions[p], directions[i])
        })
        .flat_map(|(dir1, dir2)| Turn::try_from((&dir1, &dir2)))
        .collect();

    // Determine whether the inside of the polygon is on the right or left side of the path.
    let turn_sum = turns
        .iter()
        .map(|turn| match turn {
            Left => 1,
            Right => -1,
        })
        .sum::<isize>();
    // There will be 4 more turns in one direction: toward the inside of the polygon
    debug_assert_eq!(4, turn_sum.abs());
    let rotation = if turn_sum < 0 {
        ClockWise
    } else {
        CounterClockWise
    };

    // Construct a set of all positions directly outside the polygon
    let outside_positions: HashSet<Vec2D> = position_pairs
        .iter()
        .enumerate()
        .flat_map(|(i, (start, end))| {
            let mut pos = *start;
            let direction = directions[i];
            let increment = match direction {
                North => Vec2D::NORTH,
                South => Vec2D::SOUTH,
                West => Vec2D::WEST,
                East => Vec2D::EAST,
            };
            let mut outside_positions = vec![];
            while &pos != end {
                let outside_pos = outside_pos(&pos, &rotation, &direction);
                if !border_positions.contains(&outside_pos) {
                    outside_positions.push(outside_pos);
                }
                pos += increment;
            }
            // Add last pos as well
            let outside_pos = outside_pos(&pos, &rotation, &direction);
            if !border_positions.contains(&outside_pos) {
                outside_positions.push(outside_pos);
            }
            pos += increment;

            outside_positions
        })
        .collect();
    // dbg!(&outside_positions.len()); // 583'446

    // To determine whether a position is inside or outside the polygon,
    // cast a ray in any directions until we hit something, either the polygon border,
    // an position in the outside band, or the bounding box
    let polygon_contains = |pos: &Vec2D| -> bool {
        // check left (same y coordinate)
        let left_filter = |other: &Vec2D| -> Option<isize> {
            (other.y == pos.y && other.x < pos.x).then_some(other.x)
        };
        if let Some(left_border_x) = border_positions.iter().filter_map(left_filter).max() {
            if let Some(left_outside_x) = outside_positions.iter().filter_map(left_filter).max() {
                // Did we hit the border first, or the outside band?
                left_outside_x < left_border_x
            } else {
                // Hitting a border but not the outside band is impossible, I think
                unreachable!()
            }
        } else {
            false // Nothing in the way; would hit the bounding box
        }
    };

    // Fill a cache to speed up repeat lookups of the same position.
    // And initialized with known values from the border as well as the outside band around it.
    let mut is_inside_by_pos: HashMap<Vec2D, bool> =
        border_positions.iter().map(|pos| (*pos, true)).collect();
    outside_positions.iter().for_each(|pos| {
        is_inside_by_pos.insert(*pos, false);
    });
    // dbg!(&is_inside_by_pos.len()); // 1'167'134

    let mut is_position_inside_polygon = |pos: &Vec2D| -> bool {
        *is_inside_by_pos
            .entry(*pos)
            .or_insert_with(|| polygon_contains(pos))
    };

    // Check if the rectangle defined by the two corners `a` and`b` is completely the polygon.
    let mut is_rectangle_inside_polygon = |a: &Vec2D, b: &Vec2D| -> bool {
        // Rectangles where the two corners are in a straight line are inside by definition
        if a.x == b.x || a.y == b.y {
            return true;
        }

        // For other rectangles, *all* border positions must be inside the polygon

        // Let's check just the other two corners first
        let c = Vec2D { x: a.x, y: b.y };
        if !is_position_inside_polygon(&c) {
            return false;
        }
        let d = Vec2D { x: b.x, y: a.y };
        if !is_position_inside_polygon(&d) {
            return false;
        }

        // And now check the relevant positions between the corners
        // Note: Not all positions need to be checked, only those x/y coordinates that are
        // next to a red tile, because only there could the inside/outside condition be violated.
        (isize::min(a.y, b.y) + 1..isize::max(a.y, b.y))
            .filter_map(|y| ys.contains(&y).then_some([y - 1, y + 1]))
            .flatten()
            .all(|y| {
                is_position_inside_polygon(&Vec2D { x: a.x, y })
                    && is_position_inside_polygon(&Vec2D { x: b.x, y })
            })
            && (isize::min(a.x, b.x) + 1..isize::max(a.x, b.x))
                .filter_map(|x| xs.contains(&x).then_some([x - 1, x + 1]))
                .flatten()
                .all(|x| {
                    is_position_inside_polygon(&Vec2D { x, y: a.y })
                        && is_position_inside_polygon(&Vec2D { x, y: b.y })
                })
    };

    let mut index_pairs: Vec<_> = (0..positions.len() - 1)
        .flat_map(|i| (i + 1..positions.len()).map(move |j| (i, j)))
        .collect();

    // Sort pairs by descending (potential) area so we can stop at the first valid one
    index_pairs.sort_unstable_by_key(|(i, j)| -(area(&positions[*i], &positions[*j]) as isize));
    index_pairs
        .into_iter()
        .find_map(|(i, j)| {
            let pos1 = &positions[i];
            let pos2 = &positions[j];
            is_rectangle_inside_polygon(pos1, pos2).then_some(area(pos1, pos2))
        })
        .unwrap()
}

// Area of a rectangle defined by the two corners
fn area(a: &Vec2D, b: &Vec2D) -> usize {
    (((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)) as usize
}

// For clockwise rotation, the left is outside. And right for counter-clockwise.
fn outside_pos(curr: &Vec2D, rotation: &Rotation, dir: &Direction) -> Vec2D {
    match (rotation, dir) {
        (ClockWise, North) | (CounterClockWise, South) => curr.left_neighbor(),
        (ClockWise, South) | (CounterClockWise, North) => curr.right_neighbor(),
        (ClockWise, West) | (CounterClockWise, East) => curr.below_neighbor(),
        (ClockWise, East) | (CounterClockWise, West) => curr.above_neighbor(),
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl TryFrom<(&Vec2D, &Vec2D)> for Direction {
    type Error = ();

    fn try_from((pos1, pos2): (&Vec2D, &Vec2D)) -> Result<Self, Self::Error> {
        match (pos1.x.cmp(&pos2.x), pos1.y.cmp(&pos2.y)) {
            (Less, Equal) => Ok(East),
            (Equal, Less) => Ok(South),
            (Equal, Greater) => Ok(North),
            (Greater, Equal) => Ok(West),
            (_, _) => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl TryFrom<(&Direction, &Direction)> for Turn {
    type Error = String;

    fn try_from((dir1, dir2): (&Direction, &Direction)) -> Result<Self, Self::Error> {
        match (dir1, dir2) {
            (North, West) => Ok(Left),
            (North, East) => Ok(Right),
            (East, North) => Ok(Left),
            (East, South) => Ok(Right),
            (South, East) => Ok(Left),
            (South, West) => Ok(Right),
            (West, South) => Ok(Left),
            (West, North) => Ok(Right),
            (North, North) | (East, East) | (South, South) | (West, West) => {
                Err("went straight, no turn".to_string())
            }
            (North, South) | (South, North) | (East, West) | (West, East) => {
                Err("180 degree turn".to_string())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Rotation {
    ClockWise,
    CounterClockWise,
}

fn parse_red_tile_positions(input: &str) -> Vec<Vec2D> {
    input
        .trim()
        .lines()
        .flat_map(|line| Vec2D::from_str(line).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part1_example() {
        assert_eq!(50, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(4750092396, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(24, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(
            33,
            solve_part2(
                "\
1,0
3,0
3,5
16,5
16,0
18,0
18,9
13,9
13,7
6,7
6,9
1,9"
            )
        );
    }

    #[test]
    fn test_part2() {
        // 4616852656 is too high
        assert_eq!(1468516555, solve_part2(INPUT));
    }
}
