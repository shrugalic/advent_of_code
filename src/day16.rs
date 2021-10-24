const PUZZLE_INPUT: &str = "11100010111110100";

pub(crate) fn day16_part1() -> String {
    checksum_of_data_generated_to_len(PUZZLE_INPUT, 272)
}

pub(crate) fn day16_part2() -> String {
    checksum_of_data_generated_to_len(PUZZLE_INPUT, 35_651_584)
}

enum Method {
    Strings,
    BoolVec,
    UsizeBits,
}
fn checksum_of_data_generated_to_len(input: &str, min_len: usize) -> String {
    let method = Method::UsizeBits;
    match method {
        Method::Strings => {
            let data = generate_string_of_length(input, min_len);
            calc_checksum_of_string(&data[0..min_len])
        }
        Method::BoolVec => {
            let data = generate_boolvec_of_length(input, min_len);
            calc_checksum_of_boolvec(&data[0..min_len])
        }
        Method::UsizeBits => checksum_of_data_generated_to_len_with_usize_bits(input, min_len),
    }
}

fn checksum_of_data_generated_to_len_with_usize_bits(input: &str, min_len: usize) -> String {
    let mut curr_len = input.len(); // count now because leading zeroes won't be counted later
    if let Ok(mut num) = usize::from_str_radix(input, 2) {
        println!("{} = {:b}", num, num);

        // enlarge to >= min_len
        while curr_len < min_len {
            let left = num;
            if curr_len > usize::MAX.count_ones() as usize {
                unimplemented!("Number became too large!")
            }
            let right = !num.reverse_bits() >> (64 - curr_len);
            num <<= curr_len + 1;
            num += right;
            println!(
                "{:0length$b} = {:0length$b} + 0 + {:0length$b}",
                num,
                left,
                right,
                length = curr_len
            );
            curr_len = 2 * curr_len + 1;
        }
        // cut to min_len
        num >>= curr_len - min_len;
        curr_len = min_len;
        println!(
            "{:0length$b}: cut to len {}\n",
            num,
            min_len,
            length = curr_len
        );

        // shrink to odd-length checksum
        let mut checksum = num;
        while curr_len % 2 == 0 {
            let mut new = 0usize;
            let mut len = curr_len;
            while len > 0 {
                new <<= 1;
                new += if checksum & 1 == (checksum & 2) >> 1 {
                    1
                } else {
                    0
                };
                checksum >>= 2;
                len -= 2;
                println!(
                    "{}{:0len1$b} num | new {:0len2$b}",
                    " ".repeat(curr_len - len),
                    checksum,
                    new,
                    len1 = len,
                    len2 = (curr_len / 2) - len / 2
                );
            }
            curr_len /= 2;
            checksum = new.reverse_bits() >> (64 - curr_len);
            println!(
                "\n{:0length$b} num of len {}\n",
                checksum,
                curr_len,
                length = curr_len
            );
        }
        println!("{:0length$b} checksum", checksum, length = curr_len);

        format!("{:0length$b}", checksum, length = curr_len)
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
