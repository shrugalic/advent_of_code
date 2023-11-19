use crate::parse;

const INPUT: &str = include_str!("../input/day09.txt");

pub(crate) fn day9_part1() -> usize {
    let groups = parse(INPUT);
    score_groups(&groups[0])
}

pub(crate) fn day9_part2() -> usize {
    let groups = parse(INPUT);
    garbage_char_count(&groups[0])
}

fn score_groups(input: &str) -> usize {
    process_groups(input).1
}

fn garbage_char_count(input: &str) -> usize {
    process_groups(input).2
}

fn process_groups(input: &str) -> (usize, usize, usize) {
    let mut count = 0;
    let mut score = 0;
    let mut total = 0;
    let mut garbage_char_count = 0;
    let mut open_garbage: Option<Garbage> = None;
    for ch in input.chars() {
        match open_garbage.take() {
            Some(garbage) => {
                let char_count = garbage.char_count;
                open_garbage = garbage.process(ch);
                if open_garbage.is_none() {
                    garbage_char_count += char_count;
                }
            }
            None => {
                match ch {
                    // open group
                    '{' => score += 1,
                    // close and score group
                    '}' => {
                        count += 1;
                        total += score;
                        score -= 1
                    }
                    // start garbage
                    _ => open_garbage = Garbage::from(ch),
                }
            }
        }
    }
    (count, total, garbage_char_count)
}

#[derive(Debug)]
struct Garbage {
    prev: char,
    char_count: usize,
}
impl Garbage {
    fn from(ch: char) -> Option<Garbage> {
        if ch == '<' {
            Some(Garbage {
                prev: ch,
                char_count: 0,
            })
        } else {
            None
        }
    }
    fn process(mut self, ch: char) -> Option<Garbage> {
        if self.prev == '!' {
            self.char_count -= 1;
            self.prev = ' ';
        } else if ch == '>' {
            return None;
        } else {
            self.prev = ch;
            self.char_count += 1;
        }
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_groups(input: &str) -> usize {
        process_groups(input).0
    }

    fn is_garbage(input: &str) -> bool {
        let mut input: Vec<char> = input.chars().rev().collect();
        if let Some(ch) = input.pop() {
            if ch != '<' {
                return false;
            }
            let mut open_garbage = Garbage::from(ch);
            if open_garbage.is_none() {
                return false;
            }
            while let Some(ch) = input.pop() {
                let garbage = open_garbage.take().unwrap();
                open_garbage = garbage.process(ch);
                if open_garbage.is_none() {
                    return input.is_empty();
                }
            }
        }
        false
    }

    #[test]
    fn garbage_examples() {
        assert!(is_garbage("<>"));
        assert!(is_garbage("<random characters>"));
        assert!(is_garbage("<<<<>"));
        assert!(is_garbage("<{!>}>"));
        assert!(is_garbage("<!!>"));
        assert!(is_garbage("<!!!>>"));
        assert!(is_garbage("<{o\"i!a,<{i<a>"));
        // negative example
        assert!(!is_garbage("<{>}>"));
    }

    #[test]
    fn count_group_examples() {
        assert_eq!(1, count_groups("{}"));
        assert_eq!(3, count_groups("{{{}}}"));
        assert_eq!(3, count_groups("{{},{}}"));
        assert_eq!(6, count_groups("{{{},{},{{}}}}"));
        assert_eq!(1, count_groups("{<{},{},{{}}>}"));
        assert_eq!(1, count_groups("{<a>,<a>,<a>,<a>}"));
        assert_eq!(5, count_groups("{{<a>},{<a>},{<a>},{<a>}}"));
        assert_eq!(2, count_groups("{{<!>},{<!>},{<!>},{<a>}}"));
    }

    #[test]
    fn score_group_examples() {
        assert_eq!(1, score_groups("{}"));
        assert_eq!(6, score_groups("{{{}}}"));
        assert_eq!(5, score_groups("{{},{}}"));
        assert_eq!(16, score_groups("{{{},{},{{}}}}"));
        assert_eq!(1, score_groups("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, score_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, score_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, score_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }

    #[test]
    fn part1() {
        assert_eq!(16827, day9_part1());
    }

    #[test]
    fn garbage_char_count_examples() {
        assert_eq!(0, garbage_char_count("<>"));
        assert_eq!(17, garbage_char_count("<random characters>"));
        assert_eq!(3, garbage_char_count("<<<<>"));
        assert_eq!(2, garbage_char_count("<{!>}>"));
        assert_eq!(0, garbage_char_count("<!!>"));
        assert_eq!(0, garbage_char_count("<!!!>>"));
        assert_eq!(10, garbage_char_count("<{o\"i!a,<{i<a>"));
    }

    #[test]
    fn part2() {
        assert_eq!(7298, day9_part2());
    }
}
