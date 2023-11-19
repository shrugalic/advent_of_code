use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day08.txt");

pub(crate) fn day08_part1() -> usize {
    let signals = parse(INPUT);
    count_unique_digits(signals)
}

pub(crate) fn day08_part2() -> usize {
    let signals = parse(INPUT);
    sum_of_mapped_signals(signals)
}

type CharSet = HashSet<char>;
type CharSets = Vec<CharSet>;
type Signal = (CharSets, CharSets);

fn count_unique_digits(signals: Vec<Signal>) -> usize {
    signals
        .iter()
        .map(|(_inputs, outputs)| {
            outputs
                .iter()
                .filter(|set| [2, 4, 3, 7].contains(&set.len()))
                .count()
        })
        .sum()
}

fn sum_of_mapped_signals(signals: Vec<Signal>) -> usize {
    signals.into_iter().map(signal_to_value).sum()
}

fn signal_to_value(signal: Signal) -> usize {
    let (inputs, outputs) = signal;
    let sets = determine_mapping(inputs);

    let mut total = 0;
    for output in outputs {
        let decoded = sets.iter().position(|set| output.eq(set)).unwrap();
        total = 10 * total + decoded;
    }
    total
}

fn determine_mapping(inputs: CharSets) -> Vec<HashSet<char>> {
    // The set at index contains the characters used to encode the digit with value == index
    let mut sets = vec![HashSet::new(); 10];

    // Inputs with unique length signals
    sets[1] = inputs_of_len(2, &inputs).remove(0);
    sets[4] = inputs_of_len(4, &inputs).remove(0);
    sets[7] = inputs_of_len(3, &inputs).remove(0);
    sets[8] = inputs_of_len(7, &inputs).remove(0);
    let len_5_inputs = inputs_of_len(5, &inputs); // 5-segment lengths 2, 3 or 5
    let len_6_inputs = inputs_of_len(6, &inputs); // 6-segment lengths 0, 6 and 9

    // 3 is the 5-segment input that contains the same segments as 1
    sets[3] = len_5_inputs.containing_segments_of(&sets[1]);

    // adg = intersection of the 5-segment digits 2, 3 and 5:
    let adg = intersecting_segments(&len_5_inputs);

    // 8-0 = d, 8-6 = c, 8-9 = e. Let's combine these to cde
    let cde = union_of_differences(&sets[8], &len_6_inputs);

    // d = intersection of adg and cde
    let d = *adg.intersection(&cde).next().unwrap();

    // zero = the 6-segment-input without d (6 and 9 both have a segment d)
    sets[0] = len_6_inputs.not_containing_segment(&d);

    // b = 4 - 3
    let b = sets[4].difference(&sets[3]).next().unwrap();

    // 5 is the 5-segment-input with a segment b (2 and 3 don't have a segment b)
    sets[5] = len_5_inputs.containing_a_segment(b);

    // 2 = len_5s_235 - 3 - 5
    sets[2] = len_5_inputs.subtract(&sets[3], &sets[5]);

    // e = 2 - 3
    let e = sets[2].difference(&sets[3]).next().unwrap();

    // 6 = len_6s_069 with e (unlike 9) and d (unlike 0)
    sets[6] = len_6_inputs.containing_both(e, &d);

    // 9 = len_gs_069 - 0 - 6
    sets[9] = len_6_inputs.subtract(&sets[0], &sets[6]);
    sets
}

trait CharSetOps {
    fn containing_a_segment(&self, wanted: &char) -> CharSet {
        self.matching(&|set: &CharSet| set.contains(wanted))
    }
    fn containing_segments_of(&self, other: &CharSet) -> CharSet {
        self.matching(&|this: &CharSet| other.iter().all(|segment| this.contains(segment)))
    }
    fn containing_both(&self, a: &char, b: &char) -> CharSet {
        self.matching(&|set: &CharSet| set.contains(a) && set.contains(b))
    }
    fn not_containing_segment(&self, unwanted: &char) -> CharSet {
        self.matching(&|set: &CharSet| !set.contains(unwanted))
    }
    fn subtract(&self, a: &CharSet, b: &CharSet) -> CharSet {
        self.matching(&|set: &CharSet| !set.eq(a) && !set.eq(b))
    }
    fn matching(&self, filter: &dyn Fn(&CharSet) -> bool) -> CharSet;
}
impl CharSetOps for CharSets {
    fn matching(&self, filter: &dyn Fn(&CharSet) -> bool) -> CharSet {
        self.iter().find(|set| filter(set)).cloned().unwrap()
    }
}

fn intersecting_segments(sets: &[CharSet]) -> CharSet {
    sets.iter()
        .cloned()
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
}

fn union_of_differences(super_set: &CharSet, sub_sets: &[CharSet]) -> CharSet {
    sub_sets
        .iter()
        .flat_map(|sub_set| super_set.difference(sub_set).cloned().collect::<CharSet>())
        .collect()
}

fn inputs_of_len(len: usize, inputs: &[CharSet]) -> CharSets {
    inputs
        .iter()
        .filter(|set| set.len() == len)
        .cloned()
        .collect()
}

fn parse(input: &str) -> Vec<Signal> {
    input
        .trim()
        .lines()
        .map(|s| {
            let (inputs, outputs) = s.split_once(" | ").unwrap();
            let inputs = inputs
                .split_ascii_whitespace()
                .map(|s| s.chars().collect())
                .collect();
            let outputs = outputs
                .split_ascii_whitespace()
                .map(|s| s.chars().collect())
                .collect();
            (inputs, outputs)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const EXAMPLE_2: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn part1_example() {
        let input = parse(EXAMPLE_2);
        assert_eq!(26, count_unique_digits(input));
    }

    #[test]
    fn part2_example_map_signal() {
        let mut input = parse(EXAMPLE_1);
        assert_eq!(5353, signal_to_value(input.remove(0)));
    }

    #[test]
    fn part2_larger_example() {
        let input = parse(EXAMPLE_2);
        assert_eq!(61229, sum_of_mapped_signals(input));
    }

    #[test]
    fn part1() {
        assert_eq!(272, day08_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1_007_675, day08_part2());
    }
}
