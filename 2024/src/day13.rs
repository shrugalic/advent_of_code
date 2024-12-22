use crate::vec_2d::Vec2D;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../../2024/input/day13.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter_map(ClawMachine::find_solution_by_trial_and_error)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(ClawMachine::move_prize_for_part_2)
        .filter_map(ClawMachine::calculate_solution)
        .sum()
}

#[derive(Debug)]
struct ClawMachine {
    a: Vec2D,
    b: Vec2D,
    prize: Vec2D,
}

impl ClawMachine {
    fn find_solution_by_trial_and_error(&self) -> Option<usize> {
        // For the first part, there are at most 100 button presses
        let a_terms: Vec<_> = (0..=100)
            .map(|a| (a, a * self.a.x, a * self.a.y))
            .take_while(|(_a, ax, ay)| ax <= &self.prize.x && ay <= &self.prize.y)
            .collect();
        let b_terms: Vec<_> = (0..=100)
            .map(|b| (b, b * self.b.x, b * self.b.y))
            .take_while(|(_b, bx, by)| bx <= &self.prize.x && by <= &self.prize.y)
            .collect();
        for (a, a_x, a_y) in a_terms {
            for (b, b_x, b_y) in &b_terms {
                match (
                    self.prize.x.cmp(&(a_x + b_x)),
                    self.prize.y.cmp(&(a_y + b_y)),
                ) {
                    (Ordering::Less, _) | (_, Ordering::Less) => {
                        // Right side is too large, stop increasing b
                        break;
                    }
                    (Ordering::Equal, Ordering::Equal) => {
                        let cost = (3 * a + b) as usize;
                        return Some(cost);
                    }
                    _ => {}
                }
            }
        }
        None
    }
    fn calculate_solution(self) -> Option<usize> {
        // There are two equations to solve:
        // a * a.x + b * b.x = prize.x
        // a * a.y + b * b.y = prize.y
        // Let's multiply the first equation with a.y, and the second with a.x:
        // a * a.x * a.y + b * b.x * a.y = prize.x * a.y
        // a * a.y * a.x + b * b.y * a.x = prize.y * a.x
        // Then subtract the second from the first to eliminate a:
        // b * (b.x * a.y - b.y * a.x) = prize.x * a.y - prize.y * a.x
        // And solve for b:
        // b = (prize.x * a.y - prize.y * a.x) / (b.x * a.y - b.y * a.x)
        let numerator = self.prize.x * self.a.y - self.prize.y * self.a.x;
        let denominator = self.b.x * self.a.y - self.b.y * self.a.x;
        // In this exercise, b must be a positive integer
        if numerator % denominator == 0 && numerator.signum() == denominator.signum() {
            let b = numerator / denominator;
            // Now let's solve for a using the first equation
            // a * a.x + b * b.x = prize.x
            // a * a.x = prize.x - b * b.x
            // a = (prize.x - b * b.x) / a.x
            // Likewise, a must be positive and an integer
            let numerator = self.prize.x - b * self.b.x;
            if numerator > 0 && numerator % self.a.x == 0 {
                let a = numerator / self.a.x;
                return Some((3 * a + b) as usize);
            }
        }
        None
    }
    fn move_prize_for_part_2(mut self) -> Self {
        self.prize.x += 10_000_000_000_000;
        self.prize.y += 10_000_000_000_000;
        self
    }
}

fn parse(input: &str) -> Vec<ClawMachine> {
    input
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(ClawMachine::from)
        .collect()
}

impl From<&[&str]> for ClawMachine {
    fn from(lines: &[&str]) -> Self {
        let button_from = |line: &str| {
            let parts = line.split(['+', ',', ' ']).collect::<Vec<_>>();
            Vec2D::new(parts[3].parse().unwrap(), parts[6].parse().unwrap())
        };
        let a = button_from(lines[0]);
        let b = button_from(lines[1]);
        let parts = lines[2].split(['=', ',', ' ']).collect::<Vec<_>>();
        let prize = Vec2D::new(parts[2].parse().unwrap(), parts[5].parse().unwrap());

        ClawMachine { a, b, prize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part1_example() {
        assert_eq!(480, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(33_921, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(875_318_608_908, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(82_261_957_837_868, solve_part2(INPUT));
    }
}
