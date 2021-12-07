use std::mem::swap;

const INPUT: &str = include_str!("../input/day07.txt");

pub(crate) fn day07_part1() -> usize {
    let input = parse(INPUT);
    minimal_fuel_to_align(input, false)
}

pub(crate) fn day07_part2() -> usize {
    let positions = parse(INPUT);
    minimal_fuel_to_align(positions, true)
}

fn minimal_fuel_to_align(positions: Vec<isize>, increasing_cost: bool) -> usize {
    // Start at the average position
    let mut center = positions.iter().sum::<isize>() / positions.len() as isize;
    let mut prev_cost = fuel_to_align_to(center, &positions, increasing_cost);

    // Determine the direction of decreasing fuel cost. Try going right
    let mut dir = 1;
    center += dir;
    let mut cost = fuel_to_align_to(center, &positions, increasing_cost);

    // If going right increased the fuel cost, go left instead
    if cost > prev_cost {
        swap(&mut prev_cost, &mut cost);
        dir = -1;
        center += dir;
    };

    // Find the minimum
    while cost < prev_cost {
        center += dir;
        prev_cost = cost;
        cost = fuel_to_align_to(center, &positions, increasing_cost);
    }
    prev_cost
}

fn fuel_to_align_to(center: isize, positions: &[isize], increasing_cost: bool) -> usize {
    positions
        .iter()
        .map(|pos| {
            let distance = (center - pos).abs() as usize;
            if increasing_cost {
                distance * (distance + 1) / 2
            } else {
                distance
            }
        })
        .sum()
}

fn parse(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example() {
        assert_eq!(37, minimal_fuel_to_align(parse(EXAMPLE), false));
    }

    #[test]
    fn part2_example() {
        let input = parse(EXAMPLE);
        assert_eq!(168, minimal_fuel_to_align(input, true));
    }

    #[test]
    fn part1() {
        assert_eq!(day07_part1(), 348_996);
    }

    #[test]
    fn part2() {
        assert_eq!(day07_part2(), 98_231_647);
    }
}
