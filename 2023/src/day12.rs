use std::fmt::{Display, Formatter};

use Condition::*;

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    parse(input)
        .map(ConditionRecord::count_possible_arrangements)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .map(ConditionRecord::unfolded)
        .map(ConditionRecord::count_possible_arrangements)
        .sum()
}

impl ConditionRecord {
    fn unfolded(mut self) -> Self {
        let mut conditions = self.conditions;
        conditions.push(Unknown);
        self.conditions = conditions.repeat(5);
        self.conditions.remove(self.conditions.len() - 1);
        let damage_lengths = self.damage_cluster_lengths;
        self.damage_cluster_lengths = damage_lengths.repeat(5);
        self
    }
    fn count_possible_arrangements(mut self) -> usize {
        self.remove_any_operational_infix_suffix_or_tuples();

        let unknown_positions: Vec<_> = self
            .conditions
            .iter()
            .enumerate()
            .filter_map(|(i, c)| (c == &Unknown).then_some(i))
            .collect();
        let unknown_count = unknown_positions.len();
        let damaged_count = self.conditions.iter().filter(|c| c == &&Damaged).count();
        let target_damaged_count = self.damage_cluster_lengths.iter().sum::<u8>() as usize;
        let missing_damaged_count = target_damaged_count - damaged_count;

        let total_conditions_count = self.conditions.len();
        let damage_cluster_count = self.damage_cluster_lengths.len();
        let minimum_operational_count = damage_cluster_count - 1;

        // "Wriggle room" is the number of extra unknowns compared to the minimum needed to fulfill
        // the conditions imposed by the damage cluster length
        // ?#?#?#?#?#?#?#? 1,3,1,6  has 1 wriggle room: 1 more ? than minimally needed
        // ??.?? 1,1                has 2 wriggle room: 2 more ? than minimally needed
        // ???? 1                   has 3 wriggle room: 3 more ? than minimally needed
        // ?###???????? 3,2,1       has 4 wriggle room: 4 more ? than minimally needed
        let wriggle_room =
            total_conditions_count - target_damaged_count - minimum_operational_count;

        if unknown_count == 0 || missing_damaged_count == 0 || wriggle_room == 0 {
            return 1;
        }

        let missing_operational_count = unknown_count - missing_damaged_count;
        if false {
            print!("{self}: {unknown_count} unknowns = {missing_operational_count} operational");
            print!(" + {missing_damaged_count}/{target_damaged_count} damaged");
            println!(", {wriggle_room} wriggle room");
        }
        if self.replace_largest_unambiguous_damage_cluster() {
            // less ambiguity due to replacements
            return self.count_possible_arrangements();
        }
        if self.replace_largest_unambiguous_mixed_cluster() {
            // less ambiguity due to replacements
            return self.count_possible_arrangements();
        }

        let possibility_count = 2usize.pow(unknown_count as u32);
        let mut count_of_valid_arrangements = 0;

        // let mut replacements_template: Vec<_> = iter::repeat(Operational)
        //     .take(missing_operational_count)
        //     .chain(iter::repeat(Damaged).take(missing_damaged_count))
        //     .collect();
        // println!("template {}", to_string(&replacements_template));
        //
        // assert_eq!(replacements_template.len(), unknown_count);
        // for replacements in replacements_template.permutation() {
        //     println!("replacement {}", to_string(&replacements));

        for possibility in 0..possibility_count {
            let number = format!("{possibility:0>unknown_count$b}");
            if possibility.count_ones() != missing_damaged_count as u32 {
                continue;
            }
            let replacements: Vec<_> = number
                .chars()
                .map(|c| if c == '1' { Damaged } else { Operational })
                .collect();
            // assert_eq!(
            //     replacements.iter().filter(|c| c == &&Damaged).count() as u32,
            //     possibility.count_ones()
            // );
            // let repl_str = to_string(&replacements);
            self.conditions_replaced_with(replacements, &unknown_positions);

            let is_valid = self.is_valid();
            // println!(
            //     "{number} -> {} -> {} is {}valid",
            //     repl_str,
            //     to_string(&self.conditions),
            //     if is_valid { "" } else { "not " }
            // );
            if is_valid {
                // println!("{}", to_string(&resolved),);
                count_of_valid_arrangements += 1;
            }
        }
        count_of_valid_arrangements
    }
    fn remove_any_operational_infix_suffix_or_tuples(&mut self) {
        // Remove any operational prefix, suffix, or tuples
        self.conditions = to_string(&self.conditions)
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(".")
            .chars()
            .map(Condition::from)
            .collect();
    }
    fn replace_largest_unambiguous_damage_cluster(&mut self) -> bool {
        let max_dmg_len;
        if let Some(dmg_len) = self.unique_longest_damage_cluster() {
            max_dmg_len = dmg_len;
        } else {
            return false;
        }
        let cluster_starts: Vec<_> = self
            .conditions
            .windows(max_dmg_len as usize)
            .enumerate()
            .filter_map(|(i, c)| c.iter().all(|c| c == &Damaged).then_some(i))
            .collect();
        if cluster_starts.len() != 1 {
            // More than one cluster with this length
            return false;
        }

        let start = cluster_starts[0];
        let end = start + max_dmg_len as usize;
        self.replace_range_with_damaged(start, end)
    }
    fn replace_largest_unambiguous_mixed_cluster(&mut self) -> bool {
        let max_dmg_len;
        if let Some(dmg_len) = self.unique_longest_damage_cluster() {
            max_dmg_len = dmg_len;
        } else {
            return false;
        }

        // For example: ??.??.?## 1,1,3
        // Trickier one: ?#?#?.???###.# 1,1,5,1
        let cluster_starts: Vec<_> = self
            .conditions
            .windows(max_dmg_len as usize)
            .enumerate()
            // .inspect(|(i, c)| {
            //     let check_before = 1 <= *i;
            //     let check_after = i + max_dmg_len as usize <= self.conditions.len() - 1;
            //     println!(
            //         "{i}: {} check before {} and after {}",
            //         to_string(c),
            //         check_before,
            //         check_after
            //     );
            // })
            .filter_map(|(i, c)| c.iter().all(|c| c != &Operational).then_some(i))
            .filter(|&i| i == 0 || self.conditions[i - 1] != Damaged)
            .filter(|&i| {
                let after = i + max_dmg_len as usize;
                let last_valid = self.conditions.len() - 1;
                after > last_valid || self.conditions[after] != Damaged
            })
            .collect();
        if cluster_starts.len() != 1 {
            // More than one cluster with this length
            return false;
        }
        let start = cluster_starts[0];
        let end = start + max_dmg_len as usize;
        self.replace_range_with_damaged(start, end)
    }
    fn unique_longest_damage_cluster(&self) -> Option<DamageLength> {
        let max_dmg_len = *self.damage_cluster_lengths.iter().max().unwrap();
        let matching_cluster_count = self
            .damage_cluster_lengths
            .iter()
            .filter(|&&d| d == max_dmg_len)
            .count();

        if matching_cluster_count > 1 {
            // Non-unique length -> TODO distribution should be feasible
            // println!("----- learn how to distribute multiple clusters, such as {self} -----");
        }
        (matching_cluster_count == 1).then_some(max_dmg_len)
    }
    fn replace_range_with_damaged(&mut self, start: usize, end: usize) -> bool {
        let mut replaced = false;
        (start..end).for_each(|i| {
            if self.conditions[i] != Damaged {
                replaced = true;
                self.conditions[i] = Damaged;
            }
        });
        if start > 0 {
            assert_ne!(self.conditions[start - 1], Damaged);
            if self.conditions[start - 1] == Unknown {
                replaced = true;
                self.conditions[start - 1] = Operational;
            }
        }
        if end < self.conditions.len() {
            assert_ne!(self.conditions[end], Damaged);
            if self.conditions[end] == Unknown {
                replaced = true;
                self.conditions[end] = Operational;
            }
        }
        if replaced {
            // println!(
            //     "Found contiguous damaged cluster @ {}..{}: {}",
            //     start,
            //     end,
            //     to_string(&self.conditions[start..end])
            // );
        }
        replaced
    }
    fn is_valid(&self) -> bool {
        ConditionRecord::is_valid_condition(&self.conditions, &self.damage_cluster_lengths)
    }
    fn is_valid_condition(conditions: &[Condition], damage_lengths: &[u8]) -> bool {
        assert!(conditions.iter().all(|c| c != &Unknown));
        let conditions_damage_lengths: Vec<u8> = conditions
            .split(|condition| condition == &Operational)
            .map(|part| part.len() as u8)
            .filter(|n| n > &0)
            .collect();
        conditions_damage_lengths == damage_lengths
    }
    fn conditions_replaced_with(
        &mut self,
        mut replacements: Vec<Condition>,
        unknown_positions: &[usize],
    ) {
        unknown_positions.iter().for_each(|i| {
            self.conditions[*i] = replacements.pop().unwrap();
        });
    }
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            to_string(&self.conditions),
            self.damage_cluster_lengths
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

fn to_string(conditions: &[Condition]) -> String {
    conditions.iter().map(Condition::as_char).collect()
}

fn parse(input: &str) -> impl Iterator<Item = ConditionRecord> + '_ {
    input.trim().lines().map(ConditionRecord::from)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
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

type DamageLength = u8;
#[derive(Debug, PartialEq)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    damage_cluster_lengths: Vec<DamageLength>,
}

