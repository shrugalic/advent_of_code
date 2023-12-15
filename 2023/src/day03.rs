use std::collections::HashMap;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day03.txt");

pub(crate) fn part1() -> usize {
    sum_of_part_numbers_adjacent_to_symbols(INPUT)
}

pub(crate) fn part2() -> usize {
    sum_of_gear_ratios(INPUT)
}

fn sum_of_part_numbers_adjacent_to_symbols(input: &str) -> usize {
    let (symbols, part_numbers) = parse_engine_schematic(input);
    part_numbers
        .into_iter()
        .filter(|number| number.is_adjacent_to_any(&symbols))
        .map(|number| number.value)
        .sum()
}

fn sum_of_gear_ratios(input: &str) -> usize {
    let (symbols, part_numbers) = parse_engine_schematic(input);
    symbols
        .iter()
        .filter_map(|(position, &symbol)| {
            let adjacent_part_numbers = position.to_adjacent(&part_numbers);
            let is_gear = symbol == '*' && adjacent_part_numbers.len() == 2;
            if is_gear {
                let gear_ratio = adjacent_part_numbers[0] * adjacent_part_numbers[1];
                Some(gear_ratio)
            } else {
                None
            }
        })
        .sum()
}

fn parse_engine_schematic(input: &str) -> (Symbols, PartNumbers) {
    let mut symbols = Symbols::new();
    let mut part_numbers = PartNumbers::new();
    for (y, line) in input.lines().enumerate() {
        let mut accumulating_part_number: Option<NumberPosition> = None;
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_punctuation() && c != '.' {
                symbols.insert(Position::new(x, y), c);
            }
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                accumulating_part_number = Some(match accumulating_part_number.take() {
                    Some(partial_part_number) => partial_part_number.append(digit),
                    None => NumberPosition::new(x, y, digit),
                });
            } else if let Some(complete_part_number) = accumulating_part_number.take() {
                part_numbers.push(complete_part_number);
            };
        }
        if let Some(complete_part_number) = accumulating_part_number.take() {
            part_numbers.push(complete_part_number);
        }
    }
    (symbols, part_numbers)
}

type X = isize;
type Y = isize;
#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: X,
    y: Y,
}
type Symbol = char;
type Symbols = HashMap<Position, Symbol>;
struct DigitPositions {
    x_range: RangeInclusive<X>,
    y: Y,
}
type PartNumber = usize;
struct NumberPosition {
    pos: DigitPositions,
    value: PartNumber,
}
type PartNumbers = Vec<NumberPosition>;

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {
            x: x as X,
            y: y as Y,
        }
    }
    fn to_adjacent(&self, part_numbers: &PartNumbers) -> Vec<PartNumber> {
        part_numbers
            .iter()
            .filter(|number| number.is_adjacent_to(self))
            .map(|number| number.value)
            .collect()
    }
}

impl DigitPositions {
    fn new(x: X, y: Y) -> Self {
        Self { x_range: x..=x, y }
    }
    fn adjacent_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (self.x_range.start() - 1..=self.x_range.end() + 1).flat_map(|x| {
            [
                Position { x, y: self.y - 1 },
                Position { x, y: self.y },
                Position { x, y: self.y + 1 },
            ]
        })
    }
    fn extend(self) -> Self {
        DigitPositions {
            x_range: *self.x_range.start()..=self.x_range.end() + 1,
            y: self.y,
        }
    }
}

impl NumberPosition {
    fn new(x: usize, y: usize, value: u32) -> Self {
        NumberPosition {
            pos: DigitPositions::new(x as X, y as Y),
            value: value as PartNumber,
        }
    }
    fn append(mut self, digit: u32) -> Self {
        self.pos = self.pos.extend();
        self.value = self.value * 10 + digit as PartNumber;
        self
    }
    fn is_adjacent_to_any(&self, symbols: &Symbols) -> bool {
        self.pos
            .adjacent_positions()
            .any(|position: Position| symbols.contains_key(&position))
    }
    fn is_adjacent_to(&self, pos: &Position) -> bool {
        (self.pos.x_range.start() - 1..=self.pos.x_range.end() + 1).contains(&pos.x)
            && (self.pos.y - 1..=self.pos.y + 1).contains(&pos.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1_example() {
        assert_eq!(4_361, sum_of_part_numbers_adjacent_to_symbols(EXAMPLE));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(467_835, sum_of_gear_ratios(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(520_019, sum_of_part_numbers_adjacent_to_symbols(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(75_519_888, sum_of_gear_ratios(INPUT));
    }
}
