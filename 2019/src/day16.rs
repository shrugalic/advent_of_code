use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fmt;

pub(crate) fn day16_part1() -> String {
    let fft = FlawedFrequencyTransmission::from(day_16_puzzle_input());
    fft.check_sum(100)
}

pub(crate) fn day16_part2() -> String {
    let mut fft = FlawedFrequencyTransmission::from(day_16_puzzle_input());
    fft.message(100)
}

const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];
const SIGNAL_REPETITION_COUNT: usize = 10_000;

#[derive(Debug)]
struct Pattern {
    pattern: Vec<i8>,
}
impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.pattern
                .iter()
                .map(|digit| digit.to_string())
                .collect::<String>()
        )
    }
}
impl Pattern {
    fn base() -> Self {
        Pattern {
            pattern: BASE_PATTERN.to_vec(),
        }
    }
    fn new(pattern: &[i8]) -> Self {
        Pattern {
            pattern: pattern.to_vec(),
        }
    }
    fn repeat_each_digit(&self, count: usize) -> Pattern {
        Pattern::new(
            &self
                .pattern
                .iter()
                .flat_map(|&digit| (&[digit].repeat(count)).clone())
                .collect::<Vec<i8>>(),
        )
    }
    fn to_length(&self, wanted_len: usize) -> Vec<i8> {
        let mut pattern = self.pattern.clone();
        if pattern.len() - 1 < wanted_len {
            // -1 / +1 because we'll be omitting the very first value of the final pattern below
            let factor = ((wanted_len + 1) as f64 / pattern.len() as f64).ceil() as usize;
            pattern = pattern.repeat(factor);
        }
        // omit first value and cut to size
        pattern.into_iter().skip(1).take(wanted_len).collect()
    }
    fn for_pos(&self, pos: usize, list_len: usize) -> Vec<i8> {
        self.repeat_each_digit(pos).to_length(list_len)
    }
}
#[derive(Debug)]
struct FlawedFrequencyTransmission {
    list: Vec<u8>,
    pattern: Pattern,
    offset: usize,
}
impl From<&str> for FlawedFrequencyTransmission {
    fn from(input: &str) -> Self {
        let list: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let offset: usize = list
            .iter()
            .take(7)
            .map(|digit| digit.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        FlawedFrequencyTransmission {
            list,
            pattern: Pattern::base(),
            offset,
        }
    }
}
impl fmt::Display for FlawedFrequencyTransmission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.check_sum(0))
    }
}
impl FlawedFrequencyTransmission {
    // Verify that the FFT works
    fn check_sum(&self, phase_count: usize) -> String {
        let mut list = self.list.clone();
        for _ in 0..phase_count {
            list = FlawedFrequencyTransmission::apply_pattern(&list, &self.pattern);
        }
        list.iter()
            .take(8)
            .map(|digit| digit.to_string())
            .collect::<String>()
    }
    fn apply_pattern(list: &[u8], pattern: &Pattern) -> Vec<u8> {
        list.par_iter() // parallelism out here helps, below it doesn't
            .enumerate()
            .map(|(idx, _c)| {
                let pos = idx + 1; // pos starts at 1, idx at 0
                let pattern = pattern.for_pos(pos, list.len());
                let sum: isize = list
                    .iter()
                    .zip(pattern)
                    .map(|(&d, p)| d as isize * p as isize)
                    .sum();
                (sum % 10).abs() as u8
            })
            .collect()
    }
    fn message(&mut self, phase_count: usize) -> String {
        if 2 * self.offset < SIGNAL_REPETITION_COUNT * self.list.len() {
            // The offset places the signal in the first half of the list,
            // which makes the pattern more complicated
            println!("going slow path");
            let mut list = self.list.repeat(SIGNAL_REPETITION_COUNT);
            for _ in 0..phase_count {
                list = FlawedFrequencyTransmission::apply_pattern(&list, &self.pattern);
            }
            list.iter()
                .skip(self.offset)
                .take(8)
                .map(|digit| digit.to_string())
                .collect::<String>()
        } else {
            // The offset places the signal in the second half, in which
            // the pattern will be all ones, and the problem thus simplified
            if false {
                self.apply_adjusted_matrix_a_single_time(phase_count)
            } else {
                self.apply_matrix_several_times(phase_count)
            }
        }
    }
    // this works but takes about one hour!
    fn apply_adjusted_matrix_a_single_time(&self, phase_count: usize) -> String {
        let list: Vec<u8> = self
            .list
            .repeat(SIGNAL_REPETITION_COUNT)
            .par_iter()
            .skip(self.offset)
            .cloned()
            .collect();
        if list.len() < 100 {
            println!("list = {:?}", list);
        }
        if phase_count == 0 {
            list.iter().take(8).map(|digit| digit.to_string()).collect()
        } else {
            println!("size = {}", list.len());

            let mut matrix_line =
                FlawedFrequencyTransmission::top_line_of_adjusted_matrix(list.len(), phase_count);
            if list.len() < 100 {
                println!("top_line = {:?}", matrix_line);
            }

            // multiply by using top line rather than matrix that uses too much memory
            let mut result = vec![];
            while result.len() < list.len() {
                let value = list
                    .par_iter()
                    .zip(matrix_line.par_iter())
                    .map(|(&digit, &matrix)| (digit as usize * matrix as usize) % 10)
                    .sum::<usize>();
                let value = (value % 10) as u8;
                result.push(value);
                matrix_line.pop_back();
                matrix_line.push_front(0);
            }
            if false {
                FlawedFrequencyTransmission::print_counts(&matrix_line);
            }
            result
                .iter()
                .take(8)
                .map(|digit| digit.to_string())
                .collect()
        }
    }
    fn apply_matrix_several_times(&self, phase_count: usize) -> String {
        let mut result: Vec<u8> = self
            .list
            .repeat(SIGNAL_REPETITION_COUNT)
            .iter()
            .skip(self.offset)
            .cloned()
            .collect();
        if false {
            // slow matrix multiplication
            let matrix = self.build_matrix();
            for _ in 0..phase_count {
                result = FlawedFrequencyTransmission::multiply(&matrix, &result);
            }
            result
                .iter()
                .take(8)
                .map(|digit| digit.to_string())
                .collect()
        } else {
            let mut list: Vec<u8> = self
                .list
                .repeat(SIGNAL_REPETITION_COUNT)
                .iter()
                .skip(self.offset)
                .cloned()
                .collect();
            for _ in 0..phase_count {
                let mut sum: u8 = 0;
                list.iter_mut()
                    .rev() // start from the back
                    .for_each(|i| {
                        sum = (sum + *i) % 10_u8;
                        *i = sum
                    });
            }
            list.iter().take(8).map(|digit| digit.to_string()).collect()
        }
    }
    fn top_line_of_adjusted_matrix(size: usize, exponent: usize) -> VecDeque<u8> {
        // Rather than multiplying a matrix of the form
        //   1 1 1 1 1
        //   0 1 1 1 1
        //   0 0 1 1 1
        //   0 0 0 1 1
        //   0 0 0 0 1
        // with itself, construct its top-most line in an efficient way, and use this to generate the rest,
        // because it keeps its pattern. For example, for a top line of 1 2 3 4 5…, the matrix will be
        //   1 2 3 4 5
        //   0 1 2 3 4
        //   0 0 1 2 3
        //   0 0 0 1 2
        //   0 0 0 0 1
        // This example is the 2nd power of the 11111 matrix, generated with phase_count *1*.
        let mut line: Vec<u8> = vec![1; size];
        for _ in 1..exponent {
            let mut sum = 0;
            let next_line = line
                .iter()
                .map(|&i| {
                    sum = (sum + i) % 10;
                    sum
                })
                .collect();
            line = next_line;
        }
        VecDeque::from(line)
    }
    fn build_matrix(&self) -> Vec<Vec<u8>> {
        let size = self.list.len() * SIGNAL_REPETITION_COUNT - (self.offset - 1);
        println!("size = {}", size);
        let mut matrix: Vec<Vec<u8>> = vec![vec![0; size]; size];
        matrix.par_iter_mut().enumerate().for_each(|(y, row)| {
            row.par_iter_mut().enumerate().for_each(|(x, value)| {
                if x >= y {
                    *value = 1;
                }
            });
            // println!("row = {:?}", row);
        });
        matrix
    }
    fn multiply(mat: &[Vec<u8>], vec: &[u8]) -> Vec<u8> {
        mat.iter()
            .map(|row| {
                println!("mat   = {:?}", row);
                println!("vec   = {:?}", vec);
                (row.iter()
                    .zip(vec.iter())
                    .map(|(&m, &value)| m as usize * value as usize)
                    .sum::<usize>()
                    % 10) as u8
            })
            .collect::<Vec<u8>>()
    }
    fn print_counts(matrix_line: &VecDeque<u8>) {
        // counts:
        // 0: 367'093
        // 6: 91'767
        // 5: 52'419
        // 1: 13'133
        let counts = matrix_line.iter().fold(HashMap::new(), |mut map, digit| {
            *map.entry(digit).or_insert(0) += 1;
            map
        });
        println!("counts = {:?} = ", counts);
    }
}

