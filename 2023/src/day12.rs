use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use Condition::*;
use RequiredCondition::*;

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    parse(input)
        .map(|cr| cr.count_possible_arrangements())
        .sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .map(ConditionRecord::unfolded)
        .map(|cr| cr.count_possible_arrangements())
        .sum()
}

type DamagedClusterLength = u8;
#[derive(Debug, PartialEq)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged_lengths: Vec<DamagedClusterLength>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum RequiredCondition {
    Any,
    NotDamaged,
    DamageOfLength(DamagedClusterLength),
}

fn count_possible_arrangements_rec<'a>(
    conditions: &'a [Condition],
    damaged_lengths: &'a [DamagedClusterLength],
    requirement: RequiredCondition,
    cache: &mut HashMap<
        (
            RequiredCondition,
            &'a [Condition],
            &'a [DamagedClusterLength],
        ),
        usize,
    >,
) -> usize {
    // println!(
    //     "'{}' {requirement:?} {}",
    //     conditions_to_string(conditions),
    //     damage_lengths_to_string(damaged_lengths)
    // );
    if cache.contains_key(&(requirement, conditions, damaged_lengths)) {
        return cache[&(requirement, conditions, damaged_lengths)];
    }
    let count = if conditions.is_empty() {
        if !damaged_lengths.is_empty() || matches!(requirement, DamageOfLength(_)) {
            // Fail, unconsumed damage clusters or unmatched requirement
            0
        } else {
            // OK
            1
        }
    } else {
        match (requirement, &conditions[0]) {
            (Any, Operational) | (NotDamaged, Operational) | (NotDamaged, Unknown) => {
                count_possible_arrangements_rec(&conditions[1..], damaged_lengths, Any, cache)
            }
            (Any, Damaged) => {
                if damaged_lengths.is_empty() {
                    0
                } else {
                    count_possible_arrangements_rec(
                        &conditions[1..],
                        &damaged_lengths[1..],
                        requirement_based_on(&damaged_lengths[0]),
                        cache,
                    )
                }
            }
            (Any, Unknown) => {
                // Fork into operational + damaged branch
                let operational_count =
                    count_possible_arrangements_rec(&conditions[1..], damaged_lengths, Any, cache);
                let damaged_count = if damaged_lengths.is_empty() {
                    0
                } else {
                    count_possible_arrangements_rec(
                        &conditions[1..],
                        &damaged_lengths[1..],
                        requirement_based_on(&damaged_lengths[0]),
                        cache,
                    )
                };
                operational_count + damaged_count
            }
            (NotDamaged, Damaged) => 0,
            (DamageOfLength(dmg_len), Operational) => {
                debug_assert_ne!(dmg_len, 0);
                0
            }
            (DamageOfLength(dmg_len), Damaged | Unknown) => count_possible_arrangements_rec(
                &conditions[1..],
                damaged_lengths,
                requirement_based_on(&dmg_len),
                cache,
            ),
        }
    };
    *cache
        .entry((requirement, conditions, damaged_lengths))
        .or_insert(count)
}

fn requirement_based_on(dmg_len: &DamagedClusterLength) -> RequiredCondition {
    // One damage length is consumed this turn
    if dmg_len == &1 {
        NotDamaged
    } else {
        DamageOfLength(*dmg_len - 1)
    }
}

impl ConditionRecord {
    fn count_possible_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        count_possible_arrangements_rec(
            &self.conditions[..],
            &self.damaged_lengths[..],
            Any,
            &mut cache,
        )
    }
    fn unfolded(mut self) -> Self {
        let mut conditions = self.conditions;
        conditions.push(Unknown);
        self.conditions = conditions.repeat(5);
        self.conditions.remove(self.conditions.len() - 1);
        let damage_lengths = self.damaged_lengths;
        self.damaged_lengths = damage_lengths.repeat(5);
        self
    }
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            conditions_to_string(&self.conditions),
            damage_lengths_to_string(&self.damaged_lengths)
        )
    }
}

fn conditions_to_string(conditions: &[Condition]) -> String {
    conditions.iter().map(Condition::as_char).collect()
}

fn damage_lengths_to_string(damaged_lengths: &[DamagedClusterLength]) -> String {
    damaged_lengths
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn parse(input: &str) -> impl Iterator<Item = ConditionRecord> + '_ {
    input.trim().lines().map(ConditionRecord::from)
}

impl Condition {
    fn as_char(&self) -> char {
        match self {
            Operational => '.',
            Damaged => '#',
            Unknown => '?',
        }
    }
}

impl From<char> for Condition {
    fn from(c: char) -> Self {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!("Invalid input"),
        }
    }
}

