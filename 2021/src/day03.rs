use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;

const INPUT: &str = include_str!("../input/day03.txt");

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// run this with
/// ```sh
/// cargo run --bin day03 --release --features dhat-heap
/// ```
/// to see the heap output
#[allow(unused)]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    assert_eq!(day03_part1(), 284 * 3811);
}

pub fn day03_part1() -> u32 {
    let numbers = BinaryNumberLines::from(INPUT);
    numbers.gamma_times_epsilon()
}

pub fn day03_part2() -> u32 {
    let mut numbers = BinaryNumberLines::from(INPUT);
    numbers.calculate_life_support_rating()
}

impl BinaryNumberLines {
    fn gamma_times_epsilon(&self) -> u32 {
        let gamma = self.decimal_value_of_most_frequent_bit_at_each_position();
        let epsilon = (1 << self.line_width) - 1 - gamma; // complement of gamma
        gamma * epsilon
    }
    fn decimal_value_of_most_frequent_bit_at_each_position(&self) -> u32 {
        let mut gamma = BitVec::with_capacity(self.line_width as usize);
        for i in 0..self.line_width {
            let (ones, zeroes) = self.count_ones_and_zeroes_at_index(i, &self.bits);
            gamma.push(ones >= zeroes);
        }
        to_decimal(&gamma)
    }
    fn count_ones_and_zeroes_at_index(&self, i: usize, bits: &BitSlice) -> (usize, usize) {
        let ones = bits.chunks(self.line_width).filter(|&bits| bits[i]).count();
        let zeroes = self.line_count() - ones;
        (ones, zeroes)
    }

    fn calculate_life_support_rating(&mut self) -> u32 {
        let oxygen_rating = self.reduce_to_single_line_value(|ones, zeroes| ones >= zeroes);
        let co2_scrubber_rating = self.reduce_to_single_line_value(|ones, zeroes| ones < zeroes);
        oxygen_rating * co2_scrubber_rating
    }
    fn reduce_to_single_line_value(&mut self, wanted: Filter) -> u32 {
        let mut line_indices: Vec<_> = (0..self.line_count()).into_iter().collect();
        for bit_idx in 0..self.line_width {
            let (ones, zeroes) = self.count_ones_and_zeroes_of_lines(&line_indices, &bit_idx);
            line_indices.retain(|l| wanted(ones, zeroes) == self.bit_at(l, &bit_idx));
            if line_indices.len() == 1 {
                return to_decimal(self.line_at(&line_indices[0]));
            }
        }
        unreachable!()
    }
    fn count_ones_and_zeroes_of_lines(
        &self,
        line_idx: &[usize],
        bit_idx: &usize,
    ) -> (usize, usize) {
        line_idx.iter().fold((0, 0), |(ones, zeroes), line_idx| {
            if self.bit_at(line_idx, bit_idx) {
                (ones + 1, zeroes)
            } else {
                (ones, zeroes + 1)
            }
        })
    }
    fn line_at(&self, line_idx: &usize) -> &BitSlice {
        &self.bits[line_idx * self.line_width..line_idx * self.line_width + self.line_width]
    }
    fn bit_at(&self, line_idx: &usize, bit_idx: &usize) -> bool {
        self.line_at(line_idx)[*bit_idx]
    }
    fn line_count(&self) -> usize {
        self.bits.len() / self.line_width
    }
}

type Filter = fn(usize, usize) -> bool;

fn to_decimal(bits: &BitSlice) -> u32 {
    bits.iter()
        .map(|bit| bit.as_u32())
        .fold(0, |a, i| (a << 1) + i)
}

struct BinaryNumberLines {
    bits: BitVec,
    line_width: usize,
}

impl From<&str> for BinaryNumberLines {
    fn from(input: &str) -> Self {
        let mut lines_iter = input.trim().lines().peekable();
        let line_width = lines_iter.peek().unwrap().chars().count();
        let bits = lines_iter
            .flat_map(|line| line.chars())
            .map(|c| c == '1')
            .collect::<BitVec>();
        BinaryNumberLines { bits, line_width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn example1() {
        let numbers = BinaryNumberLines::from(EXAMPLE);
        assert_eq!(22 * 9, numbers.gamma_times_epsilon());
    }

    #[test]
    fn example2() {
        let mut numbers = BinaryNumberLines::from(EXAMPLE);
        assert_eq!(23 * 10, numbers.calculate_life_support_rating());
    }

    #[test]
    fn part1() {
        assert_eq!(day03_part1(), 284 * 3811);
    }

    #[test]
    fn part2() {
        assert_eq!(day03_part2(), 486 * 2784);
    }
}
