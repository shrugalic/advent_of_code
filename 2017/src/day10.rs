use line_reader::read_file_to_lines;
use std::collections::VecDeque;

pub(crate) fn day10_part1() -> usize {
    part1_hash_checksum(255, read_file_to_lines("input/day10.txt"))
}

pub(crate) fn day10_part2() -> String {
    part2_hash(read_file_to_lines("input/day10.txt"))
}

struct Hasher {
    ring: VecDeque<u8>,
    lengths: Vec<u8>,
    curr_pos: usize,
    skip_size: usize,
    orig_start_pos: usize,
}

impl Hasher {
    fn new(max_idx: u8, lengths: Vec<u8>) -> Self {
        Hasher {
            ring: (0..=max_idx).into_iter().collect(),
            lengths,
            curr_pos: 0,
            skip_size: 0,
            orig_start_pos: 0,
        }
    }

    fn do_hash_cycle(&mut self) {
        for length in self.lengths.clone().into_iter() {
            let length = length as usize;
            let end_of_range = self.adjust_into_non_wrapping_range(length);

            self.reverse_range(self.curr_pos, end_of_range);

            self.curr_pos = (self.curr_pos + length + self.skip_size) % self.ring.len();
            self.skip_size += 1;
        }
    }

    fn adjust_into_non_wrapping_range(&mut self, length: usize) -> usize {
        let mut end_of_range = (self.curr_pos + length) % self.ring.len();
        if end_of_range <= self.curr_pos {
            // This range wraps around the ring, which makes using part of it as a slice impossible.
            // Fix this by rotating the ring such that curr_pos is at index 0.
            // And keep track of the original starting index in orig_start_pos
            let is_rotating_left_more_efficient = self.curr_pos < self.ring.len() / 2;
            if is_rotating_left_more_efficient {
                self.ring.rotate_left(self.curr_pos);
                self.orig_start_pos = (self.orig_start_pos + self.curr_pos) % self.ring.len();
            } else {
                let from_right = self.ring.len() - self.curr_pos;
                self.ring.rotate_right(from_right);
                self.orig_start_pos = (self.orig_start_pos + from_right) % self.ring.len();
            }
            self.curr_pos = 0;
            end_of_range = length;
        }
        end_of_range
    }

    fn reverse_range(&mut self, start: usize, end: usize) {
        self.ring.make_contiguous();
        let part = self.ring.as_mut_slices().0;
        part[start..end].reverse();
    }

    fn sparse_hash(&mut self) -> String {
        self.rotate_to_start_idx();
        self.ring.make_contiguous();
        let sparse_hash = Hasher::sparse_hash_of(self.ring.as_slices().0);
        Hasher::dense_hash_of(&sparse_hash)
    }

    /// Rotate such that start_idx == 0
    fn rotate_to_start_idx(&mut self) {
        self.ring.rotate_left(self.orig_start_pos);
        self.curr_pos = (self.ring.len() + self.curr_pos - self.orig_start_pos) % self.ring.len();
        self.orig_start_pos = 0;
    }

    fn sparse_hash_of(ring: &[u8]) -> Vec<u8> {
        ring.chunks(16)
            .map(|chunk| {
                assert_eq!(16, chunk.len());
                chunk.iter().fold(0, |a, b| a ^ b)
            })
            .collect()
    }

    fn dense_hash_of(sparse_hash: &[u8]) -> String {
        sparse_hash
            .iter()
            .map(|num| format!("{:02x}", num))
            .collect()
    }
}

fn part1_hash_checksum(max_idx: u8, input: Vec<String>) -> usize {
    let lengths = input[0].split(',').map(|i| i.parse().unwrap()).collect();
    let mut hasher = Hasher::new(max_idx, lengths);
    hasher.do_hash_cycle();
    hasher.rotate_to_start_idx();
    hasher.ring[0] as usize * hasher.ring[1] as usize
}

fn part2_hash(input: Vec<String>) -> String {
    let mut lengths: Vec<u8> = input[0].chars().map(|c| c as u8).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut hasher = Hasher::new(255, lengths);
    (0..64).into_iter().for_each(|_| {
        hasher.do_hash_cycle();
    });
    hasher.sparse_hash()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    #[test]
    fn part1_example() {
        assert_eq!(12, part1_hash_checksum(4, read_str_to_lines("3,4,1,5")));
    }

    #[test]
    fn part1_full() {
        assert_eq!(212, day10_part1());
    }

    #[test]
    fn part2_sparse_hash() {
        assert_eq!(
            vec![64],
            Hasher::sparse_hash_of(&[65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22])
        );
    }

    #[test]
    fn part2_dense_hash() {
        assert_eq!("4007ff", Hasher::dense_hash_of(&[64, 7, 255]));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272",
            part2_hash(read_str_to_lines(""))
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            part2_hash(read_str_to_lines("AoC 2017"))
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            part2_hash(read_str_to_lines("1,2,3"))
        );
    }

    #[test]
    fn part2_example4() {
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            part2_hash(read_str_to_lines("1,2,4"))
        );
    }

    #[test]
    fn part2_full() {
        assert_eq!("96de9657665675b51cd03f0b3528ba26", day10_part2());
    }
}
