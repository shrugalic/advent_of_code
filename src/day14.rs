use std::collections::HashMap;

const MEM_WIDTH: usize = 36;

enum MaskValue {
    Zero,
    One,
    X,
}
impl From<char> for MaskValue {
    fn from(c: char) -> Self {
        match c {
            '0' => MaskValue::Zero,
            '1' => MaskValue::One,
            'X' => MaskValue::X,
            c => panic!("Illegal mask char {}", c),
        }
    }
}
#[derive(Debug)]
struct BitMask {
    ones: [bool; MEM_WIDTH],
    zeroes: [bool; MEM_WIDTH],
    floatings: [bool; MEM_WIDTH],
}

impl Default for BitMask {
    fn default() -> Self {
        BitMask {
            ones: [false; MEM_WIDTH],
            zeroes: [true; MEM_WIDTH],
            floatings: [false; MEM_WIDTH],
        }
    }
}

impl<T> From<T> for BitMask
where
    T: AsRef<str>,
{
    fn from(mask: T) -> Self {
        let mask = mask.as_ref();
        assert_eq!(mask.len(), 36);
        let mut bitmask = BitMask::default();
        mask.chars().enumerate().for_each(|(i, c)| match c {
            '1' => bitmask.ones[i] = true,
            '0' => bitmask.zeroes[i] = false,
            'X' => bitmask.floatings[i] = true,
            _ => panic!("Invalid char {} in bitmask {}", c, mask),
        });
        bitmask
    }
}
impl BitMask {
    fn apply_part1_logic(&self, value: usize) -> usize {
        let force_set = BitMask::value_from(&self.ones);
        let force_clear = BitMask::value_from(&self.zeroes);
        let mut res = value | force_set;
        res &= force_clear;
        res
    }

    fn get_memory_locations(&self, mem_loc: usize) -> Vec<usize> {
        let force_set = BitMask::value_from(&self.ones);
        let mem_loc = mem_loc | force_set;
        let floating_indices: Vec<_> = self
            .floatings
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, &b)| {
                if b {
                    // println!("floating at {}", i);
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        if floating_indices.is_empty() {
            return vec![mem_loc];
        }
        let floating_count = floating_indices.len(); // 2 -> 00, 01, 10, 11
        let mut mem_locs = vec![mem_loc; 1 << floating_count];
        // println!(
        //     "{} floatings -> need {} locs",
        //     floating_count,
        //     1 << floating_count
        // );

        // go through each index…
        // …enough times
        for (n, mem_loc) in mem_locs.iter_mut().enumerate() {
            for (i, floating_index) in floating_indices.iter().enumerate() {
                let floating_bit = 1 << floating_index;
                let pos = 1 << i;
                let set = n & pos == 0;
                // println!(
                //     "{}, n = {}, i = {}, floating {}, pos = {}",
                //     if set { "  set" } else { "unset" },
                //     n,
                //     i,
                //     floating_index,
                //     pos
                // );
                // use the set or unset version the appropriate amount of times
                // that is every 1, 2, 4, 8,… times

                *mem_loc = if set {
                    // single 1 bit, 0 everywhere else
                    floating_bit | *mem_loc
                } else {
                    let force_idx = floating_bit ^ usize::MAX; // single 0 bit, 1 everywhere else
                    force_idx & *mem_loc
                };

                // println!(
                //     "{}|{} ({:6b}): {:6b} -> set {:6b}, unset {:6b}",
                //     n, floating_index, floating_bit, mem_loc, force_set, force_unset
                // );
            }
        }
        // mem_locs
        //     .iter()
        //     .enumerate()
        //     .for_each(|(i, mem_loc)| println!("{}: {:6b}", i, mem_loc));
        mem_locs
    }

    fn value_from(bits: &[bool]) -> usize {
        bits.iter()
            .rev()
            .enumerate()
            .map(|(i, &b)| if b { 1 << i } else { 0 })
            .sum()
    }
}
pub(crate) fn day14_part1impl(input: &[String]) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = BitMask::default();
    for input in input {
        match split_input(input) {
            (Some(bitmask), None) => mask = bitmask,
            (None, Some((loc, value))) => {
                memory.insert(loc, mask.apply_part1_logic(value));
            }
            x => panic!("Invalid input {:?}", x),
        }
    }
    memory.values().sum()
}

fn split_input(input: &str) -> (Option<BitMask>, Option<(usize, usize)>) {
    let input: Vec<&str> = input.split(" = ").collect();
    assert_eq!(input.len(), 2);
    if input[0] == "mask" {
        (Some(BitMask::from(input[1])), None)
    } else {
        let mem_addr = input[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        let value = input[1].parse().unwrap();
        // println!("Value for address [{}] = {}", loc, value);
        (None, Some((mem_addr, value)))
    }
}

pub(crate) fn day14_part2impl(input: &[String]) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = BitMask::default();
    for input in input {
        match split_input(input) {
            (Some(bitmask), None) => mask = bitmask,
            (None, Some((mem_addr, value))) => {
                mask.get_memory_locations(mem_addr)
                    .into_iter()
                    .for_each(|mem_loc| {
                        memory.insert(mem_loc, value);
                    });
            }
            x => panic!("Invalid input {:?}", x),
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{day14_part1impl, day14_part2impl};
    use crate::line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn part1_example() {
        assert_eq!(day14_part1impl(&read_str_to_lines(EXAMPLE1)), 165);
    }

    #[test]
    fn part1() {
        assert_eq!(
            day14_part1impl(&read_file_to_lines("input/day14.txt")),
            6317049172545
        );
    }

    const EXAMPLE2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    #[test]
    fn part2_example_1() {
        assert_eq!(day14_part2impl(&read_str_to_lines(EXAMPLE2)), 208);
    }

    #[test]
    fn part2() {
        assert_eq!(
            day14_part2impl(&read_file_to_lines("input/day14.txt"),),
            3434009980379
        );
    }
}
