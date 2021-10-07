use md5::Digest;
use rayon::prelude::*;
use std::thread;

const PUZZLE_INPUT: &str = "abbhdwsy";

pub(crate) fn day05_part1() -> String {
    generate_part1_password_from(PUZZLE_INPUT)
}

pub(crate) fn day05_part2() -> String {
    generate_part2_password_from(PUZZLE_INPUT)
}

// The following code is adapted from 2015 day 4

#[allow(unused)]
enum CalcType {
    Threaded, // About 10s for part 1 |  4s part 2 example |  9s part 2
    Parallel, // About  3s for part 1 |  5s part 2 example |  9s part 2
    Single,   // About 23s for part 1 | 39s part 2 example | 74s part 2
}

fn generate_part1_password_from(secret_key: &str) -> String {
    generate_password_with_filter(secret_key, Part::One)
}
fn generate_part2_password_from(secret_key: &str) -> String {
    generate_password_with_filter(secret_key, Part::Two)
}

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

fn generate_password_with_filter(secret_key: &str, part: Part) -> String {
    let calc = CalcType::Parallel;
    let mut results: Vec<(usize, char, char)> = Vec::new();
    let mut password: Vec<Option<char>> = vec![None; 8];
    match calc {
        CalcType::Threaded => {
            let thread_count = 16;
            let step_size = 10_000; // seems to work fine on my 9900K (8-cores, 16-threads)
            let mut start = 0;
            while (part == Part::One && results.len() < 8)
                || (part == Part::Two && password.iter().any(|x| x.is_none()))
            {
                let mut handles = vec![];
                for from in start..(start + thread_count) {
                    let secret_key = secret_key.to_string();
                    handles.push(thread::spawn(move || {
                        let mut results = vec![];
                        for i in (from..from + step_size).into_iter().step_by(thread_count) {
                            if let Some(result) = check_match(&secret_key, i) {
                                results.push(result);
                            }
                        }
                        Some(results)
                    }));
                }
                for handle in handles {
                    if let Some(mut result) = handle.join().unwrap() {
                        results.append(&mut result);
                    }
                }
                results.sort_by_key(|&(i, _, _)| i);
                if part == Part::Two {
                    results.iter().for_each(|(_, sixth, seventh)| {
                        let pos = sixth.to_digit(16).unwrap() as usize;
                        if pos < 8 && password[pos].is_none() {
                            password[pos] = Some(*seventh);
                        }
                    });
                }
                start += step_size;
            }
        }
        CalcType::Parallel => {
            let step_size = 1_000_000; // seems to work fine on my 9900K (8-cores, 16-threads)
            let mut start = 0;
            while (part == Part::One && results.len() < 8)
                || (part == Part::Two && password.iter().any(|x| x.is_none()))
            {
                let mut matches: Vec<_> = (start..(start + step_size))
                    .into_par_iter()
                    .filter_map(|i| check_match(secret_key, i))
                    .collect();
                matches.sort_by_key(|&(i, _, _)| i);

                results.append(&mut matches);
                if part == Part::Two {
                    results.iter().for_each(|(_, sixth, seventh)| {
                        let pos = sixth.to_digit(16).unwrap() as usize;
                        if pos < 8 && password[pos].is_none() {
                            password[pos] = Some(*seventh);
                        }
                    });
                }
                start += step_size;
            }
        }
        CalcType::Single => match part {
            Part::One => {
                results = (0..(usize::MAX))
                    .into_iter()
                    .filter_map(|i| check_match(secret_key, i))
                    .take(8)
                    .collect();
            }
            Part::Two => {
                let mut i = 0;
                while password.iter().any(|x| x.is_none()) {
                    if let Some((_, sixth, seventh)) = check_match(secret_key, i) {
                        let pos = sixth.to_digit(16).unwrap() as usize;
                        if pos < 8 && password[pos].is_none() {
                            password[pos] = Some(seventh);
                        }
                    }
                    i += 1;
                }
            }
        },
    }
    match part {
        Part::One => results
            .into_iter()
            .take(8)
            .map(|(_, c, _)| c)
            .collect::<String>(),
        Part::Two => password.into_iter().map(|x| x.unwrap()).collect::<String>(),
    }
}

fn check_match(secret_key: &str, i: usize) -> Option<(usize, char, char)> {
    let digest = md5::compute(format!("{}{}", secret_key, i));
    if starts_with_5_leading_zeroes(digest) {
        let hash = format!("{:x}", digest);
        // println!("digest {}", hash);
        Some((
            i,
            hash.chars().nth(5).unwrap(),
            hash.chars().nth(6).unwrap(),
        ))
    } else {
        None
    }
}

fn starts_with_5_leading_zeroes(digest: Digest) -> bool {
    // The 5 zeroes are made up of 5 hex digits
    // Two hex digits are made up of a single u8
    // 16 equals 0b10000, so < 16 means the left-most 4 bits are zero
    digest[0] == 0 && digest[1] == 0 && digest[2] < 16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!("18f47a30", generate_part1_password_from("abc"));
    }
    #[test]
    fn part1() {
        assert_eq!("801b56a7", day05_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!("05ace8e3", generate_part2_password_from("abc"));
    }

    #[test]
    fn part2() {
        assert_eq!("424a0197", day05_part2());
    }
}