impl From<&str> for ConditionRecord {
    fn from(line: &str) -> Self {
        let (conditions, lengths) = line.split_once(' ').unwrap();

        // Replace extra operational
        let mut conditions = conditions.to_string();
        while conditions.contains("..") {
            conditions = conditions.replace("..", ".");
        }

        ConditionRecord {
            conditions: conditions.chars().map(Condition::from).collect(),
            damaged_lengths: lengths.split(',').filter_map(|n| n.parse().ok()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_example_parsing() {
        let cr = ConditionRecord::from("??.## 1,2");
        assert_eq!(
            cr,
            ConditionRecord {
                conditions: vec![Unknown, Unknown, Operational, Damaged, Damaged],
                damaged_lengths: vec![1, 2]
            }
        );
    }

    #[test]
    fn test_part1_example_line_0() {
        assert_eq!(1, get_example_1_line(0).count_possible_arrangements());
    }
    #[test]
    fn test_part1_example_line_1() {
        assert_eq!(4, get_example_1_line(1).count_possible_arrangements());
    }
    #[test]
    fn test_part1_example_line_2() {
        assert_eq!(1, get_example_1_line(2).count_possible_arrangements());
    }
    #[test]
    fn test_part1_example_line_3() {
        assert_eq!(1, get_example_1_line(3).count_possible_arrangements());
    }
    #[test]
    fn test_part1_example_line_4() {
        assert_eq!(4, get_example_1_line(4).count_possible_arrangements());
    }
    #[test]
    fn test_part1_example_line_5() {
        assert_eq!(10, get_example_1_line(5).count_possible_arrangements());
    }
    fn get_example_1_line(n: usize) -> ConditionRecord {
        parse(EXAMPLE).nth(n).unwrap()
    }

    #[test]
    fn just_unknowns() {
        assert_eq!(
            3,
            ConditionRecord::from("??? 1").count_possible_arrangements()
        );
        assert_eq!(
            10,
            ConditionRecord::from("??????? 2,1").count_possible_arrangements()
        );
    }

    #[test]
    fn test_known_contiguous_in_between() {
        let cr = ConditionRecord::from(".??.####.??. 1,4,1");
        assert_eq!(4, cr.count_possible_arrangements());
    }

    #[test]
    fn test_part1_examples() {
        assert_eq!(21, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(7_025, solve_part1(INPUT));
    }

    #[test]
    fn test_a_part1_input_line() {
        let cr = ConditionRecord::from("#???.#???#?.?.??.? 2,1,5,1,1");
        // ##.#.#####.....#.#
        // ##.#.#####....#..#
        // ##.#.#####..#....#
        // ##.#.#####..#..#..
        // ##.#.#####..#.#...
        assert_eq!(5, cr.count_possible_arrangements());
    }

    #[test]
    fn test_another_part1_input_line() {
        let cr = ConditionRecord::from("??#??#???????? 1,5,1");
        // #.#####.#.....
        // #.#####..#....
        // #.#####...#...
        // #.#####....#..
        // #.#####.....#.
        // #.#####......#
        // ..#.#####.#...
        // ..#.#####..#..
        // ..#.#####...#.
        // ..#.#####....#
        // ..#..#####.#..
        // ..#..#####..#.
        // ..#..#####...#
        assert_eq!(13, cr.count_possible_arrangements());
    }

    #[test]
    fn identify_single_damage_cluster_of_unique_length() {
        let cr = ConditionRecord::from("??.####.??? 1,4,2");
        assert_eq!(4, cr.count_possible_arrangements());
    }

    #[test]
    fn identify_single_mixed_unknown_and_damage_cluster_of_unique_length() {
        let cr = ConditionRecord::from("??.#?#?.??? 1,4,2");
        assert_eq!(4, cr.count_possible_arrangements());
    }

    #[test]
    fn test_0_wriggle_room_means_1_arrangement() {
        parse(
            "\
???#.???????# 2,1,8
#.??#??.???# 1,3,1,4
#?.?.##.#??#?# 2,1,2,6
?#??#.??????#### 5,1,1,1,4
?.?#.???????????? 1,2,9,2
???#??##???.?? 1,9,2
??##??.???.# 6,1,1,1
????#??.#.??????#? 5,1,1,8",
        )
        .for_each(|cr| assert_eq!(1, cr.count_possible_arrangements()));
    }

    #[test]
    fn test_unfold() {
        let cr = ConditionRecord::from(".# 1").unfolded();
        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", cr.to_string());
    }

    #[test]
    fn test_part2_example_line_0() {
        assert_eq!(1, get_example_2_line(0).count_possible_arrangements());
    }
    #[test]
    fn test_part2_example_line_1() {
        assert_eq!(16_384, get_example_2_line(1).count_possible_arrangements());
    }
    #[test]
    fn test_part2_example_line_2() {
        assert_eq!(1, get_example_2_line(2).count_possible_arrangements());
    }
    #[test]
    fn test_part2_example_line_3() {
        assert_eq!(16, get_example_2_line(3).count_possible_arrangements());
    }
    #[test]
    fn test_part2_example_line_4() {
        assert_eq!(2_500, get_example_2_line(4).count_possible_arrangements());
    }
    #[test]
    fn test_part2_example_line_5() {
        assert_eq!(506_250, get_example_2_line(5).count_possible_arrangements());
    }
    fn get_example_2_line(n: usize) -> ConditionRecord {
        parse(EXAMPLE)
            .map(ConditionRecord::unfolded)
            .nth(n)
            .unwrap()
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(525_152, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(11_461_095_383_315, solve_part2(INPUT));
    }
}
