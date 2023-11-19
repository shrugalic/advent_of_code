use std::ops::Deref;

const INPUT: &str = include_str!("../input/day25.txt");

pub(crate) fn day25_part1() -> String {
    let snafu_numbers = parse(INPUT);
    let sum: usize = snafu_numbers.iter().map(SnafuNumber::to_decimal).sum();
    SnafuNumber::from(sum).to_string()
}

#[derive(Debug)]
/// A base 5 number with weird digits:
/// '0' worth 0, '1' worth 1, '2' worth 2, '=' worth -2 and '-' worth -1
struct SnafuNumber(Vec<char>);

impl From<usize> for SnafuNumber {
    fn from(mut decimal: usize) -> Self {
        let mut digits: Vec<char> = vec![];
        loop {
            let (digit, carry) = match decimal % 5 {
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 1),
                4 => ('-', 1),
                _ => unreachable!(),
            };
            digits.push(digit);
            decimal = decimal / 5 + carry;
            if decimal == 0 {
                break;
            }
        }
        SnafuNumber(digits.into_iter().rev().collect())
    }
}
impl SnafuNumber {
    fn to_decimal(&self) -> usize {
        let mut decimal: isize = 0;
        let mut multiplier = 1;
        for digit in self.iter().rev() {
            let value = match digit {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '=' => -2,
                '-' => -1,
                _ => unreachable!(),
            };
            decimal += multiplier * value;
            multiplier *= 5;
        }

        decimal as usize
    }
}
impl From<&str> for SnafuNumber {
    fn from(snafu: &str) -> Self {
        SnafuNumber(snafu.chars().collect())
    }
}
impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        self.iter().collect()
    }
}
impl Deref for SnafuNumber {
    type Target = [char];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

fn parse(input: &str) -> Vec<SnafuNumber> {
    input.trim().lines().map(SnafuNumber::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    const EXAMPLES: [(&str, usize); 16] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("1=", 3),
        ("1-", 4),
        ("10", 5),
        ("11", 6),
        ("12", 7),
        ("2=", 8),
        ("2-", 9),
        ("20", 10),
        ("1=0", 15),
        ("1-0", 20),
        ("1=11-2", 2_022),
        ("1-0---0", 12_345),
        ("1121-1110-1=0", 314_159_265),
    ];

    #[test]
    fn test_snafu_to_decimal() {
        for (snafu, decimal) in EXAMPLES {
            assert_eq!(decimal, SnafuNumber::from(snafu).to_decimal())
        }
    }

    #[test]
    fn test_snafu_from_decimal() {
        for (snafu_str, decimal) in EXAMPLES {
            let snafu = SnafuNumber::from(decimal);
            assert_eq!(snafu_str, snafu.to_string())
        }
    }

    #[test]
    fn part1_example() {
        let numbers = parse(EXAMPLE);
        let sum: usize = numbers.iter().map(SnafuNumber::to_decimal).sum();
        assert_eq!(4_890, sum);
        assert_eq!("2=-1=0", SnafuNumber::from(sum).to_string());
    }

    #[test]
    fn part1() {
        assert_eq!("2-212-2---=00-1--102", day25_part1());
    }
}
