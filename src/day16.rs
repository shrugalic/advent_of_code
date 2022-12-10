use crate::parse;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn day16_part1() -> String {
    dance_once()
}

pub(crate) fn day16_part2() -> String {
    dance_a_billion_times()
}

fn dance_once() -> String {
    let moves = parse_dance_moves();
    let mut programs = get_programs();
    programs.dance(&moves);
    programs.iter().collect()
}

fn get_programs() -> Vec<char> {
    "abcdefghijklmnop".chars().collect::<Vec<_>>()
}

fn dance_a_billion_times() -> String {
    let moves = parse_dance_moves();
    let mut programs = get_programs();

    let mut seen = HashMap::new();
    seen.insert(programs.iter().collect::<String>(), 0);

    let mut i = 0;
    let first;
    loop {
        programs.dance(&moves);
        i += 1;
        if let Some(prev) = seen.insert(programs.iter().collect::<String>(), i) {
            first = prev;
            break;
        }
    }
    let period = i - first;
    let total = 1_000_000_000;
    let cycles = (total - first) / period;
    let missing = total - first - cycles * period;
    let round = first + missing;

    seen.iter().find(|(_, i)| i == &&round).unwrap().0.clone()
}

trait Dance {
    fn dance(&mut self, moves: &[Move]);
}

impl Dance for Vec<char> {
    fn dance(&mut self, moves: &[Move]) {
        for dance_move in moves {
            match dance_move {
                Move::Spin(s) => self.rotate_right(*s),
                Move::Exchange(a, b) => self.swap(*a, *b),
                Move::Partner(a, b) => {
                    let a = self.iter().position(|c| c == a).unwrap();
                    let b = self.iter().position(|c| c == b).unwrap();
                    self.swap(a, b);
                }
            }
        }
    }
}

fn parse_dance_moves() -> Vec<Move> {
    let input = parse(INPUT);
    input[0].split(',').map(Move::from).collect()
}

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match &s[0..1] {
            "s" => Move::Spin(s[1..].parse().unwrap()),
            "x" => {
                let (l, r) = s[1..].split_once('/').unwrap();
                Move::Exchange(l.parse().unwrap(), r.parse().unwrap())
            }
            "p" => {
                let (l, r) = s[1..].split_once('/').unwrap();
                Move::Partner(l.parse().unwrap(), r.parse().unwrap())
            }
            _ => panic!("invalid move {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!("olgejankfhbmpidc", day16_part1());
    }

    #[test]
    fn part2() {
        assert_eq!("gfabehpdojkcimnl", day16_part2());
    }
}
