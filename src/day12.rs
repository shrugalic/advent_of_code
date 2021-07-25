use crate::day12::Pot::HasPlant;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
enum Pot {
    HasPlant,
    IsEmpty,
}

impl Pot {
    fn has_plant(&self) -> bool {
        match self {
            HasPlant => true,
            Pot::IsEmpty => false,
        }
    }
}
impl From<char> for Pot {
    fn from(c: char) -> Self {
        match c {
            '#' => Pot::HasPlant,
            '.' => Pot::IsEmpty,
            c => panic!("Invalid state '{}'", c),
        }
    }
}
impl ToString for Pot {
    fn to_string(&self) -> String {
        match self {
            HasPlant => "#".to_string(),
            Pot::IsEmpty => ".".to_string(),
        }
    }
}

struct Pots {
    pots: VecDeque<Pot>,
    negative_pot_count: usize,
}
impl Pots {
    pub fn iterate(mut self, rules: &Rules) -> Pots {
        self.pots.make_contiguous();
        self.pots = self
            .pots
            .as_slices()
            .0 // Everything is in here because of the make_contiguous() call above
            .windows(5)
            .map(|c| rules.result_for_combination(c))
            .collect();
        // The new left-most position is the center of a 5-wide window, and
        // thus two positions to the right of the previous left-most position:
        // previous: [-4 -3 -2 -1 0 … ]
        // new:            [-2 -1 0 … ]
        self.negative_pot_count -= 2;
        self.ensure_4_empty_pots_at_both_ends()
    }

    fn ensure_4_empty_pots_at_both_ends(mut self) -> Pots {
        while self.pots.iter().take(4).any(Pot::has_plant) {
            self.pots.push_front(Pot::IsEmpty);
            self.negative_pot_count += 1;
        }
        while self.pots.iter().rev().take(4).any(Pot::has_plant) {
            self.pots.push_back(Pot::IsEmpty);
        }
        self
    }
    pub fn sum_of_pot_numbers(&self) -> isize {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, pot)| pot.has_plant())
            .map(|(idx, _)| idx as isize - self.negative_pot_count as isize)
            .sum()
    }
}
impl From<&str> for Pots {
    fn from(initial_state: &str) -> Self {
        let pots: VecDeque<Pot> = initial_state.chars().map(Pot::from).collect();
        let pots = Pots {
            pots,
            negative_pot_count: 0,
        };
        pots.ensure_4_empty_pots_at_both_ends()
    }
}
impl ToString for Pots {
    fn to_string(&self) -> String {
        self.pots.iter().map(Pot::to_string).collect::<String>()
    }
}

type Combination<'a> = &'a [&'a Pot; 5];
/// Convert a Combination of Pots into a number between 0 and 32
fn combination_to_number(combination: Combination) -> usize {
    let binary_string = combination
        .iter()
        .map(|c| match c {
            Pot::HasPlant => '1',
            Pot::IsEmpty => '0',
        })
        .collect::<String>();
    usize::from_str_radix(&binary_string, 2).unwrap()
}

struct Rules {
    plant_producing_inputs: [Pot; 32],
}
impl Rules {
    fn result_for_combination(&self, pots: &[Pot]) -> Pot {
        assert_eq!(pots.len(), 5);
        let combination = &[&pots[0], &pots[1], &pots[2], &pots[3], &pots[4]];
        self.plant_producing_inputs[combination_to_number(combination)]
    }
}
impl ToString for Rules {
    fn to_string(&self) -> String {
        self.plant_producing_inputs
            .iter()
            .map(Pot::to_string)
            .collect::<String>()
    }
}
impl From<&[String]> for Rules {
    fn from(notes: &[String]) -> Self {
        let mut plant_producing_inputs = [Pot::IsEmpty; 32];
        notes.iter().for_each(|note: &String| {
            // Example note:
            // ...## => #
            let pots = note[..5].chars().map(Pot::from).collect::<Vec<_>>();
            let combination = &[&pots[0], &pots[1], &pots[2], &pots[3], &pots[4]];
            let result = Pot::from(note[9..].chars().next().unwrap());
            if matches!(result, Pot::HasPlant) {
                let index = combination_to_number(combination);
                plant_producing_inputs[index] = Pot::HasPlant;
            }
        });
        Rules {
            plant_producing_inputs,
        }
    }
}

pub(crate) fn number_of_plants_after_20_gens(input: &[String]) -> isize {
    let mut pots = Pots::from(input[0].strip_prefix("initial state: ").unwrap());
    let notes = &input[2..];
    let rules = Rules::from(notes);
    println!("{:2}: {}", 0, pots.to_string());
    for _gen in 1..=20 {
        pots = pots.iterate(&rules);
        println!("{:2}: {}", _gen, pots.to_string());
    }

    pots.sum_of_pot_numbers()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn convert_combination_to_number() {
        assert_eq!(
            3,
            combination_to_number(&[
                &Pot::IsEmpty,
                &Pot::IsEmpty,
                &Pot::IsEmpty,
                &Pot::HasPlant,
                &Pot::HasPlant
            ])
        );
    }

    #[test]
    fn example() {
        assert_eq!(
            325,
            number_of_plants_after_20_gens(&read_str_to_lines(EXAMPLE))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            2063,
            number_of_plants_after_20_gens(&read_file_to_lines("input/day12.txt"))
        );
    }
}
