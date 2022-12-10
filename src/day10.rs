const INPUT: &str = include_str!("../input/day10.txt");

pub(crate) fn day10_part1() -> isize {
    let operations = parse(INPUT);
    let values = calculate_values(operations);
    calculate_signal_strength(values)
}

pub(crate) fn day10_part2() -> String {
    let operations = parse(INPUT);
    let values = calculate_values(operations);
    draw_image(values)
}

fn parse(input: &str) -> Vec<Op> {
    input.trim().lines().map(Op::from).collect()
}

/// Returns a vector of values _during_ operation `i`, where `i` is the index.
fn calculate_values(operations: Vec<Op>) -> Vec<isize> {
    let mut value = 1;
    let mut values = vec![value, value];
    for op in operations {
        match op {
            Op::Noop => values.push(value),
            Op::AddX(delta) => {
                values.push(value);
                value += delta;
                values.push(value);
            }
        }
    }
    values
}

fn calculate_signal_strength(values: Vec<isize>) -> isize {
    values
        .into_iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(cycle, value)| cycle as isize * value)
        .sum()
}

fn draw_image(values: Vec<isize>) -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut pixels: String = values
        .into_iter()
        .skip(1) // skip initial value before 1st cycle
        .take(HEIGHT * WIDTH)
        .enumerate()
        .map(|(cycle, sprite_pos)| {
            let pixel_pos = (cycle % WIDTH) as isize;
            // The sprite_pos is at the center of 3 pixels,
            // if the pixel_pos is within it, light it up
            let lit = (sprite_pos - pixel_pos).abs() <= 1;
            if lit {
                '#'
            } else {
                '.'
            }
        })
        .collect();

    // Wrap pixels at screen width to produce final image
    for y in (1..HEIGHT).into_iter().rev() {
        pixels.insert(y * WIDTH, '\n');
    }
    pixels
}

#[derive(PartialEq, Debug)]
enum Op {
    Noop,
    AddX(isize),
}
impl From<&str> for Op {
    fn from(command: &str) -> Self {
        match command {
            "noop" => Op::Noop,
            _ => Op::AddX(command[5..].parse().expect("a valid isize")),
        }
    }
}

pub(crate) const PART2_RESULT_IMAGE: &str = "\
####.#....###..#....####..##..####.#....
#....#....#..#.#.......#.#..#....#.#....
###..#....#..#.#......#..#......#..#....
#....#....###..#.....#...#.##..#...#....
#....#....#....#....#....#..#.#....#....
####.####.#....####.####..###.####.####.";

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_EXAMPLE: &str = "\
noop
addx 3
addx -5";

    #[test]
    fn part1_example_parse() {
        let operations = parse(SIMPLE_EXAMPLE);
        assert_eq!(vec![Op::Noop, Op::AddX(3), Op::AddX(-5)], operations);
    }

    #[test]
    fn part1_example_calculate_register_values() {
        let values = calculate_values(vec![Op::Noop, Op::AddX(3), Op::AddX(-5)]);
        assert_eq!(vec![1, 1, 1, 1, 4, 4, -1], values);
    }

    #[test]
    fn part1_example_calculate_signal_strength() {
        let operations = parse(EXAMPLE);
        let values = calculate_values(operations);
        assert_eq!(13_140, calculate_signal_strength(values));
    }

    #[test]
    fn part1() {
        assert_eq!(14_780, day10_part1());
    }

    const EXAMPLE_RESULT_IMAGE: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn part2_example() {
        let operations = parse(EXAMPLE);
        let values = calculate_values(operations);
        assert_eq!(EXAMPLE_RESULT_IMAGE, draw_image(values));
    }

    #[test]
    fn part2() {
        assert_eq!(PART2_RESULT_IMAGE, day10_part2());
    }

    const EXAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
