use std::str::FromStr;

const INPUT: &str = include_str!("../../2025/input/day12.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let (shapes, regions) = parse(input);
    regions
        .iter()
        .filter(|region| region.can_fit_all_presents(&shapes))
        .count()
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut blocks: Vec<&str> = input.trim().split("\n\n").collect();
    let regions = blocks.remove(blocks.len() - 1);
    let regions = regions.lines().map(Region::from).collect();
    let shapes = blocks.into_iter().map(parse_shape).collect();
    (shapes, regions)
}

fn parse_shape(block: &str) -> Shape {
    let v: Vec<bool> = block
        .chars()
        .filter_map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .collect();
    [v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]]
}

impl From<&str> for Region {
    fn from(line: &str) -> Self {
        let (dimensions, shape_quantities) = line.split_once(": ").unwrap();
        let dimensions: Vec<usize> = dimensions
            .split('x')
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        let shape_quantities: Vec<usize> = shape_quantities
            .split_whitespace()
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        Region {
            width: dimensions[0],
            height: dimensions[1],
            shape_quantities,
        }
    }
}

// 012
// 345
// 678
type Shape = [bool; 9];

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shape_quantities: Vec<usize>,
}

impl Region {
    fn can_fit_all_presents(&self, shapes: &[Shape]) -> bool {
        // Trivially impossible
        let available_spaces = self.width * self.height;
        let needed_spaces = self
            .shape_quantities
            .iter()
            .enumerate()
            .map(|(idx, quantity)| quantity * shapes[idx].iter().filter(|&b| *b).count())
            .sum::<usize>();
        if available_spaces < needed_spaces {
            return false;
        }

        // Trivially possible
        let available_3x3_blocks = self.width / 3 * self.height / 3;
        let needed_3x3_blocks = self.shape_quantities.iter().sum::<usize>();
        if needed_3x3_blocks <= available_3x3_blocks {
            return true;
        }

        // The example could be solved by packing two shape 4 into a 4x4 square,
        // combining shape 0 and 2 into a 3x5 rectangle, and two shape 5 into another 3x5 rectangle,
        // and putting all of them next to each other, small side by small side.
        // Packing 3 rectangles into a region would be a considerably easier problem
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_part1_example() {
        // This fails because I didn't really implement any packing whatsoever,
        // because it isn't needed for the full input.
        assert_eq!(2, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        // 1000 (all of them) is too high ;)
        // 427 are just the trivially possible ones
        assert_eq!(427, solve_part1(INPUT));
    }
}
