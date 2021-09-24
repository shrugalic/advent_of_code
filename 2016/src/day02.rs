use line_reader::read_file_to_lines;
use NumPad::*;

pub(crate) fn day02_part1() -> String {
    bathroom_code(read_file_to_lines("input/day02.txt"), NumPadType::Simple)
}

pub(crate) fn day02_part2() -> String {
    bathroom_code(read_file_to_lines("input/day02.txt"), NumPadType::Complex)
}

enum NumPadType {
    Simple,
    Complex,
}
fn bathroom_code(input: Vec<String>, numpad_type: NumPadType) -> String {
    let dirs: Vec<Vec<_>> = input
        .iter()
        .map(|line| line.chars().map(Dir::from).collect())
        .collect();
    let mut numpad = NumPad::default();
    let mut code: Vec<char> = vec![];
    for pattern in dirs {
        match numpad_type {
            NumPadType::Simple => numpad.apply_simple_pattern(pattern),
            NumPadType::Complex => numpad.apply_complex_pattern(pattern),
        }
        code.push(numpad.to_char());
    }

    code.iter().collect()
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Unknown dir '{}'", c),
        }
    }
}
enum NumPad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}
impl Default for NumPad {
    fn default() -> Self {
        Five
    }
}
impl NumPad {
    fn apply_simple_pattern(&mut self, pattern: Vec<Dir>) {
        for dir in pattern {
            match dir {
                Dir::Up => match self {
                    One | Two | Three => {}
                    Four => *self = One,
                    Five => *self = Two,
                    Six => *self = Three,
                    Seven => *self = Four,
                    Eight => *self = Five,
                    Nine => *self = Six,
                    _ => unreachable!(),
                },
                Dir::Down => match self {
                    One => *self = Four,
                    Two => *self = Five,
                    Three => *self = Six,
                    Four => *self = Seven,
                    Five => *self = Eight,
                    Six => *self = Nine,
                    Seven | Eight | Nine => {}
                    _ => unreachable!(),
                },
                Dir::Left => match self {
                    One | Four | Seven => {}
                    Two => *self = One,
                    Five => *self = Four,
                    Eight => *self = Seven,
                    Three => *self = Two,
                    Six => *self = Five,
                    Nine => *self = Eight,
                    _ => unreachable!(),
                },
                Dir::Right => match self {
                    One => *self = Two,
                    Four => *self = Five,
                    Seven => *self = Eight,
                    Two => *self = Three,
                    Five => *self = Six,
                    Eight => *self = Nine,
                    Three | Six | Nine => {}
                    _ => unreachable!(),
                },
            }
        }
    }
    fn apply_complex_pattern(&mut self, pattern: Vec<Dir>) {
        for dir in pattern {
            match dir {
                Dir::Up => match self {
                    One | Two | Four | Five | Nine => {}
                    Three => *self = One,
                    Six => *self = Two,
                    Seven => *self = Three,
                    Eight => *self = Four,
                    A => *self = Six,
                    B => *self = Seven,
                    C => *self = Eight,
                    D => *self = B,
                },
                Dir::Down => match self {
                    One => *self = Three,
                    Two => *self = Six,
                    Three => *self = Seven,
                    Four => *self = Eight,
                    Six => *self = A,
                    Seven => *self = B,
                    Eight => *self = C,
                    B => *self = D,
                    Five | Nine | A | C | D => {}
                },
                Dir::Left => match self {
                    One | Two | Five | A | D => {}
                    Three => *self = Two,
                    Seven => *self = Six,
                    B => *self = A,
                    Four => *self = Three,
                    Eight => *self = Seven,
                    C => *self = B,
                    Six => *self = Five,
                    Nine => *self = Eight,
                },
                Dir::Right => match self {
                    Two => *self = Three,
                    Six => *self = Seven,
                    A => *self = B,
                    Three => *self = Four,
                    Seven => *self = Eight,
                    B => *self = C,
                    Five => *self = Six,
                    Eight => *self = Nine,
                    One | Four | Nine | C | D => {}
                },
            }
        }
    }
    fn to_char(&self) -> char {
        match self {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn part1_example() {
        assert_eq!(
            "1985",
            bathroom_code(read_str_to_lines(EXAMPLE), NumPadType::Simple)
        );
    }

    #[test]
    fn part1() {
        assert_eq!("99332", day02_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            "5DB3",
            bathroom_code(read_str_to_lines(EXAMPLE), NumPadType::Complex)
        );
    }

    #[test]
    fn part2() {
        assert_eq!("DD483", day02_part2());
    }
}
