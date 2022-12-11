use crate::parse;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day03.txt");

type Coordinate = (usize, usize);

#[derive(PartialEq, Debug)]
struct Claim {
    id: usize,
    from_left: usize,
    from_top: usize,
    width: usize,
    height: usize,
}
impl Claim {
    fn to_coordinates(&self) -> Vec<Coordinate> {
        let (from_left, from_top) = (self.from_left, self.from_top);
        (0..self.height)
            .into_iter()
            .flat_map(|y| {
                (0..self.width)
                    .into_iter()
                    .map(move |x| (from_left + x, from_top + y))
            })
            .collect()
    }
}

impl<T: AsRef<str>> From<T> for Claim {
    fn from(s: T) -> Self {
        let (id, parts) = s.as_ref().split_once(" @ ").unwrap();
        let id = id.trim_start_matches('#').parse().unwrap();
        let (top_left, width_height) = parts.split_once(": ").unwrap();
        let (from_left, from_top) = top_left.split_once(',').unwrap();
        let (width, height) = width_height.split_once('x').unwrap();
        Claim {
            id,
            from_left: from_left.parse().unwrap(),
            from_top: from_top.parse().unwrap(),
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
        }
    }
}

pub(crate) fn day3_part1() -> usize {
    overlapping_claim_count(&parse(INPUT))
}

fn overlapping_claim_count(input: &[&str]) -> usize {
    let claims: Vec<_> = input.iter().map(Claim::from).collect();
    let count_by_coordinate = get_counts_by_coordinate(&claims);
    count_by_coordinate.values().filter(|v| v > &&1).count()
}

pub(crate) fn day3_part2() -> usize {
    id_of_non_overlapping_claim(&parse(INPUT))
}

fn id_of_non_overlapping_claim(input: &[&str]) -> usize {
    let claims: Vec<_> = input.iter().map(Claim::from).collect();
    let count_by_coordinate = get_counts_by_coordinate(&claims);
    claims
        .iter()
        .find(|claim| {
            claim
                .to_coordinates()
                .iter()
                .all(|coord| count_by_coordinate.get(coord).unwrap() == &1)
        })
        .unwrap()
        .id
}

fn get_counts_by_coordinate(claims: &Vec<Claim>) -> HashMap<Coordinate, usize> {
    let mut count_by_coordinate: HashMap<Coordinate, usize> = HashMap::new();
    claims
        .iter()
        .flat_map(Claim::to_coordinates)
        .for_each(|coord| {
            *count_by_coordinate.entry(coord).or_insert(0) += 1;
        });
    count_by_coordinate
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn claim_from_string() {
        assert_eq!(
            Claim::from("#123 @ 3,2: 5x4"),
            Claim {
                id: 123,
                from_left: 3,
                from_top: 2,
                width: 5,
                height: 4,
            }
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(4, overlapping_claim_count(&parse(EXAMPLE)));
    }

    #[test]
    fn part_1() {
        assert_eq!(113_576, day3_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(3, id_of_non_overlapping_claim(&parse(EXAMPLE)));
    }

    #[test]
    fn part_2() {
        assert_eq!(825, day3_part2());
    }
}
