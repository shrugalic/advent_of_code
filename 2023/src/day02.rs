use std::cmp::max;

const INPUT: &str = include_str!("../input/day02.txt");

pub(crate) fn part1() -> usize {
    id_sum_of_games_possible_with_given_cube_count(INPUT)
}

pub(crate) fn part2() -> usize {
    power_sum_of_minimal_set_of_cubes(INPUT)
}

fn id_sum_of_games_possible_with_given_cube_count(input: &str) -> usize {
    let cube_count = CubeCount::new(12, 13, 14);
    parse_games(input)
        .into_iter()
        .filter(|game| game.is_possible_with(&cube_count))
        .map(|game| game.id)
        .sum()
}

fn power_sum_of_minimal_set_of_cubes(input: &str) -> usize {
    parse_games(input)
        .into_iter()
        .map(Game::minimal_cube_count)
        .map(CubeCount::power)
        .sum()
}

fn parse_games(input: &str) -> Vec<Game> {
    input.trim().lines().map(Game::from).collect()
}

#[derive(PartialEq, Debug)]
struct Game {
    id: usize,
    reveals: Vec<CubeCount>,
}

#[derive(PartialEq, Debug, Default)]
struct CubeCount {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (game, reveals) = line.split_once(": ").expect("Game ${id}: ${reveals}");
        let id = game.strip_prefix("Game ").expect("'Game ' prefix");
        Game {
            id: id.parse().expect("Valid number"),
            reveals: reveals.split("; ").map(CubeCount::from).collect(),
        }
    }
}

impl Game {
    fn is_possible_with(&self, total: &CubeCount) -> bool {
        self.reveals.iter().all(|reveal| reveal.is_within(total))
    }
    fn minimal_cube_count(self) -> CubeCount {
        self.reveals
            .iter()
            .fold(CubeCount::default(), |minimum, revealed| {
                minimum.containing(revealed)
            })
    }
}

impl CubeCount {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        CubeCount { red, green, blue }
    }
    fn is_within(&self, limit: &CubeCount) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
    fn containing(self, other: &CubeCount) -> CubeCount {
        CubeCount {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }
    fn power(self) -> usize {
        self.red * self.green * self.blue
    }
}

impl From<&str> for CubeCount {
    fn from(counts: &str) -> Self {
        let mut cube_count = CubeCount::default();
        for single in counts.split(", ") {
            let (count, color) = single.split_once(' ').expect("count followed by color");
            let count: usize = count.parse().expect("valid number");
            match color {
                "red" => cube_count.red += count,
                "green" => cube_count.green += count,
                "blue" => cube_count.blue += count,
                _ => panic!("Unexpected color '{color}'"),
            }
        }
        cube_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                reveals: vec![
                    CubeCount::new(4, 0, 3),
                    CubeCount::new(1, 2, 6),
                    CubeCount::new(0, 2, 0),
                ]
            }
        );
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(8, id_sum_of_games_possible_with_given_cube_count(EXAMPLE));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2_286, power_sum_of_minimal_set_of_cubes(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(2_207, id_sum_of_games_possible_with_given_cube_count(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(62_241, power_sum_of_minimal_set_of_cubes(INPUT));
    }
}
