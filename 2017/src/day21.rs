use crate::parse;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day21.txt");

const STARTING_PATTERN: &str = ".#./..#/###";
const ON: char = '#';
const OFF: char = '.';

pub(crate) fn day21_part1() -> usize {
    pixels_after_n_iterations(parse(INPUT), 5)
}

pub(crate) fn day21_part2() -> usize {
    pixels_after_n_iterations(parse(INPUT), 18)
}

fn pixels_after_n_iterations(input: Vec<&str>, n: usize) -> usize {
    let rules: Vec<Rule> = input.iter().map(Rule::from).collect();
    let pattern = Pattern::from(STARTING_PATTERN);
    let mut cache: HashMap<(Pattern, usize), usize> = HashMap::new();
    iterate(n, &rules, pattern, &mut cache)
}

fn iterate(
    n: usize,
    rules: &[Rule],
    mut pattern: Pattern,
    mut cache: &mut HashMap<(Pattern, usize), usize>,
) -> usize {
    if n >= 3 {
        // After 3 iterations there are nine 3x3 patterns, which can be handled individually
        if let Some(count) = cache.get(&(pattern.clone(), n)) {
            *count
        } else {
            let entry = cache.entry((pattern.clone(), 3)).or_insert(0);
            pattern = Pattern::from_joined(pattern.process_rules(rules));
            pattern = Pattern::from_joined(pattern.process_rules(rules));
            let patterns = pattern.process_rules(rules);
            *entry = patterns.iter().map(|p| p.active_pixel_count()).sum();
            patterns
                .into_iter()
                .map(|pattern| iterate(n - 3, rules, pattern, &mut cache))
                .sum()
        }
    } else {
        for _ in 0..n {
            pattern = Pattern::from_joined(pattern.process_rules(rules));
        }
        pattern.active_pixel_count()
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Pattern {
    size: usize,
    pattern: Vec<char>,
}
impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pattern
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let mut s = p.to_string();
                    if i > 0 && i % self.size == 0 {
                        s.insert(0, '\n')
                    }
                    s
                })
                .collect::<String>()
        )
    }
}
impl<T: AsRef<str>> From<T> for Pattern {
    fn from(s: T) -> Self {
        let pattern: Vec<_> = s
            .as_ref()
            .split('/')
            .flat_map(|line| line.chars())
            .collect();
        let size = (pattern.len() as f64).sqrt() as usize;
        Pattern { size, pattern }
    }
}
impl Pattern {
    fn active_pixel_count(&self) -> usize {
        self.pattern.iter().filter(|&p| p == &ON).count()
    }
    fn corner_indices(&self) -> (usize, usize, usize, usize) {
        (
            0,
            self.size - 1,
            (self.size - 1) * self.size,
            self.size * self.size - 1,
        )
    }
    fn flip_horizontally(&mut self) -> &Pattern {
        let (top_left, _, bottom_left, _) = self.corner_indices();
        for i in 0..self.size {
            self.pattern.swap(top_left + i, bottom_left + i);
        }
        self
    }
    fn rotate_90_degrees_clockwise(&mut self) -> &Pattern {
        let (top_left, top_right, bottom_left, bottom_right) = self.corner_indices();
        // corners
        self.pattern.swap(top_left, top_right);
        self.pattern.swap(bottom_left, bottom_right);
        self.pattern.swap(top_left, bottom_right);
        if self.size == 3 {
            // mid points
            self.pattern.swap(1, 5);
            self.pattern.swap(3, 7);
            self.pattern.swap(1, 7);
        }
        self
    }
    fn process_rules(self, rules: &[Rule]) -> Vec<Pattern> {
        let small_size = if self.size % 2 == 0 { 2 } else { 3 };
        let mut patterns = self.split_into_patterns_of(small_size);
        let rules: Vec<_> = rules.iter().filter(|r| r.size == small_size).collect();
        // println!("rules {:?}", rules);
        patterns
            .iter_mut()
            .map(|pattern| {
                let rule = rules.iter().find(|rule| rule.matches(pattern)).unwrap();
                rule.to.clone()
            })
            .collect()
    }
    fn split_into_patterns_of(self, small_size: usize) -> Vec<Pattern> {
        let patterns_per_side = self.size / small_size;
        let mut patterns = vec![
            Pattern {
                size: small_size,
                pattern: vec![OFF; small_size * small_size],
            };
            patterns_per_side * patterns_per_side
        ];
        for (i, pattern_idx, sub_col) in Pattern::index_mapping(self.size, small_size) {
            patterns[pattern_idx].pattern[sub_col] = self.pattern[i];
        }
        patterns
    }
    fn from_joined(patterns: Vec<Pattern>) -> Self {
        let count = (patterns.len() as f64).sqrt() as usize;
        let size = patterns[0].size * count;
        let mut joined = Pattern {
            size,
            pattern: vec![OFF; size * size],
        };
        for (i, pattern_idx, sub_col) in Pattern::index_mapping(size, patterns[0].size) {
            joined.pattern[i] = patterns[pattern_idx].pattern[sub_col];
        }
        joined
    }
    /// Returns a mapping of indices between between a single pattern and multiple patterns.
    /// `large` is the size of the large pattern, such as 6 for example,
    /// and `small` is the size of a small pattern, such as 2 or 3 for example.
    /// A 6x6 could be divided into four 3x3 or nine 2x2 patterns.
    /// `pattern_idx` is the index of such a small pattern in the list of small patterns,
    /// like 0..=3 in the four 3x3 example, or 0..=8 in the nine 2x2 example.
    /// `sub_col` is each small pattern's internal index, like
    /// 0..=8 in the four 3x3 example, or 0..=3 in the nine 2x2 example.
    fn index_mapping(large: usize, small: usize) -> Vec<(usize, usize, usize)> {
        let count = large / small;
        (0..(large * large))
            .into_iter()
            .map(|i| {
                let col = i % large;
                let row = i / large;
                let pattern_idx = i / (count * small * small) * count + col / small;
                let sub_col = (row % small) * small + col % small;
                (i, pattern_idx, sub_col)
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    size: usize,
    from: HashSet<Pattern>,
    to: Pattern,
}
impl Rule {
    fn matches(&self, pattern: &Pattern) -> bool {
        self.from.contains(pattern)
    }
}
impl<T: AsRef<str>> From<T> for Rule {
    fn from(s: T) -> Self {
        let (from, to) = s.as_ref().split_once(" => ").unwrap();
        let mut pattern = Pattern::from(from);
        let mut flipped = pattern.clone();
        flipped.flip_horizontally();
        let mut from = HashSet::new();
        from.insert(pattern.clone());
        from.insert(flipped.clone());
        for _ in 0..3 {
            pattern.rotate_90_degrees_clockwise();
            flipped.rotate_90_degrees_clockwise();
            from.insert(pattern.clone());
            from.insert(flipped.clone());
        }
        Rule {
            size: pattern.size,
            from,
            to: Pattern::from(to),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE_RULES: &str = "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn parse_pattern() {
        let pattern = vec![ON, OFF, ON, OFF, OFF, ON, ON, ON, OFF];
        assert_eq!(Pattern { size: 3, pattern }, Pattern::from("#.#/..#/##."));
    }

    #[test]
    fn pattern2_rotate_90_degrees_clockwise() {
        let mut rotated = Pattern {
            size: 2,
            pattern: vec![ON, OFF, OFF, OFF],
        };
        rotated.rotate_90_degrees_clockwise();
        let expected = Pattern {
            size: 2,
            pattern: vec![OFF, ON, OFF, OFF],
        };
        assert_eq!(expected, rotated);
    }

    #[test]
    fn pattern3_rotate_90_degrees_clockwise() {
        let mut rotated = Pattern {
            size: 3,
            pattern: vec![ON, OFF, ON, OFF, OFF, ON, ON, ON, OFF],
        };
        rotated.rotate_90_degrees_clockwise();
        let expected = Pattern {
            size: 3,
            pattern: vec![ON, OFF, ON, ON, OFF, OFF, OFF, ON, ON],
        };
        assert_eq!(expected, rotated);
    }

    #[test]
    fn pattern_flip_horizontally() {
        let mut rotated = Pattern {
            size: 3,
            pattern: vec![OFF, ON, OFF, OFF, OFF, ON, ON, ON, ON],
        };
        rotated.flip_horizontally();
        let expected = Pattern {
            size: 3,
            pattern: vec![ON, ON, ON, OFF, OFF, ON, OFF, ON, OFF],
        };
        assert_eq!(expected, rotated);
    }

    #[test]
    fn split_into_patterns_of_size_1() {
        let pattern = Pattern::from(STARTING_PATTERN);
        assert_eq!(
            vec![Pattern::from(STARTING_PATTERN)],
            pattern.split_into_patterns_of(3)
        );
    }

    #[test]
    fn split_into_patterns_of_size_2() {
        let four_by_four = four_by_four();
        assert_eq!(
            vec![
                Pattern {
                    size: 2,
                    pattern: vec![ON, OFF, OFF, OFF],
                },
                Pattern {
                    size: 2,
                    pattern: vec![OFF, ON, OFF, OFF],
                },
                Pattern {
                    size: 2,
                    pattern: vec![OFF, OFF, ON, OFF],
                },
                Pattern {
                    size: 2,
                    pattern: vec![OFF, OFF, OFF, ON],
                }
            ],
            four_by_four.split_into_patterns_of(2)
        );
    }

    #[test]
    fn split_into_patterns_of_size_3() {
        let six_by_six = six_by_six();
        assert_eq!(
            vec![
                three_by_three(),
                three_by_three(),
                three_by_three(),
                three_by_three()
            ],
            six_by_six.split_into_patterns_of(3)
        );
    }

    #[test]
    fn join_four_2x2_patterns_into_a_4x4_pattern() {
        assert_eq!(
            four_by_four(),
            Pattern::from_joined(vec![
                Pattern {
                    size: 2,
                    pattern: vec![ON, OFF, OFF, OFF],
                },
                Pattern {
                    size: 2,
                    pattern: vec![OFF, ON, OFF, OFF],
                },
                Pattern {
                    size: 2,
                    pattern: vec![OFF, OFF, ON, OFF],
                },
                Pattern {
                    size: 2,
                    pattern: vec![OFF, OFF, OFF, ON],
                }
            ])
        );
    }

    #[test]
    fn join_four_3x3_patterns_into_a_6x6_pattern() {
        let six_by_six = six_by_six();
        assert_eq!(
            six_by_six,
            Pattern::from_joined(vec![
                three_by_three(),
                three_by_three(),
                three_by_three(),
                three_by_three()
            ])
        );
    }

    #[test]
    fn matches_rule() {
        let rule: Rule = Rule::from(&parse(EXAMPLE_RULES)[1]);
        let pattern = Pattern::from(STARTING_PATTERN);
        assert!(rule.matches(&pattern));
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            12,
            pixels_after_n_iterations(parse(EXAMPLE_RULES), 2)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(190, day21_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(2335049, day21_part2());
    }

    fn three_by_three() -> Pattern {
        Pattern {
            size: 3,
            pattern: vec![ON, ON, OFF, ON, OFF, OFF, OFF, OFF, OFF],
        }
    }

    fn four_by_four() -> Pattern {
        Pattern {
            size: 4,
            pattern: vec![
                ON, OFF, OFF, ON, //
                OFF, OFF, OFF, OFF, //
                OFF, OFF, OFF, OFF, //
                ON, OFF, OFF, ON,
            ],
        }
    }

    fn six_by_six() -> Pattern {
        Pattern {
            size: 6,
            pattern: vec![
                ON, ON, OFF, ON, ON, OFF, //
                ON, OFF, OFF, ON, OFF, OFF, //
                OFF, OFF, OFF, OFF, OFF, OFF, //
                ON, ON, OFF, ON, ON, OFF, //
                ON, OFF, OFF, ON, OFF, OFF, //
                OFF, OFF, OFF, OFF, OFF, OFF,
            ],
        }
    }
}
