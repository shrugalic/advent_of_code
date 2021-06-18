struct Slope {
    right: usize,
    down: usize,
}

// Traverse map with given slope and return number of trees encountered
fn traverse_map(map: &Vec<String>, slope: &Slope) -> usize {
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
        assert_eq!(traverse_map(&map, &Slope { right: 3, down: 1 }), 7);
    }

    #[test]
    fn part1() {
        let map = read_file_to_lines("input.txt");
        assert_eq!(traverse_map(&map, &Slope { right: 3, down: 1 }), 244);
    }

    const PART2_SLOPES: [Slope; 5] = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    #[test]
    fn part2_example() {
        let map = read_str_to_lines(EXAMPLE_MAP);
        let product = PART2_SLOPES
            .iter()
            .map(|slope| traverse_map(&map, slope))
            .reduce(|a, b| a * b)
            .unwrap();

        assert_eq!(product, 336);
    }

    #[test]
    fn part2() {
        let map = read_file_to_lines("input.txt");
        let product = PART2_SLOPES
            .iter()
            .map(|slope| traverse_map(&map, slope))
            .reduce(|a, b| a * b)
            .unwrap();

        assert_eq!(product, 9406609920);
    }
}