fn day_16_puzzle_input() -> &'static str {
    "59755896917240436883590128801944128314960209697748772345812613779993681653921392130717892227131006192013685880745266526841332344702777305618883690373009336723473576156891364433286347884341961199051928996407043083548530093856815242033836083385939123450194798886212218010265373470007419214532232070451413688761272161702869979111131739824016812416524959294631126604590525290614379571194343492489744116326306020911208862544356883420805148475867290136336455908593094711599372850605375386612760951870928631855149794159903638892258493374678363533942710253713596745816693277358122032544598918296670821584532099850685820371134731741105889842092969953797293495"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(
            FlawedFrequencyTransmission::from("12345678").list,
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
    #[test]
    fn pattern_repeat_each_digit_once() {
        assert_eq!(
            Pattern::base().repeat_each_digit(1).pattern,
            vec![0, 1, 0, -1]
        );
    }
    #[test]
    fn pattern_repeat_each_digit_twice() {
        assert_eq!(
            Pattern::base().repeat_each_digit(2).pattern,
            vec![0, 0, 1, 1, 0, 0, -1, -1]
        );
    }
    #[test]
    fn pattern_repeat_each_digit_thrice() {
        assert_eq!(
            Pattern::base().repeat_each_digit(3).pattern,
            vec![0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );
    }
    #[test]
    fn base_pattern_to_length_2() {
        assert_eq!(Pattern::base().to_length(2), vec![1, 0]);
    }
    #[test]
    fn base_pattern_to_lengthen_4() {
        assert_eq!(Pattern::base().to_length(4), vec![1, 0, -1, 0]);
    }
    #[test]
    fn base_pattern_to_length_8() {
        assert_eq!(Pattern::base().to_length(8), vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }
    #[test]
    fn pattern_for_pos_1() {
        assert_eq!(
            Pattern::base().for_pos(1, 8),
            vec![1, 0, -1, 0, 1, 0, -1, 0]
        );
    }
    #[test]
    fn pattern_for_pos_2() {
        assert_eq!(
            Pattern::base().for_pos(2, 8),
            vec![0, 1, 1, 0, 0, -1, -1, 0]
        );
    }
    #[test]
    fn pattern_for_pos_3() {
        assert_eq!(Pattern::base().for_pos(3, 8), vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }
    #[test]
    fn day_16_initial_fft() {
        assert_eq!(
            FlawedFrequencyTransmission::from("12345678").check_sum(0),
            "12345678"
        );
    }
    #[test]
    fn day_16_part_1_fft_after_phase_1() {
        let fft = FlawedFrequencyTransmission::from("12345678");
        assert_eq!(fft.check_sum(1), "48226158");
    }
    #[test]
    fn day_16_part_1_fft_after_phase_4() {
        let fft = FlawedFrequencyTransmission::from("12345678");
        assert_eq!(fft.check_sum(4), "01029498");
    }
    #[test]
    fn day_16_part_1_fft_after_phase_8() {
        let fft = FlawedFrequencyTransmission::from("12345678");
        assert_eq!(fft.check_sum(8), "38765018");
    }
    #[test]
    fn day_16_part_1_large_example_1() {
        let fft = FlawedFrequencyTransmission::from("80871224585914546619083218645595");
        assert_eq!(fft.check_sum(100), "24176176");
    }
    #[test]
    fn day_16_part_1_large_example_2() {
        let fft = FlawedFrequencyTransmission::from("19617804207202209144916044189917");
        assert_eq!(fft.check_sum(100), "73745418");
    }
    #[test]
    fn day_16_part_1_large_example_3() {
        let fft = FlawedFrequencyTransmission::from("69317163492948606335995924319873");
        assert_eq!(fft.check_sum(100), "52432133");
    }
    #[test]
    fn day_16_part_1() {
        assert_eq!(day16_part1(), "78009100");
    }
    #[test]
    fn day_16_part_2_example_1_offset() {
        let fft = FlawedFrequencyTransmission::from("03036732577212944063491565474664");
        assert_eq!(fft.offset, 303673);
    }
    #[test]
    fn day_16_part_2_example_2_offset() {
        let fft = FlawedFrequencyTransmission::from("02935109699940807407585447034323");
        assert_eq!(fft.offset, 293510);
    }
    #[test]
    fn day_16_part_2_example_3_offset() {
        let fft = FlawedFrequencyTransmission::from("03081770884921959731165446850517");
        assert_eq!(fft.offset, 308177);
    }
    #[test]
    fn day_16_part_2_simple_test_0() {
        let mut fft = FlawedFrequencyTransmission::from("014999211111111");
        assert_eq!(fft.message(0), "11111111");
    }
    #[test]
    fn day_16_part_2_simple_test_1() {
        let mut fft = FlawedFrequencyTransmission::from("014999211111111");
        assert_eq!(fft.message(1), "87654321");
    }
    #[test]
    fn day_16_part_2_simple_test_2() {
        let mut fft = FlawedFrequencyTransmission::from("014999211111111");
        assert_eq!(fft.message(2), "68150631");
    }
    #[test]
    fn day_16_part_2_simple_test_3() {
        let mut fft = FlawedFrequencyTransmission::from("014999211111111");
        assert_eq!(fft.message(3), "04650041");
    }
    #[test]
    fn day_16_part_2_simple_test_0_non_last() {
        let mut fft = FlawedFrequencyTransmission::from("014998411111111");
        assert_eq!(fft.message(0), "10149984");
    }
    #[test]
    fn day_16_part_2_simple_test_1_non_last() {
        let mut fft = FlawedFrequencyTransmission::from("014998411111111");
        assert_eq!(fft.message(1), "43328902");
    }
    #[test]
    fn day_16_part_2_simple_test_2_non_last() {
        let mut fft = FlawedFrequencyTransmission::from("014998411111111");
        assert_eq!(fft.message(2), "73075788");
    }
    #[test]
    fn day_16_part_2_simple_test_3_non_last() {
        let mut fft = FlawedFrequencyTransmission::from("014998411111111");
        assert_eq!(fft.message(3), "58558368");
    }
    #[test]
    fn day_16_part_2_example_1() {
        let mut fft = FlawedFrequencyTransmission::from("03036732577212944063491565474664");
        assert_eq!(fft.message(100), "84462026");
    }
    #[test]
    fn day_16_part_2_example_2() {
        let mut fft = FlawedFrequencyTransmission::from("02935109699940807407585447034323");
        assert_eq!(fft.message(100), "78725270");
    }
    #[test]
    fn day_16_part_2_example_3() {
        let mut fft = FlawedFrequencyTransmission::from("03081770884921959731165446850517");
        assert_eq!(fft.message(100), "53553731");
    }
    #[test]
    fn day_16_part_2() {
        assert_eq!(day16_part2(), "37717791");
    }

    #[test]
    fn top_line_of_powered_matrix_pow_1() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 1),
            vec![1, 1, 1, 1, 1, 1, 1, 1]
        );
    }
    #[test]
    fn top_line_of_powered_matrix_pow_2() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 2),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
    #[test]
    fn top_line_of_powered_matrix_pow_3() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 3),
            vec![1, 3, 6, 0, 5, 1, 8, 6] // 1, 3, 6, 10, 15, 21, 28, 36
        );
    }
    #[test]
    fn top_line_of_powered_matrix_pow_4() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 4),
            vec![1, 4, 0, 0, 5, 6, 4, 0] // 1, 4, 10, 20, 35, 56, 84, 120
        );
    }
    #[test]
    fn top_line_of_powered_matrix_pow_5() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 5),
            vec![1, 5, 5, 5, 0, 6, 0, 0] // 1, 5, 15, 35, 70, 126, 210, 330
        );
    }
    #[test]
    fn top_line_of_powered_matrix_pow_11() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 11),
            vec![1, 1, 6, 6, 1, 3, 8, 8] // 1, 11, 66, 286, 1001, 3003, 8008, 19448
        );
    }
    #[test]
    fn top_line_of_powered_matrix_pow_20() {
        assert_eq!(
            FlawedFrequencyTransmission::top_line_of_adjusted_matrix(8, 20),
            vec![1, 0, 0, 0, 5, 4, 0, 0] // 1, 20, 210, 1540, 8855, 42504, 177100, 657800
        );
    }
}
