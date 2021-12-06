const INPUT: &str = include_str!("../input/day06.txt");

pub(crate) fn day06_part1() -> usize {
    let timers = parse(INPUT);
    multiply(timers, 80)
}

pub(crate) fn day06_part2() -> usize {
    let timers = parse(INPUT);
    multiply(timers, 256)
}

fn multiply(timers: Vec<usize>, generations: usize) -> usize {
    // Index equals timer value, so index 0 contains the count of fish with timer 0
    let mut counts_by_timer = vec![0usize; 9];
    timers.into_iter().for_each(|f| {
        counts_by_timer[f] += 1;
    });
    for _ in 0..generations {
        // A left rotation represents the timer (=index) decreasing by 1.
        // The fish with timer 0 will not only produce new fish with timer 8,
        // but also reset their timer to 6
        let count_of_fish_with_timer_0 = counts_by_timer[0];
        counts_by_timer.rotate_left(1);
        counts_by_timer[6] += count_of_fish_with_timer_0; // == counts_by_timer[8]
    }
    counts_by_timer.into_iter().sum()
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn part1_example() {
        assert_eq!(26, multiply(parse(EXAMPLE), 18));
        assert_eq!(5934, multiply(parse(EXAMPLE), 80));
    }
    #[test]
    fn part2_example() {
        let timers = parse(EXAMPLE);
        assert_eq!(26_984_457_539, multiply(timers, 256));
    }

    #[test]
    fn part1() {
        assert_eq!(day06_part1(), 345_387);
    }

    #[test]
    fn part2() {
        assert_eq!(day06_part2(), 1_574_445_493_136);
    }
}
