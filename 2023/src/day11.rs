use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day11.txt");

pub(crate) fn part1() -> usize {
    sum_of_distances_between_galaxies_in_universe_expanded_by_two(INPUT)
}

pub(crate) fn part2() -> usize {
    sum_of_distances_between_galaxies_in_universe_expanded_by_a_million(INPUT)
}

fn sum_of_distances_between_galaxies_in_universe_expanded_by_two(input: &str) -> usize {
    let mut universe = Universe::from(input);
    universe.sum_of_distances_between_galaxies_in_expanded_universe(2)
}

fn sum_of_distances_between_galaxies_in_universe_expanded_by_a_million(input: &str) -> usize {
    let mut universe = Universe::from(input);
    universe.sum_of_distances_between_galaxies_in_expanded_universe(1_000_000)
}

type Coord = i32;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    x: Coord,
    y: Coord,
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Pos>,
}

impl Universe {
    fn sum_of_distances_between_galaxies_in_expanded_universe(
        &mut self,
        expansion_factor: Coord,
    ) -> usize {
        // print!("Original:\n{self}\n\n");
        self.expand_empty_space_by_factor_of(expansion_factor);
        // print!("Expanded:\n{self}\n\n");
        self.sum_of_shortest_distances_between_galaxy_pairs()
    }
    fn expand_empty_space_by_factor_of(&mut self, expansion_factor: Coord) {
        let empty_positions = |filled_positions: BTreeSet<Coord>| -> BTreeSet<Coord> {
            let min = *filled_positions.first().unwrap();
            let max = *filled_positions.last().unwrap();
            (min + 1..max)
                .into_iter()
                .filter(|x| !filled_positions.contains(x))
                .collect()
        };
        let empty_cols = empty_positions(self.galaxies.iter().map(|pos| pos.x).collect());
        let empty_rows = empty_positions(self.galaxies.iter().map(|pos| pos.y).collect());

        let count_preceding_empties = |empties: &BTreeSet<Coord>, value: &Coord| {
            empties.iter().filter(|&empty| empty < value).count() as Coord
        };

        let additional_row_or_col_count = expansion_factor - 1;
        for pos in self.galaxies.iter_mut() {
            pos.x += additional_row_or_col_count * count_preceding_empties(&empty_cols, &pos.x);
            pos.y += additional_row_or_col_count * count_preceding_empties(&empty_rows, &pos.y);
        }
    }
    fn sum_of_shortest_distances_between_galaxy_pairs(&self) -> usize {
        let mut distance_sum = 0;
        for i in 0..self.galaxies.len() {
            let current = self.galaxies.get(i).unwrap();
            for j in i + 1..self.galaxies.len() {
                if i == j {
                    continue;
                }
                let other = self.galaxies.get(j).unwrap();
                distance_sum += current.hamming_distance_to(other);
            }
        }
        distance_sum
    }
}

impl Pos {
    fn new(x: Coord, y: Coord) -> Self {
        Pos { x, y }
    }
    fn hamming_distance_to(&self, other: &Pos) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs()) as usize
    }
}

impl From<&str> for Universe {
    fn from(input: &str) -> Self {
        let mut galaxies: Vec<_> = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| (c == '#').then_some(Pos::new(x as Coord, y as Coord)))
                    .collect::<Vec<_>>()
            })
            .collect();
        galaxies.sort_unstable();
        Universe { galaxies }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_min = self.galaxies.first().unwrap().x;
        let x_max = self.galaxies.last().unwrap().x;
        let xs = x_min..=x_max;

        let y_min = self.galaxies.iter().min_by_key(|pos| pos.y).unwrap().y;
        let y_max = self.galaxies.iter().max_by_key(|pos| pos.y).unwrap().y;
        let ys = y_min..=y_max;

        write!(
            f,
            "{}",
            ys.into_iter()
                .map(|y| {
                    xs.clone()
                        .into_iter()
                        .map(|x| {
                            if self.galaxies.contains(&Pos::new(x, y)) {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1_example() {
        assert_eq!(
            374,
            sum_of_distances_between_galaxies_in_universe_expanded_by_two(EXAMPLE)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            9_647_174,
            sum_of_distances_between_galaxies_in_universe_expanded_by_two(INPUT)
        );
    }

    #[test]
    fn test_part2_example_with_expansion_factor_10() {
        let mut universe = Universe::from(EXAMPLE);
        assert_eq!(
            1030,
            universe.sum_of_distances_between_galaxies_in_expanded_universe(10)
        );
    }
    #[test]
    fn test_part2_example_with_expansion_factor_100() {
        let mut universe = Universe::from(EXAMPLE);
        assert_eq!(
            8410,
            universe.sum_of_distances_between_galaxies_in_expanded_universe(100)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            377_318_892_554,
            sum_of_distances_between_galaxies_in_universe_expanded_by_a_million(INPUT)
        );
    }
}
