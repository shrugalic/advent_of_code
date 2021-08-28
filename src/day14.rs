use crate::day10::day10_part2_hash;

const PUZZLE_DAY14_INPUT: &str = "ffayrhll";
const GRID_SIZE: usize = 128;

pub(crate) fn day14_part1() -> usize {
    count_used_cells(PUZZLE_DAY14_INPUT)
}

pub(crate) fn day14_part2() -> usize {
    count_regions_of_used_cells(PUZZLE_DAY14_INPUT)
}

fn count_used_cells(input: &str) -> usize {
    let grid = generate_grid(input);
    grid.iter()
        .map(|row| row.iter().filter(|&s| s == &State::Used).count())
        .sum()
}

fn count_regions_of_used_cells(input: &str) -> usize {
    let mut grid = generate_grid(input);
    let mut locations_to_visit = locations_to_visit(&mut grid);

    let mut unique_region_count = 0;
    while let Some((x, y)) = locations_to_visit.pop() {
        if grid[y][x] == State::Labeled {
            continue;
        }
        unique_region_count += 1;
        let mut this_region = vec![(x, y)];
        while let Some((x, y)) = this_region.pop() {
            grid[y][x] = State::Labeled;
            safe_adjacent_neighbors(x, y)
                .into_iter()
                .filter(|(x, y)| grid[*y][*x] == State::Used)
                .for_each(|(x, y)| this_region.push((x, y)));
        }
    }
    unique_region_count
}

fn locations_to_visit(grid: &mut Vec<Vec<State>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, s)| s == &&State::Used)
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn safe_adjacent_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x + 1 < GRID_SIZE {
        neighbors.push((x + 1, y));
    }
    if y + 1 < GRID_SIZE {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn generate_grid(key: &str) -> Grid {
    (0..GRID_SIZE)
        .into_iter()
        .map(|row_num| {
            let input = format!("{}-{}", key, row_num);
            let hash = day10_part2_hash(&input);
            hash_to_grid_row(&hash)
        })
        .collect()
}

fn hash_to_grid_row(hash: &str) -> Vec<State> {
    hash.chars()
        .map(hex_char_to_bits)
        .flat_map(bits_to_states)
        .collect()
}

fn hex_char_to_bits(hex: char) -> String {
    format!("{:04b}", u8::from_str_radix(&hex.to_string(), 16).unwrap())
}

fn bits_to_states(four_bits: String) -> Vec<State> {
    four_bits.chars().map(State::from).collect::<Vec<_>>()
}

type Grid = Vec<Vec<State>>;

#[derive(Debug, PartialEq)]
enum State {
    Free,
    Used,
    Labeled, // This state is only for part 2
}
impl From<char> for State {
    fn from(ch: char) -> Self {
        match ch {
            '0' => State::Free,
            '1' => State::Used,
            _ => panic!("Unsupported char {}", ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "flqrgnkx";

    #[test]
    fn part1_example() {
        assert_eq!(8108, count_used_cells(EXAMPLE1));
    }

    #[test]
    fn part1_full() {
        assert_eq!(8190, day14_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(1242, count_regions_of_used_cells(EXAMPLE1));
    }

    #[test]
    fn part2_full() {
        assert_eq!(1134, day14_part2());
    }
}
