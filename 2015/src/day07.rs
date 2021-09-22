use line_reader::read_file_to_lines;
use std::collections::HashMap;

pub(crate) fn day07_part1() -> Value {
    let input = read_file_to_lines("input/day07.txt");
    let signals = determine_signals(input);
    *signals.get(&"a".to_string()).unwrap()
}

pub(crate) fn day07_part2() -> Value {
    let input = read_file_to_lines("input/day07.txt");
    let mut instructions: Vec<_> = input.into_iter().map(Instruction::from).collect();
    instructions.iter_mut().for_each(|instr| {
        if let Instruction::Signal { v, o } = instr {
            if "b".eq(o) {
                *v = 46065;
            }
        }
    });
    let signals = signals_from(instructions);
    *signals.get(&"a".to_string()).unwrap()
}

fn determine_signals(input: Vec<String>) -> HashMap<Id, Value> {
    let instructions: Vec<_> = input.into_iter().map(Instruction::from).collect();
    signals_from(instructions)
}

fn signals_from(mut instructions: Vec<Instruction>) -> HashMap<Id, Value> {
    let mut signals = Signals::new();
    while let Some(pos) = instructions.iter().position(|instr| match instr {
        Instruction::Signal { .. } => true,
        Instruction::Forward { i, o: _ } => signals.contains_key(i),
        Instruction::AndV { i1: _, i2, o: _ } => signals.contains_key(i2),
        Instruction::And { i1, i2, o: _ } | Instruction::Or { i1, i2, o: _ } => {
            signals.contains_key(i1) && signals.contains_key(i2)
        }
        Instruction::LShift { i, v: _, o: _ } | Instruction::RShift { i, v: _, o: _ } => {
            signals.contains_key(i)
        }
        Instruction::Not { i, o: _ } => signals.contains_key(i),
    }) {
        match instructions.remove(pos) {
            Instruction::Signal { v, o } => {
                signals.insert(o, v);
            }
            Instruction::Forward { i, o } => {
                let v = *signals.get(&i).unwrap();
                signals.insert(o, v);
            }
            Instruction::AndV { i1, i2, o } => {
                signals.insert(o, i1 & signals.get(&i2).unwrap());
            }
            Instruction::And { i1, i2, o } => {
                signals.insert(o, signals.get(&i1).unwrap() & signals.get(&i2).unwrap());
            }
            Instruction::Or { i1, i2, o } => {
                signals.insert(o, signals.get(&i1).unwrap() | signals.get(&i2).unwrap());
            }
            Instruction::LShift { i, v, o } => {
                signals.insert(o, signals.get(&i).unwrap() << v);
            }
            Instruction::RShift { i, v, o } => {
                signals.insert(o, signals.get(&i).unwrap() >> v);
            }
            Instruction::Not { i, o } => {
                signals.insert(o, !(*signals.get(&i).unwrap()));
            }
        }
    }
    signals
}

type Id = String;
type Value = u16;
type Signals = HashMap<Id, Value>;

#[derive(Debug)]
enum Instruction {
    Signal { v: Value, o: Id },
    Forward { i: Id, o: Id },
    AndV { i1: Value, i2: Id, o: Id },
    And { i1: Id, i2: Id, o: Id },
    Or { i1: Id, i2: Id, o: Id },
    LShift { i: Id, v: Value, o: Id },
    RShift { i: Id, v: Value, o: Id },
    Not { i: Id, o: Id },
}
impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let p: Vec<_> = s.split_ascii_whitespace().collect();
        match p.len() {
            3 => {
                let i = p[0].to_string();
                let o = p[2].to_string();
                if let Ok(v) = i.parse() {
                    Instruction::Signal { v, o }
                } else {
                    Instruction::Forward { i, o }
                }
            }
            4 => Instruction::Not {
                i: p[1].to_string(),
                o: p[3].to_string(),
            },
            5 => {
                let i1 = p[0].to_string();
                let i2 = p[2].to_string();
                let o = p[4].to_string();
                match p[1] {
                    "AND" => {
                        if let Ok(i1) = i1.parse() {
                            Instruction::AndV { i1, i2, o }
                        } else {
                            Instruction::And { i1, i2, o }
                        }
                    }
                    "OR" => Instruction::Or { i1, i2, o },
                    "LSHIFT" => Instruction::LShift {
                        i: i1,
                        v: i2.parse().unwrap(),
                        o,
                    },
                    "RSHIFT" => Instruction::RShift {
                        i: i1,
                        v: i2.parse().unwrap(),
                        o,
                    },
                    _ => panic!("Unknown 5-word instruction {}", s),
                }
            }
            _ => panic!("Unknown instruction {}", s),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn part1_example() {
        let signals = determine_signals(read_str_to_lines(EXAMPLE));
        println!("signals {:?}", signals);
        assert_eq!(Some(&72), signals.get("d"));
        assert_eq!(Some(&507), signals.get("e"));
        assert_eq!(Some(&492), signals.get("f"));
        assert_eq!(Some(&114), signals.get("g"));
        assert_eq!(Some(&65412), signals.get("h"));
        assert_eq!(Some(&65079), signals.get("i"));
        assert_eq!(Some(&123), signals.get("x"));
        assert_eq!(Some(&456), signals.get("y"));
    }

    #[test]
    fn part1() {
        assert_eq!(46065, day07_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(14134, day07_part2());
    }
}
