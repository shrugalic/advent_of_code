use std::collections::HashMap;

const DAY3_PART1_PUZZLE_INPUT: usize = 361527;

pub(crate) fn day3_part1() -> usize {
    manhattan_distance_to_origin_of_nth_spiral_point(DAY3_PART1_PUZZLE_INPUT)
}

pub(crate) fn day3_part2() -> usize {
    nth_spiral_points_value_up_to_limit(usize::MAX)
}

fn manhattan_distance_to_origin_of_nth_spiral_point(n: usize) -> usize {
    let mut spiral = Spiral::default();
    while spiral.total_steps_taken < n - 1 {
        spiral.do_step();
    }
    spiral.current_position().distance_to_origin()
}

fn nth_spiral_points_value_up_to_limit(n: usize) -> usize {
    let mut spiral = Spiral::default();
    let mut grid: HashMap<Position, usize> = HashMap::new();
    grid.insert(*spiral.current_position(), 1);
    while spiral.total_steps_taken < n - 1 {
        spiral.do_step();
        let value = spiral.neighbor_sum(&grid);
        if value > DAY3_PART1_PUZZLE_INPUT {
            return value;
        }
        grid.insert(*spiral.current_position(), value);
    }
    *grid.get(spiral.current_position()).unwrap()
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl Position {
    fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Down => self.y -= 1,
        }
    }
    fn distance_to_origin(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
    fn neighbors(&self) -> [Position; 8] {
        [
            self.offset(1, 0),
            self.offset(1, 1),
            self.offset(0, 1),
            self.offset(-1, 1),
            self.offset(-1, 0),
            self.offset(-1, -1),
            self.offset(0, -1),
            self.offset(1, -1),
        ]
    }
    fn offset(&self, x: isize, y: isize) -> Self {
        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        };
    }
}

struct Spiral {
    position: Position,
    direction: Direction,
    total_steps_taken: usize,
    steps_in_same_direction: usize,
    side_length: usize,
    times_turned: usize,
}

impl Default for Spiral {
    fn default() -> Self {
        Spiral {
            position: Position::default(),
            direction: Direction::Right,
            total_steps_taken: 0,
            steps_in_same_direction: 0,
            side_length: 1,
            times_turned: 0,
        }
    }
}

impl Spiral {
    fn do_step(&mut self) {
        self.step_forward();
        if self.reached_corner() {
            self.turn_left()
        }
    }
    fn step_forward(&mut self) {
        self.total_steps_taken += 1;
        self.steps_in_same_direction += 1;
        self.position.move_in_direction(&self.direction);
    }

    fn reached_corner(&self) -> bool {
        self.steps_in_same_direction == self.side_length
    }

    fn turn_left(&mut self) {
        self.steps_in_same_direction = 0;
        self.direction.turn_left();
        self.times_turned += 1;
        if self.times_turned % 2 == 0 {
            self.side_length += 1;
        }
    }

    fn current_position(&self) -> &Position {
        &self.position
    }

    fn neighbor_sum(&self, grid: &HashMap<Position, usize>) -> usize {
        self.current_position()
            .neighbors()
            .iter()
            .filter_map(|pos| grid.get(pos))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(0, manhattan_distance_to_origin_of_nth_spiral_point(1));
        assert_eq!(3, manhattan_distance_to_origin_of_nth_spiral_point(12));
        assert_eq!(2, manhattan_distance_to_origin_of_nth_spiral_point(23));
        assert_eq!(31, manhattan_distance_to_origin_of_nth_spiral_point(1024));
    }

    #[test]
    fn part1() {
        assert_eq!(326, day3_part1());
    }

    #[test]
    fn examples_part2() {
        assert_eq!(1, nth_spiral_points_value_up_to_limit(1));
        assert_eq!(1, nth_spiral_points_value_up_to_limit(2));
        assert_eq!(2, nth_spiral_points_value_up_to_limit(3));
        assert_eq!(4, nth_spiral_points_value_up_to_limit(4));
        assert_eq!(5, nth_spiral_points_value_up_to_limit(5));
        assert_eq!(10, nth_spiral_points_value_up_to_limit(6));
        assert_eq!(11, nth_spiral_points_value_up_to_limit(7));
        assert_eq!(23, nth_spiral_points_value_up_to_limit(8));
        assert_eq!(25, nth_spiral_points_value_up_to_limit(9));
        assert_eq!(26, nth_spiral_points_value_up_to_limit(10));
        assert_eq!(54, nth_spiral_points_value_up_to_limit(11));
        assert_eq!(57, nth_spiral_points_value_up_to_limit(12));
        assert_eq!(59, nth_spiral_points_value_up_to_limit(13));
        assert_eq!(122, nth_spiral_points_value_up_to_limit(14));
        assert_eq!(133, nth_spiral_points_value_up_to_limit(15));
        assert_eq!(142, nth_spiral_points_value_up_to_limit(16));
        assert_eq!(147, nth_spiral_points_value_up_to_limit(17));
        assert_eq!(304, nth_spiral_points_value_up_to_limit(18));
        assert_eq!(330, nth_spiral_points_value_up_to_limit(19));
        assert_eq!(351, nth_spiral_points_value_up_to_limit(20));
        assert_eq!(362, nth_spiral_points_value_up_to_limit(21));
        assert_eq!(747, nth_spiral_points_value_up_to_limit(22));
        assert_eq!(806, nth_spiral_points_value_up_to_limit(23));
    }

    #[test]
    fn part2() {
        assert_eq!(363010, day3_part2());
    }
}
