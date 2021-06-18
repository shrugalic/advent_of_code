struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Slope { right, down }
    }
}

// Traverse map with given slope and return number of trees encountered
fn traverse_map(map: &Vec<String>, slope: Slope) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut tree_count = 0usize;
    let (mut x, mut y) = (0usize, 0usize);
    while y < height {
        match map[y].chars().nth(x).unwrap() {
            '.' => {}
            '#' => tree_count += 1,
            _ => unreachable!(),
        }
        x = (x + slope.right) % width;
        y += slope.down;
    }
    tree_count
}

#[cfg(test)]
mod tests {
    use crate::{traverse_map, Slope};
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_MAP: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part1_example() {
        let map = read_str_to_lines(EXAMPLE_MAP);
        assert_eq!(traverse_map(&map, Slope::new(3, 1)), 7);
    }

    #[test]
    fn part1() {
        let map = read_file_to_lines("input.txt");
        assert_eq!(traverse_map(&map, Slope::new(3, 1)), 244);
    }
}
