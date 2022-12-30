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
    let (width, bits) = parse(INPUT);
    gamma_times_epsilon(width, bits)
}

pub fn day03_part2() -> u32 {
    let (width, bits) = parse(INPUT);
    reduce_bits(width, bits)
}

fn gamma_times_epsilon(width: usize, bits: BitVec) -> u32 {
    let mut gamma = BitVec::with_capacity(width as usize);
    for i in 0..width {
        let (ones, zeroes) = count_ones_and_zeroes_at_index(width, &bits, i);
        gamma.push(ones >= zeroes);
    }
    let gamma = to_decimal(&gamma);
    let epsilon = (1 << width) - 1 - gamma; // complement of gamma
    gamma * epsilon
}

fn count_ones_and_zeroes_at_index(width: usize, bits: &BitSlice, i: usize) -> (usize, usize) {
    let ones = bits.chunks(width).filter(|&bits| bits[i]).count();
    let zeroes = bits.len() / width - ones;
    (ones, zeroes)
}

fn reduce_bits(width: usize, bits: BitVec) -> u32 {
    let og_rating = reduce(width, bits.clone(), |ones, zeroes| ones >= zeroes);
    let cs_rating = reduce(width, bits, |ones, zeroes| ones < zeroes);
    og_rating * cs_rating
}

type Filter = fn(usize, usize) -> bool;
fn reduce(width: usize, mut bits: BitVec, wanted: Filter) -> u32 {
    let mut i = 0;
    let mut start;
    let mut end = bits.len();
    while end > width {
        let (ones, zeroes) = count_ones_and_zeroes_at_index(width, &bits[0..end], i);
        start = 0;
        while start < end {
            if bits[start + i] == wanted(ones, zeroes) {
                // good -> keep this chunk, and check the next
                start += width;
            } else {
                // bad -> move the end one chunk to the left, and make sure this chunk is after it
                // (if it's not already the last one, swap it with the one after the new end)
                end -= width;
                if start < end {
                    for k in 0..width {
                        bits.swap(start + k, end + k);
                    }
                }
            }
        }
        i += 1;
    }
    to_decimal(&bits[0..end])
}

fn to_decimal(bits: &BitSlice) -> u32 {
    bits.iter()
        .map(|bit| bit.as_u32())
        .fold(0, |a, i| (a << 1) + i)
}

fn parse(input: &str) -> (usize, BitVec) {
    let mut lines_iter = input.trim().lines().peekable();
    let width = lines_iter.peek().unwrap().chars().count();
    let bits = lines_iter
        .flat_map(|line| line.chars())
        .map(|c| c == '1')
        .collect::<BitVec>();
    (width, bits)
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
        let (width, bits) = parse(EXAMPLE);
        assert_eq!(22 * 9, gamma_times_epsilon(width, bits));
    }

    #[test]
    fn example2() {
        let (width, bits) = parse(EXAMPLE);
        assert_eq!(23 * 10, reduce_bits(width, bits));
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