impl From<&str> for ConditionRecord {
    fn from(line: &str) -> Self {
        let (conditions, lengths) = line.split_once(' ').unwrap();

        // Replace extra operationals
        let mut conditions = conditions.to_string();
        while conditions.contains("..") {
            conditions = conditions.replace("..", ".");
        }

        ConditionRecord {
            conditions: conditions.chars().map(Condition::from).collect(),
            damage_cluster_lengths: lengths.split(',').filter_map(|n| n.parse().ok()).collect(),
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
                damage_cluster_lengths: vec![1, 2]
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
    fn test_known_contiguous_in_between() {
        let cr = ConditionRecord::from(".??.####.??. 1,4,1");
        assert_eq!(4, cr.count_possible_arrangements());
    }

    #[test]
    fn test_is_valid() {
        assert!(ConditionRecord::from("#.#.### 1,1,3").is_valid());
        assert!(!ConditionRecord::from("....### 1,1,3").is_valid());
        assert!(!ConditionRecord::from("..#.### 1,1,3").is_valid());
        assert!(!ConditionRecord::from(".#..### 1,1,3").is_valid());
        assert!(!ConditionRecord::from(".##.### 1,1,3").is_valid());
        assert!(!ConditionRecord::from("#...### 1,1,3").is_valid());
        assert!(!ConditionRecord::from("##..### 1,1,3").is_valid());
        assert!(!ConditionRecord::from("###.### 1,1,3").is_valid());
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
    fn test_replaced_unambiguous_clusters_for_only_damaged() {
        let mut cr = ConditionRecord::from("?###???????? 3,2,1");
        let did_replace = cr.replace_largest_unambiguous_damage_cluster();
        assert!(did_replace);
        assert_eq!(".###.??????? 3,2,1", cr.to_string());
    }

    #[test]
    fn test_replaced_unambiguous_clusters_for_well_separated_mixed() {
        let mut cr = ConditionRecord::from("??.??.?## 1,1,3");
        let did_replace = cr.replace_largest_unambiguous_mixed_cluster();
        assert!(did_replace);
        assert_eq!("??.??.### 1,1,3", cr.to_string());
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
    fn test_remove_any_operational_infix_suffix_or_tuples_after_unfolding() {
        let mut cr = ConditionRecord::from(".?..#. 1").unfolded();
        cr.remove_any_operational_infix_suffix_or_tuples();
        assert_eq!("?.#.?.?.#.?.?.#.?.?.#.?.?.# 1,1,1,1,1", cr.to_string());
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
        assert_eq!(1, solve_part2(INPUT));
    }
}
