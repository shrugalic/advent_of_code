const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn day01_part1() -> usize {
    let depths = parse(INPUT);
    part1(&depths)
}

pub(crate) fn day01_part2() -> usize {
    let depths = parse(INPUT);
    part2(&depths)
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn part1(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part2(depths: &[usize]) -> usize {
    depths.windows(4).filter(|w| w[0] < w[3]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_count_increasing_depths() {
        let depths = parse(EXAMPLE);
        assert_eq!(7, part1(&depths));
    }

    #[test]
    fn test_count_increasing_3_depth_averages() {
        let depths = parse(EXAMPLE);
        assert_eq!(5, part2(&depths));
    }

    #[test]
    fn test_part1() {
        assert_eq!(day01_part1(), 1475);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day01_part2(), 1516);
    }
}
