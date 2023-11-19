use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::ops::RangeInclusive;
use Direction::*;

const INPUT: &str = include_str!("../input/day24.txt");

pub(crate) fn day24_part1() -> usize {
    let field = Field::from(INPUT);
    field.shortest_time_to_exit()
}

pub(crate) fn day24_part2() -> usize {
    let field = Field::from(INPUT);
    field.shortest_time_to_exit_and_back_to_entrance_and_back_to_exit()
}

impl Field {
    fn shortest_time_to_exit(&self) -> usize {
        let below_entrance = (1, 1);
        let above_exit = (self.width - 2, self.height - 2);
        self.shortest_time(State::new(below_entrance, 1), above_exit)
    }
    fn shortest_time_to_exit_and_back_to_entrance_and_back_to_exit(&self) -> usize {
        let below_entrance = (1, 1);
        let above_exit = (self.width - 2, self.height - 2);

        // From start to exit
        let time = self.shortest_time(State::new(below_entrance, 1), above_exit);

        // Back to entrance
        let time = self.shortest_time(State::new(above_exit, time + 1), below_entrance);

        // And back to exit again
        self.shortest_time(State::new(below_entrance, time + 1), above_exit)
    }
    fn shortest_time(&self, mut initial: State, exit: Pos) -> usize {
        let mut queue = BinaryHeap::new();
        let mut seen = HashSet::new();

        for _ in 0..self.state_count {
            if self.is_valid_state(&initial) {
                seen.insert(initial.clone());
                queue.push(Reverse(initial.clone()));
            }
            initial.time += 1;
        }

        while let Some(Reverse(State { time, pos })) = queue.pop() {
            let time = time + 1;
            if pos == exit {
                return time;
            }

            // Add next positions to candidates
            for pos in self.next_positions(&pos) {
                let next = State { time, pos };
                if self.is_valid_state(&next) && !seen.contains(&next) {
                    seen.insert(next.clone());
                    queue.push(Reverse(next));
                }
            }
        }
        unreachable!()
    }
    fn is_valid_state(&self, state: &State) -> bool {
        self.valid_states
            .contains(&state.reduced_by(self.state_count))
    }
    fn next_positions(&self, (x, y): &Pos) -> Vec<Pos> {
        [(-1, 0), (1, 0), (0, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(dx, dy): (isize, isize)| {
                ((dx + *x as isize) as usize, (dy + *y as isize) as usize)
            })
            .filter(|(x, y)| self.x_range().contains(x) && self.y_range().contains(y))
            .collect()
    }
    fn calculate_all_states(&mut self) {
        assert_eq!(self.blizzards_by_time.len(), 1);

        for time in 1..self.state_count {
            // Calculate next blizzards
            let mut blizzards = Blizzards::new(self.width, self.height);
            for x in 1..=self.width - 2 {
                for y in 1..=self.height - 2 {
                    let pos = (x, y);
                    for dir in &self.blizzards_by_time[time - 1].blizzards[y][x] {
                        let next_pos = self.next_pos(&pos, dir);
                        blizzards.insert(next_pos, *dir);
                    }
                }
            }
            self.blizzards_by_time.push(blizzards);

            // Initialize valid states
            for x in 1..=self.width - 2 {
                for y in 1..=self.height - 2 {
                    if self.blizzards_by_time[time].blizzards[y][x].is_empty() {
                        self.valid_states.insert(State { pos: (x, y), time });
                    }
                }
            }
        }
    }
    fn x_range(&self) -> RangeInclusive<Coord> {
        1..=self.width - 2
    }
    fn y_range(&self) -> RangeInclusive<Coord> {
        1..=self.height - 2
    }
    fn next_pos(&self, pos: &Pos, dir: &Direction) -> Pos {
        let xs = self.x_range();
        let ys = self.y_range();
        let (x, y) = *pos;
        match dir {
            Left => (if &x == xs.start() { *xs.end() } else { x - 1 }, y),
            Right => (if &x == xs.end() { *xs.start() } else { x + 1 }, y),
            Up => (x, if &y == ys.start() { *ys.end() } else { y - 1 }),
            Down => (x, if &y == ys.end() { *ys.start() } else { y + 1 }),
        }
    }
    fn least_common_multiple(w: usize, h: usize) -> usize {
        let mut lcm = usize::max(w, h);
        while !(lcm % w == 0 && lcm % h == 0) {
            lcm += 1;
        }
        lcm
    }
    #[allow(unused)] // for testing/debugging
    fn string_at_time(&self, time: Minutes) -> String {
        let time = time % self.state_count;
        (0..self.height)
            .into_iter()
            .map(|y| {
                (0..self.width)
                    .into_iter()
                    .map(|x| {
                        let dirs = &self.blizzards_by_time[time].blizzards[y][x];
                        match dirs.len() {
                            0 => {
                                let is_entrance = x == 1 && y == 0;
                                let is_exit = x == self.width - 2 && y == self.height - 1;
                                let is_border =
                                    x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1;
                                if is_entrance || is_exit {
                                    '.'
                                } else if is_border {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            1 => dirs[0].to_char(),
                            len => ((len as u8) + b'0') as char,
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

type Coord = usize;
type Minutes = usize;
type Pos = (Coord, Coord);

#[derive(Debug)]
struct Field {
    width: usize,
    height: usize,
    // Blizzards indexed by time, starting at 0, and ending at state_count - 1
    blizzards_by_time: Vec<Blizzards>,
    // The blizzards repeat after a while, this is the number of distinct states
    state_count: usize,
    valid_states: HashSet<State>,
}
impl From<&str> for Field {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input.trim().lines().collect();
        let height = lines.len();
        let width = lines[0].chars().count();

        let mut blizzards = Blizzards::new(width, height);
        for (pos, dir) in lines.into_iter().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| Direction::from(c).map(|dir| ((x, y), dir)))
        }) {
            blizzards.insert(pos, dir);
        }

        let mut valid_states = HashSet::new();
        for x in 1..=width - 2 {
            for y in 1..=height - 2 {
                if blizzards.blizzards[y][x].is_empty() {
                    valid_states.insert(State::new((x, y), 0));
                }
            }
        }
        let mut field = Field {
            width,
            height,
            state_count: Field::least_common_multiple(width - 2, height - 2),
            blizzards_by_time: vec![blizzards],
            valid_states,
        };
        field.calculate_all_states();
        field
    }
}

#[derive(Debug)]
struct Blizzards {
    blizzards: Vec<Vec<Vec<Direction>>>,
}
impl Blizzards {
    fn new(width: usize, height: usize) -> Self {
        Blizzards {
            blizzards: vec![vec![vec![]; width]; height],
        }
    }
    fn insert(&mut self, (x, y): Pos, dir: Direction) {
        self.blizzards[y][x].push(dir);
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn from(c: char) -> Option<Direction> {
        match c {
            '<' => Some(Left),
            '>' => Some(Right),
            '^' => Some(Up),
            'v' => Some(Down),
            '#' | '.' => None,
            _ => unreachable!("Invalid char {c}"),
        }
    }
    fn to_char(self) -> char {
        match self {
            Left => '<',
            Right => '>',
            Up => '^',
            Down => 'v',
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct State {
    time: Minutes,
    pos: Pos,
}
impl State {
    fn new(pos: Pos, time: Minutes) -> Self {
        State { pos, time }
    }
    fn reduced_by(&self, state_count: usize) -> Self {
        State {
            time: self.time % state_count,
            pos: self.pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_EXAMPLE: [&str; 5] = [
        "\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#",
        "\
#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#",
        "\
#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#",
        "\
#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#",
        "\
#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#",
    ];

    const EXAMPLE: [&str; 12] = [
        "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
        "\
#.######
#.>3.<.#
#<..<<.#
#>2.22.#
#>v..^<#
######.#",
        "\
#.######
#.2>2..#
#.^22^<#
#.>2.^>#
#.>..<.#
######.#",
        "\
#.######
#<^<22.#
#.2<.2.#
#><2>..#
#..><..#
######.#",
        "\
#.######
#.<..22#
#<<.<..#
#<2.>>.#
#.^22^.#
######.#",
        "\
#.######
#2.v.<>#
#<.<..<#
#.^>^22#
#.2..2.#
######.#",
        "\
#.######
#>2.<.<#
#.2v^2<#
#>..>2>#
#<....>#
######.#",
        "\
#.######
#.22^2.#
#<v.<2.#
#>>v<>.#
#>....<#
######.#",
        "\
#.######
#.<>2^.#
#..<<.<#
#.22..>#
#.2v^2.#
######.#",
        "\
#.######
#<.2>>.#
#.<<.<.#
#>2>2^.#
#.v><^.#
######.#",
        "\
#.######
#.2..>2#
#<2v2^.#
#<>.>2.#
#..<>..#
######.#",
        "\
#.######
#2^.^2>#
#<v<.^<#
#..2.>2#
#.<..>.#
######.#",
    ];

    #[test]
    fn part1_simple_example_states() {
        let fields = Field::from(SIMPLE_EXAMPLE[0]);
        for (time, example) in SIMPLE_EXAMPLE.into_iter().enumerate() {
            let result = fields.string_at_time(time);
            if example != result {
                println!("time = {time}");
            }
            assert_eq!(example, result);
        }
    }

    #[test]
    fn part1_example_states() {
        let fields = Field::from(EXAMPLE[0]);
        for (time, example) in EXAMPLE.into_iter().enumerate() {
            let result = fields.string_at_time(time);
            if example != result {
                println!("time = {time}");
            }
            assert_eq!(example, result);
        }
    }

    #[test]
    fn part1_example() {
        let fields = Field::from(EXAMPLE[0]);
        assert_eq!(18, fields.shortest_time_to_exit());
    }

    #[test]
    fn part1() {
        assert_eq!(308, day24_part1());
    }

    #[test]
    fn part2_example() {
        let fields = Field::from(EXAMPLE[0]);
        assert_eq!(
            18 + 23 + 13,
            fields.shortest_time_to_exit_and_back_to_entrance_and_back_to_exit()
        );
    }

    #[test]
    fn part2_example_from_end_back_to_start() {
        let fields = Field::from(EXAMPLE[0]);
        let time_to_exit = 18;
        let start_above_exit = (fields.width - 2, fields.height - 2);
        let start_time = time_to_exit + 1;
        let end_below_start = (1, 1);
        assert_eq!(
            time_to_exit + 23,
            fields.shortest_time(State::new(start_above_exit, start_time), end_below_start)
        );
    }

    #[test]
    fn part2_example_from_start_to_end_but_later() {
        let fields = Field::from(EXAMPLE[0]);
        let time_back_to_start = 18 + 23;
        let start_below_start = (1, 1);
        let end_above_exit = (fields.width - 2, fields.height - 2);
        assert_eq!(
            time_back_to_start + 13,
            fields.shortest_time(
                State::new(start_below_start, time_back_to_start + 1),
                end_above_exit
            )
        );
    }

    #[test]
    fn part2() {
        assert_eq!(908, day24_part2());
    }
}
