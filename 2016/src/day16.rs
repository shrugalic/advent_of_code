const PUZZLE_INPUT: &str = "11100010111110100";

pub(crate) fn day16_part1() -> String {
    checksum_of_data_generated_to_len(PUZZLE_INPUT, 272)
}

pub(crate) fn day16_part2() -> String {
    checksum_of_data_generated_to_len(PUZZLE_INPUT, 35_651_584)
}

enum Method {
    #[allow(unused)]
    Strings,
    #[allow(unused)]
    BoolVec,
    Fast,
}
fn checksum_of_data_generated_to_len(input: &str, min_len: usize) -> String {
    let method = Method::Fast;
    match method {
        Method::Strings => {
            let data = generate_string_of_length(input, min_len);
            calc_checksum_of_string(&data[0..min_len])
        }
        Method::BoolVec => {
            let data = generate_boolvec_of_length(input, min_len);
            calc_checksum_of_boolvec(&data[0..min_len])
        }
        Method::Fast => faster_checksum_of_data_generated_to_len(input, min_len),
    }
}
fn faster_checksum_of_data_generated_to_len(input: &str, min_len: usize) -> String {
    // Let's call the input sequence a. One step of lengthening appends 0 to this sequence,
    // as well as its reversed negation. Let's call this reversed_negation(a) == b, so the
    // complete sequence after one step looks like a_0_b (underscores are for formatting only).
    // The next lengthening step does the same thing: it appends 0 and reversed_negation(a0b).
    // Interestingly, the reversed_negation(a0b) is equal to a1b! So the complete sequence
    // after 2 steps is a0b_0_a1b. So basically it's a sequence of alternating a and b separated
    // by 0s and 1s. If we focus on the 0s and 1s only this would be 001.
    // After 3 steps it's a0b_0_a1b_0_a0b_1_a1b, or 001_0_011.
    // After 4 steps a0b0a1b_0_a0b1a1b_0_a0b0a1b_1_a0b1a1b; or 001_0_011_0_001_1_011
    // After 5 steps a0b0a1b0a0b1a1b_0_a0b0a1b1a0b1a1b_0_a0b0a1b0a0b1a1b_1_a0b0a1b1a0b1a1b;
    //               or 0010011_0_0011011_0_0010011_1_0011011
    // Notice how the right half (after the center 0) consists of the same two parts as the
    // left half, but with a 1 as a separator on the right instead of the 0 separator on the left.
    // Just the separators, with the center marked as |0| and left _0_ and right _1_:
    // After 1 step :                                  |0|
    // After 2 steps:                 0                |0|                1
    // After 3 steps:               0_0_1              |0|              0_1_1
    // After 4 steps:             001_0_011            |0|            001_1_011
    // After 5 steps:         0010011_0_0011011        |0|        0010011_1_0011011
    // After 6 steps: 001001100011011_0_001001110011011|0|001001100011011_1_001001110011011

    if let Ok(a) = usize::from_str_radix(input, 2) {
        // Let's calculate the number of lengthening steps i to reach the wanted length
        // total length = length of the inputs (2^i * input.len()) plus separator length (2^i - 1)
        let tot_len = |i| 2usize.pow(i) * (input.len() + 1) - 1;
        let mut i = 0;
        // left and right separators
        let mut left = vec![];
        let mut right = vec![];
        while tot_len(i) < min_len {
            i += 1;
            match i {
                0..=1 => { /*noop*/ }
                2 => {
                    // init
                    left.push(false);
                    right.push(true);
                }
                _ => {
                    left.push(false);
                    left.append(&mut right);
                    right = left.clone();
                    // right is the same as left, but with the middle bit 1 instead of 0
                    right[left.len() / 2] = true;
                }
            }
        }
        left.push(false);
        left.append(&mut right);
        // This extra separator makes sure that the last part of the input sequence will be
        // processed within the for-loop. It will not actually be used, and can be set to whatever.
        left.push(true);
        let separators = left;

        // The checksum of this sequence will reduce the sequence back to the original input length,
        // and each bit of the final checksum is a result of i checksum passes, over a part of the
        // sequence that is 2^i bits long. The final 2^i - 1 bits of this sequence will be discarded,
        // as this part is just one bit too short to result in a checksum bit of its own.

        // Examples with an input length of 17 bits:
        // - For min_len 272 there are 4 steps to reach a total length of 287.
        //   Each of the 17 checksum bits result from 2^4 = 16 bits of the sequence (17 * 16 = 272),
        //   and the final 15 bits are discarded (272 + 15 = 287)
        // - For min_len 35'651'584 there are 21 steps to reach a total length of 37'748'735.
        //   Each checksum bit uses 2^21 = 2'097'152 bits of the sequence (17 * 2'097'152 = 35'651'584),
        //   and the final 2'097'151 bits are discarded.

        let b = !a.reverse_bits() >> (64 - input.len());
        // println!("a = {:0len$b}, b = {:0len$b}", a, b, len = input.len());

        let wanted_bit_count = 2usize.pow(i);
        let mut actual_bit_count = 0;

        // Interestingly, the checksum of a sequence of at least 4 bits can be simplified to counting
        // the sequence's ones and zeroes. If their counts are even, the checksum is 1, otherwise 0.
        // For example:
        // 4-bit sequences yielding a checksum of 1 have four 0s and no 1s, or vice versa, or two of each:
        // - 0000, 1111, 0011, 1100 -> 11 -> 1
        // - 0101, 1010, 0110, 1001 -> 00 -> 1
        // 4-bit sequences yielding a checksum of 0 have a single 0 and three 1s, or vice versa:
        // - 1000, 0100, 0111, 1011 -> 01 -> 0
        // - 0010, 0001, 1101, 1110 -> 10 -> 0
        let mut checksum = vec![];
        let mut ones_count = 0;

        let bits_per_usize = usize::MAX.count_ones() as usize;
        'FOR_LOOP: for (i, sep) in separators.into_iter().enumerate() {
            let sep = if sep { 1 } else { 0 };
            let mut num = if i % 2 == 0 { a } else { b };
            num <<= 1;
            num |= sep;

            let mut num_bit_count = input.len() + 1;
            // println!("num = {:0len$b}", num, len = num_bit_count);

            while actual_bit_count + num_bit_count >= wanted_bit_count {
                let needed_bit_count = wanted_bit_count - actual_bit_count;

                num_bit_count -= needed_bit_count;
                let num_copy = num;

                num >>= num_bit_count;
                ones_count += num.count_ones();
                checksum.push(ones_count % 2 == 0);
                ones_count = 0;
                actual_bit_count = 0;

                if checksum.len() >= input.len() {
                    break 'FOR_LOOP;
                }

                if num_bit_count == 0 {
                    // avoid shift left with overflow
                    num = 0;
                } else {
                    let mut leftover_bits = num_copy;
                    leftover_bits <<= bits_per_usize - num_bit_count;
                    leftover_bits >>= bits_per_usize - num_bit_count;
                    num = leftover_bits;
                };
            }
            ones_count += num.count_ones();
            actual_bit_count += num_bit_count;
        }

        checksum.to_string()
    } else {
        panic!("Invalid input {}", input);
    }
}

