const INPUT: &str = include_str!("../input/day20.txt");

pub(crate) fn day20_part1() -> isize {
    Ring::from(INPUT).sum_of_grove_coordinates_p1()
}

pub(crate) fn day20_part2() -> isize {
    Ring::from(INPUT).sum_of_grove_coordinates_p2()
}

struct Ring {
    numbers: Vec<isize>,
}
impl From<&str> for Ring {
    fn from(input: &str) -> Self {
        let numbers: Vec<_> = input
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        Ring { numbers }
    }
}

impl Ring {
    fn sum_of_grove_coordinates_p1(self) -> isize {
        self.sum_of_grove_coordinates(1, 1)
    }
    fn sum_of_grove_coordinates_p2(self) -> isize {
        self.sum_of_grove_coordinates(811_589_153, 10)
    }
    fn sum_of_grove_coordinates(self, decryption_key: isize, mix_count: usize) -> isize {
        let len = self.numbers.len() as isize;
        let numbers: Vec<_> = self.numbers.iter().map(|n| *n * decryption_key).collect();
        // The copy to be mixed. Store the original index because it has duplicates!
        let mut mixed: Vec<_> = numbers.clone().into_iter().enumerate().collect();
        for _ in 0..mix_count {
            for (idx, num) in numbers.iter().enumerate() {
                let index = mixed
                    .iter()
                    .position(|(i, n)| n == num && i == &idx)
                    .unwrap();
                let move_count = num.abs() % (len - 1);
                let direction = num.signum();
                let mut curr = index as isize;
                for _ in 0..move_count {
                    let next = (curr + direction + len) % len;
                    mixed.swap(next as usize, curr as usize);
                    curr = next;
                }
                // println!("{num}: swapped {move_count} in direction {direction} to get {mixed:?}");
            }
        }

        let index_of_0 = mixed.iter().map(|(_, n)| n).position(|n| n == &0).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|offset| mixed[(index_of_0 + offset) % len as usize].1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4";

    #[test]
    fn part1_example() {
        assert_eq!(3, Ring::from(EXAMPLE).sum_of_grove_coordinates_p1());
    }

    #[test]
    fn part1() {
        assert_eq!(13_289, day20_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            1_623_178_306,
            Ring::from(EXAMPLE).sum_of_grove_coordinates_p2()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(2_865_721_299_243, day20_part2());
    }
}
