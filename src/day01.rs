use line_reader::read_file_to_lines;

pub(crate) fn day01_part1() -> usize {
    let depths = parse(read_file_to_lines("input/day01.txt"));
    count_increasing_depths(&depths)
}

pub(crate) fn day01_part2() -> usize {
    let depths = parse(read_file_to_lines("input/day01.txt"));
    count_increasing_3_depth_averages(&depths)
}

fn parse(input: Vec<String>) -> Vec<usize> {
    input.into_iter().filter_map(|s| s.parse().ok()).collect()
}

fn count_increasing_depths(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_increasing_3_depth_averages(depths: &[usize]) -> usize {
    // w[0] + w[1] + w[2] < w[1] + w[2] + w[3] can be simplified to w[0] < w[3]
    depths.windows(4).filter(|w| w[0] < w[3]).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

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
        let depths = parse(read_str_to_lines(EXAMPLE));
        assert_eq!(7, count_increasing_depths(&depths));
    }
    #[test]
    fn test_count_increasing_3_depth_averages() {
        let depths = parse(read_str_to_lines(EXAMPLE));
        assert_eq!(5, count_increasing_3_depth_averages(&depths));
    }

    #[test]
    fn part1() {
        assert_eq!(day01_part1(), 1475);
    }

    #[test]
    fn part2() {
        assert_eq!(day01_part2(), 1516);
    }
}
