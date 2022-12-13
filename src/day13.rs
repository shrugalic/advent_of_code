use std::cmp::Ordering;
use Element::*;

const INPUT: &str = include_str!("../input/day13.txt");

pub(crate) fn day13_part1() -> usize {
    let elements = parse_elements(INPUT);
    sum_of_indices_of_pairs_in_the_right_order(elements)
}

pub(crate) fn day13_part2() -> usize {
    let elements = parse_elements(INPUT);
    calculate_decoder_key(elements)
}

fn parse_elements(input: &str) -> Vec<Element> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Element::from)
        .collect()
}

fn sum_of_indices_of_pairs_in_the_right_order(elements: Vec<Element>) -> usize {
    elements
        .chunks(2)
        .enumerate()
        .filter(|(_, c)| (&c[0], &c[1]).is_in_right_order())
        .map(|(i, _)| i + 1)
        .sum()
}

fn calculate_decoder_key(mut elements: Vec<Element>) -> usize {
    let divider1 = Element::from("[[2]]");
    let divider2 = Element::from("[[6]]");
    elements.push(divider1);
    elements.push(divider2);
    elements.sort_unstable();
    // println!("elements {:?}", elements);
    let divider1 = Element::from("[[2]]");
    let divider2 = Element::from("[[6]]");
    let pos1 = elements.iter().position(|e| e == &divider1).unwrap() + 1;
    let pos2 = elements.iter().position(|e| e == &divider2).unwrap() + 1;
    pos1 * pos2
}

trait IsInRightOrder {
    fn is_in_right_order(&self) -> bool;
}
impl IsInRightOrder for (&Element, &Element) {
    fn is_in_right_order(&self) -> bool {
        match self.0.cmp(self.1) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => unreachable!("unless there are two equal elements"),
        }
    }
}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self;
        let right = other;
        // println!("left {:?} right {:?}", left, right);
        match (left, right) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (l @ List(_), Integer(r)) => l.cmp(&List(vec![Integer(*r)])),
            (Integer(l), r @ List(_)) => List(vec![Integer(*l)]).cmp(r),
            (List(left), List(right)) => {
                let (mut l, mut r) = (0, 0);
                while l < left.len() && r < right.len() {
                    match &left[l].cmp(&right[r]) {
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Equal => {
                            // continue
                        }
                    }
                    l += 1;
                    r += 1;
                }
                if l == left.len() && r < right.len() {
                    // println!("Left side ran out of items");
                    Ordering::Less
                } else if r == right.len() && l < left.len() {
                    // println!("Right side ran out of items");
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

/*
Vec<Element>
*/

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Integer(u8),
    List(Vec<Element>),
}
impl From<&str> for Element {
    fn from(s: &str) -> Self {
        let mut element_lists = vec![];
        for part in s.split(',') {
            let mut i = 0;
            while i < part.len() {
                match &part[i..=i] {
                    "[" => {
                        element_lists.push(vec![]);
                        i += 1;
                    }
                    "]" => {
                        let elements = element_lists.pop().unwrap();
                        let list = List(elements);
                        if element_lists.is_empty() {
                            // This was the last list, and thus we closed the outer-most list
                            return list;
                        }
                        element_lists.last_mut().unwrap().push(list);
                        i += 1;
                    }
                    _ => {
                        let start = i;
                        i += 1;
                        while i < part.len() && !["[", "]"].contains(&&part[i..=i]) {
                            i += 1;
                        }
                        let num: u8 = part[start..i].parse().unwrap();
                        element_lists.last_mut().unwrap().push(Integer(num));
                    }
                }
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    fn parse(input: &str) -> Vec<(&str, &str)> {
        input
            .split("\n\n")
            .map(|pair| pair.split_once('\n').unwrap())
            .collect()
    }

    #[test]
    fn element_from_pair_1() {
        let (left, right) = parse(EXAMPLE)[0];
        let left = Element::from(left);
        let right = Element::from(right);
        assert_eq!(
            List(vec![
                Integer(1),
                Integer(1),
                Integer(3),
                Integer(1),
                Integer(1),
            ]),
            left
        );
        assert_eq!(
            List(vec![
                Integer(1),
                Integer(1),
                Integer(5),
                Integer(1),
                Integer(1),
            ]),
            right
        );
    }
    #[test]
    fn element_from_pair_2() {
        let (left, right) = parse(EXAMPLE)[1];
        let left = Element::from(left);
        let right = Element::from(right);
        assert_eq!(
            List(vec![
                List(vec![Integer(1)]),
                List(vec![Integer(2), Integer(3), Integer(4)])
            ]),
            left
        );
        assert_eq!(List(vec![List(vec![Integer(1)]), Integer(4)]), right);
    }
    #[test]
    fn element_from_pair_3() {
        let (left, right) = parse(EXAMPLE)[2];
        let left = Element::from(left);
        let right = Element::from(right);
        assert_eq!(List(vec![Integer(9)]), left);
        assert_eq!(
            List(vec![List(vec![Integer(8), Integer(7), Integer(6)])]),
            right
        );
    }
    #[test]
    fn element_from_pair_4() {
        let (left, right) = parse(EXAMPLE)[3];
        let left = Element::from(left);
        let right = Element::from(right);
        assert_eq!(
            List(vec![
                List(vec![Integer(4), Integer(4)]),
                Integer(4),
                Integer(4)
            ]),
            left
        );
        assert_eq!(
            List(vec![
                List(vec![Integer(4), Integer(4)]),
                Integer(4),
                Integer(4),
                Integer(4)
            ]),
            right
        );
    }

    #[test]
    fn is_in_right_order_work_for_element_pair_1() {
        let (left, right) = parse(EXAMPLE)[0];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!((&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_2() {
        let (left, right) = parse(EXAMPLE)[1];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!((&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_3() {
        let (left, right) = parse(EXAMPLE)[2];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!(!(&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_4() {
        let (left, right) = parse(EXAMPLE)[3];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!((&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_5() {
        let (left, right) = parse(EXAMPLE)[4];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!(!(&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_6() {
        let (left, right) = parse(EXAMPLE)[5];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!((&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_7() {
        let (left, right) = parse(EXAMPLE)[6];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!(!(&left, &right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_element_pair_8() {
        let (left, right) = parse(EXAMPLE)[7];
        let left = Element::from(left);
        let right = Element::from(right);
        assert!(!(&left, &right).is_in_right_order());
    }

    #[test]
    fn part1_example() {
        let elements = parse_elements(EXAMPLE);
        assert_eq!(13, sum_of_indices_of_pairs_in_the_right_order(elements));
    }

    #[test]
    fn part1() {
        assert_eq!(6_369, day13_part1());
    }

    #[test]
    fn part2_example() {
        let elements = parse_elements(EXAMPLE);
        assert_eq!(140, calculate_decoder_key(elements));
    }

    #[test]
    fn part2() {
        assert_eq!(25_800, day13_part2());
    }
}
