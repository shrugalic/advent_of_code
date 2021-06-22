use std::collections::HashMap;

const MEM_WIDTH: usize = 36;

struct BitMask {
    force_set: [bool; MEM_WIDTH],
    force_clear: [bool; MEM_WIDTH],
}

impl Default for BitMask {
    fn default() -> Self {
        let force_set_bits = [false; MEM_WIDTH];
        let force_clear_bits = [true; MEM_WIDTH];
        BitMask {
            force_set: force_set_bits,
            force_clear: force_clear_bits,
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
            '1' => bitmask.force_set[i] = true,
            '0' => bitmask.force_clear[i] = false,
            'X' => {}
            _ => panic!("Invalid char {} in bitmask {}", c, mask),
        });
        bitmask
    }
}
impl BitMask {
    fn apply(&self, value: usize) -> usize {
        let force_set: usize = self
            .force_set
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &b)| if b { 1 << i } else { 0 })
            .sum();
        let force_clear: usize = self
            .force_clear
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &b)| if b { 1 << i } else { 0 })
            .sum();
        let mut res = value | force_set;
        // println!(
        //     "Applied force-set-bits of mask {} to value {} = {}",
        //     force_set, value, res
        // );
        res &= force_clear;
        // println!(
        //     "Applied force-clear-bits of mask {} to value {} = {}",
        //     force_clear, value, res
        // );
        res
    }
}
fn part1impl(input: &[String]) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = BitMask::default();
    for input in input {
        let input: Vec<&str> = input.split(" = ").collect();
        assert_eq!(input.len(), 2);
        if input[0] == "mask" {
            mask = BitMask::from(input[1]);
        } else {
            let loc = input[0]
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
            let mut value = input[1].parse().unwrap();
            // println!("Value for address [{}] = {}", loc, value);
            value = mask.apply(value);
            memory.insert(loc, value);
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::part1impl;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn part1_example() {
        assert_eq!(part1impl(&read_str_to_lines(EXAMPLE)), 165);
    }

    #[test]
    fn part1() {
        assert_eq!(part1impl(&read_file_to_lines("input.txt")), 6317049172545);
    }
}
