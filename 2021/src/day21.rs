use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn day21_part1() -> usize {
    InitialPositions::from(INPUT).roll_count_multiplied_with_score_of_losing_player()
}

pub(crate) fn day21_part2() -> usize {
    InitialPositions::from(INPUT).max_winning_universe_count()
}

impl InitialPositions {
    fn roll_count_multiplied_with_score_of_losing_player(&self) -> usize {
        let mut roll_count = 0;
        let mut player = 0;
        let mut positions = self.initial_positions;
        let mut scores = [0; 2];

        for roll_1 in (1..=100).into_iter().cycle().step_by(3) {
            // roll = roll_1 + roll_2 + roll_3 = roll_1 + (roll_1 + 1) + (roll_1 + 2) = 3 * roll_1 + 3
            let roll = (roll_1 + 1) * 3;
            roll_count += 3;
            positions[player] = next_position(positions[player], roll);
            let score = positions[player];

            scores[player] += score;
            if scores[player] >= 1000 {
                return roll_count * scores[(player + 1) % 2];
            }
            player = (player + 1) % 2;
        }
        unreachable!()
    }
    fn max_winning_universe_count(&self) -> usize {
        let mut curr_states = HashMap::from([(
            (
                [self.initial_positions[0], self.initial_positions[1]],
                [0; 2], // score by player 0 or 1
            ),
            1, // universe_count
        )]);
        let mut win_count = [0; 2]; // by player 0 or 1
        let mut player = 0;
        while !curr_states.is_empty() {
            // println!("State count: {}", states.len());
            let mut next_states = HashMap::new();
            for ((positions, scores), universe_count) in curr_states.drain() {
                for (roll, multiplier) in QUANTUM_ROLLS {
                    let position = next_position(positions[player], roll);
                    let score = scores[player] + position;
                    if score >= 21 {
                        win_count[player] += universe_count * multiplier;
                    } else {
                        let (mut positions, mut scores) = (positions, scores);
                        positions[player] = position;
                        scores[player] = score;
                        *next_states.entry((positions, scores)).or_default() +=
                            universe_count * multiplier;
                    }
                }
            }
            curr_states = next_states;
            player = (player + 1) % 2;
        }
        win_count.into_iter().max().unwrap()
    }
}
#[derive(Debug, PartialEq)]
struct InitialPositions {
    initial_positions: [usize; 2],
}
impl From<&str> for InitialPositions {
    fn from(input: &str) -> Self {
        let mut initial_position = [0; 2];
        for (i, line) in input.trim().lines().enumerate() {
            initial_position[i] = line.chars().last().unwrap().to_digit(10).unwrap() as usize;
        }
        InitialPositions {
            initial_positions: initial_position,
        }
    }
}

// The quantum dice roll 1, 2 and 3 during each of the 3 rolls in a turn. This produces 27 different
// outputs. The sum of the rolls can range from 3 to 9, where middle values are more frequent:
// - 3 can happen with 1 roll: 1+1+1
// - 4 can happen with 3 rolls: 1+1+2, 1+2+1, 2+1+1
// - 5 can happen with 6 rolls: 1+1+3, 1+2+2, 1+3+1, 2+1+2, 2+2+1, 3+1+1
// - 6 can happen with 7 rolls
// - 7 can happen with 6 rolls
// - 8 can happen with 3 rolls
// - 9 can happen with 1 roll: 3+3+3
const QUANTUM_ROLLS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type Position = usize;
type Roll = usize;
fn next_position(position: Position, roll: Roll) -> Position {
    (position + roll - 1) % 10 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_initial_positions_from() {
        assert_eq!(
            InitialPositions::from(EXAMPLE),
            InitialPositions {
                initial_positions: [4, 8]
            },
        );
        assert_eq!(
            InitialPositions::from(INPUT),
            InitialPositions {
                initial_positions: [2, 5]
            },
        );
    }

    #[test]
    fn test_next_position() {
        assert_eq!(next_position(1, 2), 3);
        assert_eq!(next_position(1, 10), 1);
        assert_eq!(next_position(1, 11), 2);
        assert_eq!(next_position(9, 1), 10);
        assert_eq!(next_position(9, 2), 1);
        assert_eq!(next_position(10, 1), 1);
        assert_eq!(next_position(10, 2), 2);
        assert_eq!(next_position(10, 10), 10);
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            745 * 993,
            InitialPositions::from(EXAMPLE).roll_count_multiplied_with_score_of_losing_player()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(576_600, day21_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            444_356_092_776_315,
            InitialPositions::from(EXAMPLE).max_winning_universe_count()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(131_888_061_854_776, day21_part2());
    }
}
