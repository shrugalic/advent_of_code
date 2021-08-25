use std::collections::HashMap;
const DAY6_INPUT: [usize; 16] = [2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];

pub(crate) fn day6_part1() -> usize {
    count_reallocation_cycles(DAY6_INPUT.to_vec()).0
}

pub(crate) fn day6_part2() -> usize {
    count_reallocation_cycles(DAY6_INPUT.to_vec()).1
}

fn count_reallocation_cycles(banks: Vec<usize>) -> (usize, usize) {
    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut config = banks;
    let mut counter = 0;
    while !seen.contains_key(&config) {
        // println!("{} {:?}", counter, config);
        seen.insert(config.clone(), counter);
        let max = *config.iter().max().unwrap();
        let mut pos = config.iter().position(|c| c == &max).unwrap();
        config[pos] = 0;

        let mut block_count = max;
        while block_count > 0 {
            pos = (pos + 1) % config.len();
            config[pos] += 1;
            block_count -= 1;
        }
        counter += 1;
    }
    (counter, counter - seen.get(&config).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{count_reallocation_cycles, DAY6_INPUT};

    #[test]
    fn example_part1() {
        assert_eq!(5, count_reallocation_cycles(vec![0, 2, 7, 0]).0);
    }

    #[test]
    fn part1() {
        assert_eq!(3156, count_reallocation_cycles(DAY6_INPUT.to_vec()).0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(4, count_reallocation_cycles(vec![0, 2, 7, 0]).1);
    }

    #[test]
    fn part2() {
        assert_eq!(1610, count_reallocation_cycles(DAY6_INPUT.to_vec()).1);
    }
}
