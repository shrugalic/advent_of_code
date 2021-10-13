const PUZZLE_INPUT: &str = "11100010111110100";

pub(crate) fn day16_part1() -> String {
    checksum_of_data_generated_to_len(PUZZLE_INPUT, 272)
}

pub(crate) fn day16_part2() -> String {
    checksum_of_data_generated_to_len(PUZZLE_INPUT, 35_651_584)
}

fn checksum_of_data_generated_to_len(input: &str, len: usize) -> String {
    let data = generate_data_of_length(input, len);
    calc_checksum(&data[0..len])
}

fn generate_data_of_length(input: &str, len: usize) -> String {
    let mut output = input.to_string();
    while output.len() < len {
        output = lengthen_data(&output);
    }
    output
}

fn lengthen_data<T: AsRef<str>>(input: T) -> String {
    let output: String = input
        .as_ref()
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();

    format!("{}0{}", input.as_ref(), output)
}

fn calc_checksum(s: &str) -> String {
    let mut checksum = reduce_data(s);
    while checksum.len() % 2 == 0 {
        checksum = reduce_data(&checksum);
    }
    checksum
}

fn reduce_data(s: &str) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .step_by(2)
        .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lengthen_data() {
        assert_eq!("100", lengthen_data("1"));
        assert_eq!("001", lengthen_data("0"));
        assert_eq!("11111000000", lengthen_data("11111"));
        assert_eq!("1111000010100101011110000", lengthen_data("111100001010"));
    }

    #[test]
    fn test_generate_data_of_length() {
        assert_eq!(
            "10000011110010000111110",
            generate_data_of_length("10000", 20)
        );
    }

    #[test]
    fn test_calc_checksum() {
        assert_eq!("100", calc_checksum("110010110100"));
    }

    #[test]
    fn test_checksum_of_data_generated_to_len() {
        assert_eq!("01100", checksum_of_data_generated_to_len("10000", 20));
    }

    #[test]
    fn part1() {
        assert_eq!("10100011010101011", day16_part1());
    }

    #[test]
    fn part2() {
        assert_eq!("01010001101011001", day16_part2());
    }
}
