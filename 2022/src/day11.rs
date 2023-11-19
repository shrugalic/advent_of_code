use std::cmp::Reverse;

const INPUT: &str = include_str!("../input/day11.txt");

pub(crate) fn day11_part1() -> usize {
    let monkeys = parse(INPUT);
    calculate_level_of_monkey_business_part1(monkeys)
}

pub(crate) fn day11_part2() -> usize {
    let monkeys = parse(INPUT);
    calculate_level_of_monkey_business_part2(monkeys)
}

fn parse(input: &str) -> Vec<Monkey> {
    input.trim().split("\n\n").map(Monkey::from).collect()
}

type WorryLevel = usize;

enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    divisible_by: usize,
    target_if_true: usize,
    target_if_false: usize,
    inspect_counter: usize,
}
impl Monkey {
    fn inspect(&mut self) -> WorryLevel {
        self.inspect_counter += 1;
        let item = self.items.remove(0);
        match self.operation {
            Operation::Add(addend) => item + addend,
            Operation::Multiply(multiplier) => item * multiplier,
            Operation::Square => item * item,
        }
    }
    fn test(&self, item: WorryLevel) -> usize {
        if item % self.divisible_by == 0 {
            self.target_if_true
        } else {
            self.target_if_false
        }
    }
}
impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let lines: Vec<_> = s.lines().collect();
        let items: Vec<_> = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let parts: Vec<_> = lines[2]
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_ascii_whitespace()
            .collect();
        let operation = match parts[0] {
            "+" => Operation::Add(parts[1].parse().unwrap()),
            "*" => match parts[1] {
                "old" => Operation::Square,
                multiplier => Operation::Multiply(multiplier.parse().unwrap()),
            },
            op => unreachable!("Unknown operation {}", op),
        };

        fn parse_number_after_prefix(s: &str, prefix: &str) -> usize {
            s.strip_prefix(prefix).unwrap().parse().unwrap()
        }

        Monkey {
            items,
            operation,
            divisible_by: parse_number_after_prefix(lines[3], "  Test: divisible by "),
            target_if_true: parse_number_after_prefix(lines[4], "    If true: throw to monkey "),
            target_if_false: parse_number_after_prefix(lines[5], "    If false: throw to monkey "),
            inspect_counter: 0,
        }
    }
}

fn calculate_level_of_monkey_business_part1(monkeys: Vec<Monkey>) -> usize {
    calculate_level_of_monkey_business(monkeys, 20, true)
}
fn calculate_level_of_monkey_business_part2(monkeys: Vec<Monkey>) -> usize {
    calculate_level_of_monkey_business(monkeys, 10_000, false)
}
fn calculate_level_of_monkey_business(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    divide_by_3: bool,
) -> usize {
    let divisor: usize = monkeys.iter().map(|m| m.divisible_by).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let monkey = monkeys.get_mut(i).unwrap();
                let mut item = monkey.inspect();
                if divide_by_3 {
                    item /= 3;
                }
                item %= divisor; // This works for part 1 and 2
                let target = monkey.test(item);
                monkeys.get_mut(target).unwrap().items.push(item);
            }
        }
    }
    let mut counts: Vec<_> = monkeys.into_iter().map(|m| m.inspect_counter).collect();
    counts.sort_unstable_by_key(|&k| Reverse(k));
    counts.into_iter().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        let monkeys = parse(EXAMPLE);
        assert_eq!(101 * 105, calculate_level_of_monkey_business_part1(monkeys));
    }

    #[test]
    fn part2_example() {
        let monkeys = parse(EXAMPLE);
        assert_eq!(
            2_713_310_158,
            calculate_level_of_monkey_business_part2(monkeys)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(54_054, day11_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(14_314_925_001, day11_part2());
    }
}