fn generate_string_of_length(input: &str, min_len: usize) -> String {
    let mut output = input.to_string();
    while output.len() < min_len {
        output = lengthen_string(&output);
    }
    output
}

fn generate_boolvec_of_length(input: &str, min_len: usize) -> Vec<bool> {
    let mut output = input.to_boolvec();
    while output.len() < min_len {
        output = lengthen_boolvec(output);
    }
    output
}
trait ToBoolVec {
    fn to_boolvec(&self) -> Vec<bool>;
}
impl ToBoolVec for &str {
    fn to_boolvec(&self) -> Vec<bool> {
        self.chars().map(|c| c == '1').collect()
    }
}
trait BoolVecToString {
    fn to_string(&self) -> String;
}
impl BoolVecToString for Vec<bool> {
    fn to_string(&self) -> String {
        self.iter().map(|on| if *on { '1' } else { '0' }).collect()
    }
}

fn lengthen_string<T: AsRef<str>>(input: T) -> String {
    let output: String = input
        .as_ref()
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();

    format!("{}0{}", input.as_ref(), output)
}

fn lengthen_boolvec(input: Vec<bool>) -> Vec<bool> {
    let mut right: Vec<_> = input.iter().rev().map(|b| !*b).collect();
    let mut output = input;
    output.push(false);
    output.append(&mut right);
    output
}

fn calc_checksum_of_string(s: &str) -> String {
    let mut checksum = reduce_string(s);
    while checksum.len() % 2 == 0 {
        checksum = reduce_string(&checksum);
    }
    checksum
}

fn calc_checksum_of_boolvec(input: &[bool]) -> String {
    let mut checksum = reduce_boolvec(input);
    while checksum.len() % 2 == 0 {
        checksum = reduce_boolvec(&checksum);
    }
    checksum.to_string()
}

fn reduce_string(s: &str) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .step_by(2)
        .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
        .collect()
}

fn reduce_boolvec(bv: &[bool]) -> Vec<bool> {
    bv.windows(2).step_by(2).map(|p| p[0] == p[1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lengthen_data() {
        assert_eq!("100", lengthen_string("1"));
        assert_eq!("001", lengthen_string("0"));
        assert_eq!("11111000000", lengthen_string("11111"));
        assert_eq!("1111000010100101011110000", lengthen_string("111100001010"));
    }

    #[test]
    fn test_generate_string_of_length() {
        assert_eq!(
            "10000011110010000111110",
            generate_string_of_length("10000", 20)
        );
    }

    #[test]
    fn test_calc_checksum_of_string() {
        assert_eq!("100", calc_checksum_of_string("110010110100"));
    }
    #[test]
    fn test_generate_boolvec_of_length() {
        assert_eq!(
            "10000011110010000111110".to_boolvec(),
            generate_boolvec_of_length("10000", 20)
        );
    }

    #[test]
    fn test_calc_checksum_of_boolvec() {
        assert_eq!(
            "100",
            calc_checksum_of_boolvec(&"110010110100".to_boolvec())
        );
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
