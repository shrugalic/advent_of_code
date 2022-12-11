use intcode::IntCodeComputer;
use crate::parse;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../input/day19.txt");

pub(crate) fn day19_part1() -> usize {
    count_1s_in_50x50_tractor_beam_picture()
}

pub(crate) fn day19_part2() -> usize {
    find_position_of_100_100_square()
}

fn count_1s_in_50x50_tractor_beam_picture() -> usize {
    let size = 50;
    let picture = collect_picture(size, size);
    // print(&picture);
    picture
        .iter()
        .map(|row| row.iter().filter(|v| v == &&1).count())
        .sum()
}

fn collect_picture(width: usize, height: usize) -> Vec<Vec<usize>> {
    (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .map(|x| scan_position(x, y))
                .collect()
        })
        .collect()
}

fn scan_position(x: usize, y: usize) -> usize {
    let mut icc = intcode_computer_from_puzzle_input();
    icc.add_inputs(&[x as isize, y as isize]);
    icc.run_until_halted().unwrap() as usize
}

fn find_position_of_100_100_square() -> usize {
    // beam starts at (6,5) and leaves the 50x50 pic at (49,40) at the top and (49,44) at the bottom
    // so a rough estimation of the lower bound is an upper vector (43, 35) and lower vector (43, 39)
    //
    // A square inset between these two lines will have its corners touch the lines,
    // and the diagonal between the corners will have a 45 degree angle (because it's a square)
    // which means that points on the diagonal sum up to some constant c = x + y
    //
    // Idea: For a certain y increase x until hitting the bottom line (square's bottom left corner)
    // check the corresponding top right corner: is it inside the beam, or outside?
    // if inside, check with smaller x until it's exactly on the edge
    // if outside, check with larger x until it's exactly on the edge

    let x = 1850;
    let mut bottom = 1680;
    loop {
        let (left, y) = find_bottom_left_corner(x, bottom);
        let (right, top) = find_top_right_corner(left, y);

        let size = right - left + 1;
        // println!(
        //     "b/l = ({}, {}), t/r = ({}, {}), size = {}",
        //     left, bottom, right, top, size
        // );

        match size.cmp(&100) {
            Ordering::Less => {
                bottom += 1;
            }
            Ordering::Greater => {
                bottom -= 1;
            }
            Ordering::Equal => {
                return 10_000 * left + top;
            }
        }
    }
}

fn find_bottom_left_corner(mut x: usize, bottom: usize) -> (usize, usize) {
    while scan_position(x, bottom) != 1 {
        x += 1;
    }
    (x, bottom)
}

fn find_top_right_corner(left: usize, bottom: usize) -> (usize, usize) {
    let (mut x, mut y) = (left, bottom);
    while scan_position(x + 1, y - 1) == 1 {
        x += 1;
        y -= 1;
    }
    (x, y)
}

#[allow(unused)]
fn print(picture: &[Vec<usize>]) {
    picture.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|v| if v == &1 { '#' } else { '.' })
                .collect::<String>()
        )
    });
}

fn intcode_computer_from_puzzle_input() -> IntCodeComputer {
    let input = parse(INPUT);
    let instr = input[0].split(',').map(|n| n.parse().unwrap()).collect();
    intcode::IntCodeComputer::new(instr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(118, day19_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(18651593, day19_part2());
    }
}

// Here's a 50x45 picture
// #.................................................
// ..................................................
// ..................................................
// ..................................................
// ..................................................
// ......#...........................................
// .......#..........................................
// ........#.........................................
// .........#........................................
// ..........##......................................
// ............#.....................................
// .............#....................................
// ..............#...................................
// ...............##.................................
// ................##................................
// .................##...............................
// ..................##..............................
// ...................##.............................
// ....................###...........................
// .....................###..........................
// .......................##.........................
// ........................##........................
// .........................###......................
// ..........................###.....................
// ...........................###....................
// ............................###...................
// .............................####.................
// ..............................####................
// ...............................####...............
// ................................####..............
// ..................................###.............
// ...................................####...........
// ....................................####..........
// .....................................####.........
// ......................................####........
// .......................................#####......
// ........................................#####.....
// .........................................#####....
// ..........................................#####...
// ...........................................######.
// .............................................#####
// ..............................................####
// ...............................................###
// ................................................##
// .................................................#

// A detail further down
// .#########.. 79
// .##########. 80
// ..########## 81 upper = (99, 81)
// ...######### 82
// ....######## 83
// .....####### 84
// ......###### 85
// .......##### 86
// ........#### 87
// .........### 88
// ...........# 89 lower = (99, 89)
// ............ 90
// 890123456789
// 889999999999
