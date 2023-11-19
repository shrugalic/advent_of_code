use crate::parse;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day06.txt");

pub(crate) fn day06_part1() -> String {
    error_corrected_message(parse(INPUT), Part::One)
}

pub(crate) fn day06_part2() -> String {
    error_corrected_message(parse(INPUT), Part::Two)
}

enum Part {
    One,
    Two,
}
fn error_corrected_message(messages: Vec<&str>, part: Part) -> String {
    let mut corrected = vec![];
    for idx in 0..messages[0].len() {
        let mut frequency = HashMap::new();
        for message in &messages {
            *frequency
                .entry(message.chars().nth(idx).unwrap())
                .or_insert(0) += 1;
        }
        let ch = match part {
            Part::One => frequency.iter().max_by_key(|(_ch, &freq)| freq),
            Part::Two => frequency.iter().min_by_key(|(_ch, &freq)| freq),
        };
        corrected.push(*ch.map(|(ch, _freq)| ch).unwrap());
    }

    corrected.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn part1_example() {
        assert_eq!(
            "easter".to_string(),
            error_corrected_message(parse(EXAMPLE), Part::One)
        );
    }

    #[test]
    fn part1() {
        assert_eq!("qtbjqiuq", day06_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            "advent".to_string(),
            error_corrected_message(parse(EXAMPLE), Part::Two)
        );
    }

    #[test]
    fn part2() {
        assert_eq!("akothqli", day06_part2());
    }
}
