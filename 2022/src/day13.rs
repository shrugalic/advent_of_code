use std::cmp::Ordering;
use PacketData::*;

const INPUT: &str = include_str!("../input/day13.txt");

pub(crate) fn day13_part1() -> usize {
    let packets = parse_packets(INPUT);
    sum_of_indices_of_pairs_in_the_right_order(packets)
}

pub(crate) fn day13_part2() -> usize {
    let packets = parse_packets(INPUT);
    calculate_decoder_key(packets)
}

fn parse_packets(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::from)
        .collect()
}

fn sum_of_indices_of_pairs_in_the_right_order(packets: Vec<Packet>) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter(|(_, c)| c[0].cmp(&c[1]).is_in_right_order())
        .map(|(i, _)| i + 1)
        .sum()
}

fn calculate_decoder_key(mut packets: Vec<Packet>) -> usize {
    let divider1 = Packet::from("[[2]]");
    let divider2 = Packet::from("[[6]]");
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort_unstable();
    // println!("packets {:?}", packets);
    let pos1 = packets.iter().position(|e| e == &divider1).unwrap() + 1;
    let pos2 = packets.iter().position(|e| e == &divider2).unwrap() + 1;
    pos1 * pos2
}

trait IsInRightOrder {
    fn is_in_right_order(&self) -> bool;
}
impl IsInRightOrder for Ordering {
    fn is_in_right_order(&self) -> bool {
        match self {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => unreachable!("unless there are two equal packets"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketData {
    Integer(u8),
    List(Vec<PacketData>),
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("left {:?} right {:?}", left, right);
        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (List(l), List(r)) => l.cmp(r),
            (l @ List(_), Integer(r)) => l.cmp(&List(vec![Integer(*r)])),
            (Integer(l), r @ List(_)) => List(vec![Integer(*l)]).cmp(r),
        }
    }
}
impl PartialOrd<Self> for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Packet {
    contents: Vec<PacketData>,
}
impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        let mut contents_stack = vec![vec![]];
        let mut i = 1;
        let mut start = i;
        while i < s.len() {
            let c = &s[i..=i].chars().next().unwrap();
            match c {
                '[' => {
                    contents_stack.push(vec![]);
                }
                ']' => {
                    if let Ok(num) = &s[start..i].parse() {
                        contents_stack.last_mut().unwrap().push(Integer(*num));
                    }
                    let list = contents_stack.pop().unwrap();
                    if contents_stack.is_empty() {
                        // This was the outer-most/last list, and thus the packet is done
                        return Packet { contents: list };
                    }
                    let list = List(list);
                    contents_stack.last_mut().unwrap().push(list);
                }
                ',' => {
                    if let Ok(num) = &s[start..i].parse() {
                        contents_stack.last_mut().unwrap().push(Integer(*num));
                    }
                    start = i;
                }
                _ => { /* digit */ }
            }
            if !c.is_ascii_digit() {
                start += 1;
            }
            i += 1;
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
    fn packet_from_pair_1() {
        let (left, right) = parse(EXAMPLE)[0];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert_eq!(
            vec![Integer(1), Integer(1), Integer(3), Integer(1), Integer(1)],
            left
        );
        assert_eq!(
            vec![Integer(1), Integer(1), Integer(5), Integer(1), Integer(1)],
            right
        );
    }
    #[test]
    fn packet_from_pair_2() {
        let (left, right) = parse(EXAMPLE)[1];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert_eq!(
            vec![
                List(vec![Integer(1)]),
                List(vec![Integer(2), Integer(3), Integer(4)])
            ],
            left
        );
        assert_eq!(vec![List(vec![Integer(1)]), Integer(4)], right);
    }
    #[test]
    fn packet_from_pair_3() {
        let (left, right) = parse(EXAMPLE)[2];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert_eq!(vec![Integer(9)], left);
        assert_eq!(vec![List(vec![Integer(8), Integer(7), Integer(6)])], right);
    }
    #[test]
    fn packet_from_pair_4() {
        let (left, right) = parse(EXAMPLE)[3];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert_eq!(
            vec![List(vec![Integer(4), Integer(4)]), Integer(4), Integer(4)],
            left
        );
        assert_eq!(
            vec![
                List(vec![Integer(4), Integer(4)]),
                Integer(4),
                Integer(4),
                Integer(4)
            ],
            right
        );
    }

    #[test]
    fn is_in_right_order_work_for_packet_pair_1() {
        let (left, right) = parse(EXAMPLE)[0];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_2() {
        let (left, right) = parse(EXAMPLE)[1];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_3() {
        let (left, right) = parse(EXAMPLE)[2];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(!left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_4() {
        let (left, right) = parse(EXAMPLE)[3];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_5() {
        let (left, right) = parse(EXAMPLE)[4];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(!left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_6() {
        let (left, right) = parse(EXAMPLE)[5];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_7() {
        let (left, right) = parse(EXAMPLE)[6];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(!left.cmp(&right).is_in_right_order());
    }
    #[test]
    fn is_in_right_order_work_for_packet_pair_8() {
        let (left, right) = parse(EXAMPLE)[7];
        let left = Packet::from(left).contents;
        let right = Packet::from(right).contents;
        assert!(!left.cmp(&right).is_in_right_order());
    }

    #[test]
    fn part1_example() {
        let packets = parse_packets(EXAMPLE);
        assert_eq!(13, sum_of_indices_of_pairs_in_the_right_order(packets));
    }

    #[test]
    fn part1() {
        assert_eq!(6_369, day13_part1());
    }

    #[test]
    fn part2_example() {
        let packets = parse_packets(EXAMPLE);
        assert_eq!(140, calculate_decoder_key(packets));
    }

    #[test]
    fn part2() {
        assert_eq!(25_800, day13_part2());
    }
}
