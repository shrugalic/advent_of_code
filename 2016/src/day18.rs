use line_reader::read_file_to_lines;

pub(crate) fn day18_part1() -> usize {
    let input = read_file_to_lines("input/day18.txt");
    safe_tile_count_of_generated_grid(&input[0], 40)
}

pub(crate) fn day18_part2() -> usize {
    let input = read_file_to_lines("input/day18.txt");
    safe_tile_count_of_generated_grid(&input[0], 40_0000)
}

fn safe_tile_count_of_generated_grid(input: &str, line_count: usize) -> usize {
    let mut line = parse_input(input);
    let mut count = count_safe_tiles(&line);

    for _ in 1..line_count {
        line = calc_next_line(line);
        count += count_safe_tiles(&line);
    }

    count
}

fn parse_input(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '^').collect()
}

fn extend_with_a_safe_tile_on_each_side(line: &mut Vec<bool>) {
    line.insert(0, false);
    line.push(false);
}

fn count_safe_tiles(line: &[bool]) -> usize {
    line.iter().filter(|&&is_trap| !is_trap).count()
}

fn calc_next_line(mut line: Vec<bool>) -> Vec<bool> {
    extend_with_a_safe_tile_on_each_side(&mut line);
    // The center tile[1] is a trap if the previous left (tile[0]) and right (tile[2) were different
    line.windows(3).map(|tile| tile[0] ^ tile[2]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(vec![false, false, true, true, false], parse_input("..^^."));
    }

    #[test]
    fn test_extend_with_a_safe_tile_on_each_side() {
        let mut line = vec![false, false, true, true, false];
        extend_with_a_safe_tile_on_each_side(&mut line);
        assert_eq!(vec![false, false, false, true, true, false, false], line);
    }

    #[test]
    fn test_count_safe_tiles() {
        assert_eq!(3, count_safe_tiles(&[false, false, true, true, false]));
    }

    #[test]
    fn test_calc_next_line() {
        assert_eq!(
            vec![false, true, true, true, true],
            calc_next_line(vec![false, false, true, true, false])
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(38, safe_tile_count_of_generated_grid(".^^.^.^^^^", 10));
    }

    #[test]
    fn part1() {
        assert_eq!(2016, day18_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(19_998_750, day18_part2());
    }
}
