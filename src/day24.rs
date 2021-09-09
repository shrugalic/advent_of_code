use line_reader::read_file_to_lines;
use std::cmp::Ordering;

pub(crate) fn day24_part1() -> usize {
    strength_of_strongest_bridge(read_file_to_lines("input/day24.txt"))
}

pub(crate) fn day24_part2() -> usize {
    strength_of_longest_bridge(read_file_to_lines("input/day24.txt"))
}

type Pins = usize;
type Length = usize;
type Strength = Pins;
type Component = (Pins, Pins);
trait ComponentMethods {
    fn strength(&self) -> Strength;
    fn matches(&self, pins: &Pins) -> bool;
}
impl ComponentMethods for Component {
    fn strength(&self) -> Strength {
        self.0 + self.1
    }
    fn matches(&self, pins: &Pins) -> bool {
        &self.0 == pins || &self.1 == pins
    }
}
fn strength_of_strongest_bridge(input: Vec<String>) -> Strength {
    let mut components = parse_input(input);
    *strengths_and_lengths(0, 0, &0, &mut components[..])
        .iter()
        .map(|(strength, _length)| strength)
        .max()
        .unwrap()
}

fn strength_of_longest_bridge(input: Vec<String>) -> Strength {
    let mut components = parse_input(input);
    strengths_and_lengths(0, 0, &0, &mut components[..])
        .iter()
        .max_by(|a, b| match a.1.cmp(&b.1) {
            len @ (Ordering::Less | Ordering::Greater) => len,
            Ordering::Equal => a.0.cmp(&b.0),
        })
        .unwrap()
        .0
}

fn parse_input(input: Vec<String>) -> Vec<Component> {
    input
        .iter()
        .map(|s| {
            let (l, r) = s.split_once('/').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

fn strengths_and_lengths(
    str: Strength,
    len: Length,
    pins: &Pins,
    comps: &mut [Component],
) -> Vec<(Strength, Length)> {
    if comps.is_empty() {
        return vec![(str, len)];
    }
    let starts: Vec<Component> = comps.iter().filter(|c| c.matches(pins)).cloned().collect();
    if starts.is_empty() {
        return vec![(str, len)];
    }
    starts
        .iter()
        .flat_map(|start| {
            let pos = comps.iter().position(|c| c == start).unwrap();
            comps.swap(0, pos);
            if &start.0 == pins {
                strengths_and_lengths(str + start.strength(), len + 1, &start.1, &mut comps[1..])
            } else {
                strengths_and_lengths(str + start.strength(), len + 1, &start.0, &mut comps[1..])
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn example_part1() {
        assert_eq!(31, strength_of_strongest_bridge(read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part1() {
        assert_eq!(1940, day24_part1());
    }

    #[test]
    fn example_part2() {
        assert_eq!(19, strength_of_longest_bridge(read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(1928, day24_part2());
    }
}
