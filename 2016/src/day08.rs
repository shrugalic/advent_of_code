use line_reader::read_file_to_lines;

pub(crate) fn day08_part1() -> usize {
    let screen = apply_all_input_operations_to_screen();
    screen.count_on_pixels()
}

pub(crate) fn day08_part2() -> String {
    let screen = apply_all_input_operations_to_screen();
    if screen.to_string()
        == "\
####...##.#..#.###..#..#..##..###..#....#...#..##.
...#....#.#..#.#..#.#.#..#..#.#..#.#....#...#...#.
..#.....#.####.#..#.##...#....#..#.#.....#.#....#.
.#......#.#..#.###..#.#..#....###..#......#.....#.
#....#..#.#..#.#.#..#.#..#..#.#....#......#..#..#.
####..##..#..#.#..#.#..#..##..#....####...#...##.."
    {
        "ZJHRKCPLYJ"
    } else {
        "fail"
    }
    .to_string()
}

fn apply_all_input_operations_to_screen() -> Screen {
    let mut screen = Screen::new(50, 6);
    for op in parse_operations() {
        screen.apply(op);
    }
    screen
}

fn parse_operations() -> Vec<Op> {
    read_file_to_lines("input/day08.txt")
        .into_iter()
        .map(Op::from)
        .collect()
}

#[derive(PartialEq, Debug)]
enum Op {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}
impl<T: AsRef<str>> From<T> for Op {
    fn from(s: T) -> Self {
        let parts: Vec<_> = s
            .as_ref()
            .split(|c| c == ' ' || c == 'x' || c == '=')
            .collect();
        // println!("parts = {:?}", parts);
        if parts[0] == "rect" {
            Op::Rect(parts[1].parse().unwrap(), parts[2].parse().unwrap())
        } else if parts[1] == "column" {
            Op::RotateColumn(parts[4].parse().unwrap(), parts[6].parse().unwrap())
        } else if parts[1] == "row" {
            Op::RotateRow(parts[3].parse().unwrap(), parts[5].parse().unwrap())
        } else {
            panic!("Invalid input '{}'", s.as_ref())
        }
    }
}

struct Screen {
    grid: Vec<Vec<bool>>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            grid: vec![vec![false; width]; height],
        }
    }
    fn apply(&mut self, op: Op) {
        match op {
            Op::Rect(w, h) => {
                self.grid
                    .iter_mut()
                    .take(h)
                    .for_each(|row| row.iter_mut().take(w).for_each(|pixel| *pixel = true));
            }
            Op::RotateRow(y, o) => {
                self.grid[y].rotate_right(o);
            }
            Op::RotateColumn(x, o) => {
                let mut col: Vec<_> = self.grid.iter().map(|row| row[x]).collect();
                col.rotate_right(o);
                for (y, row) in self.grid.iter_mut().enumerate() {
                    row[x] = col[y];
                }
            }
        }
    }
    fn count_on_pixels(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&on| on).count())
            .sum()
    }
}
impl ToString for Screen {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&on| if on { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ops() {
        assert_eq!(Op::Rect(1, 2), Op::from("rect 1x2"));
        assert_eq!(Op::RotateRow(1, 2), Op::from("rotate row y=1 by 2"));
        assert_eq!(Op::RotateColumn(1, 2), Op::from("rotate column x=1 by 2"));
    }

    #[test]
    fn screen_to_string() {
        let screen = Screen::new(7, 3);
        assert_eq!(
            "\
.......
.......
.......",
            screen.to_string()
        );
    }

    #[test]
    fn apply_operations_to_screen() {
        let mut screen = Screen::new(7, 3);
        screen.apply(Op::from("rect 3x2"));
        assert_eq!(
            "\
###....
###....
.......",
            screen.to_string()
        );

        screen.apply(Op::from("rotate column x=1 by 1"));
        assert_eq!(
            "\
#.#....
###....
.#.....",
            screen.to_string()
        );

        screen.apply(Op::from("rotate row y=0 by 4"));
        assert_eq!(
            "\
....#.#
###....
.#.....",
            screen.to_string()
        );

        screen.apply(Op::from("rotate column x=1 by 1"));
        assert_eq!(
            "\
.#..#.#
#.#....
.#.....",
            screen.to_string()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(110, day08_part1());
    }

    #[test]
    fn part2() {
        assert_eq!("ZJHRKCPLYJ", day08_part2());
    }
}
