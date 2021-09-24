use std::collections::HashSet;

const PUZZLE_INPUT: &str = "vzbxkghb";
const INVALID_CHARS: [char; 3] = ['i', 'l', 'o'];

pub(crate) fn day11_part1() -> String {
    generate_next_password(PUZZLE_INPUT)
}

pub(crate) fn day11_part2() -> String {
    generate_next_password(&generate_next_password(PUZZLE_INPUT))
}

fn generate_next_password(input: &str) -> String {
    let mut password: Vec<_> = input.chars().collect();
    password.increment();
    while !password.is_valid() {
        password.increment();
    }
    password.iter().collect()
}

trait PasswordValidityCheck {
    fn is_valid(&self) -> bool;
}
impl PasswordValidityCheck for Vec<char> {
    fn is_valid(&self) -> bool {
        let contains_chain = self
            .windows(3)
            .any(|s| s[0] as u8 + 1 == s[1] as u8 && s[1] as u8 + 1 == s[2] as u8);
        let contains_invalid_chars = self.iter().any(|c| INVALID_CHARS.contains(c));
        let different_pair_count = self
            .windows(2)
            .filter_map(|s| if s[0] == s[1] { Some(s[0]) } else { None })
            .collect::<HashSet<char>>()
            .len();

        contains_chain && !contains_invalid_chars && different_pair_count >= 2
    }
}
impl PasswordValidityCheck for &str {
    fn is_valid(&self) -> bool {
        self.chars().collect::<Vec<_>>().is_valid()
    }
}

trait Increment {
    // increments self and returns if there was wrap-around
    fn increment(&mut self) -> bool;
}
impl Increment for char {
    fn increment(&mut self) -> bool {
        let mut num = *self as u8;
        assert!(b'a' <= num);
        assert!(num <= b'z');
        if num == b'z' {
            num = b'a';
            *self = num as char;
            true
        } else {
            num += 1;

            // This check is not strictly necessary, but speeds things up.
            if INVALID_CHARS.contains(&(num as char)) {
                num += 1;
            }

            *self = num as char;
            false
        }
    }
}
impl Increment for Vec<char> {
    fn increment(&mut self) -> bool {
        let len = self.len();
        let mut increment_next = true;

        // This block is merely a speed-up to avoid needlessly incrementing chars toward
        // the back, when an invalid char in the front needs to be incremented.
        // It works just fine without this optimisation
        let mut incremented_leading_char = false;
        for c in self.iter_mut() {
            if incremented_leading_char {
                *c = 'a'
            } else if INVALID_CHARS.contains(c) {
                c.increment();
                incremented_leading_char = true;
            }
        }

        self.iter_mut().rev().for_each(|c| {
            if increment_next {
                let wrapped_current = c.increment();
                increment_next = wrapped_current;
            }
        });
        if increment_next {
            self.insert(0, 'a');
        }
        self.len() > len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_increment_works() {
        let mut c = 'a';
        let did_wrap_around = c.increment();
        assert!(!did_wrap_around);
        assert_eq!('b', c);
    }

    #[test]
    fn char_increment_wraps_around() {
        let mut c = 'z';
        let did_wrap_around = c.increment();
        assert!(did_wrap_around);
        assert_eq!('a', c);
    }

    #[test]
    fn char_vec_increment_works() {
        let mut c = vec!['a'];
        let did_wrap_around = c.increment();
        assert!(!did_wrap_around);
        assert_eq!(vec!['b'], c);
    }

    #[test]
    fn char_vec_increment_counts_length_increase_as_wrap_around() {
        let mut c = vec!['z'];
        let did_wrap_around = c.increment();
        assert!(did_wrap_around);
        assert_eq!(vec!['a', 'a'], c);
    }

    #[test]
    fn char_vec_increment_does_not_count_single_char_wrap_without_length_increase_as_wrap() {
        let mut c = vec!['a', 'z'];
        let did_wrap_around = c.increment();
        assert!(!did_wrap_around);
        assert_eq!(vec!['b', 'a'], c);
    }

    #[test]
    fn is_valid() {
        // Own examples
        assert!("aabcxx".is_valid());
        assert!(!"aabcxxi".is_valid()); // invalid character 'i'
        assert!(!"aabxx".is_valid()); // no chain
        assert!(!"abcxx".is_valid()); // no two pairs
        assert!(!"aabcaa".is_valid()); // no two _different_ pairs

        // Given examples:
        assert!(!"hijklmmn".is_valid());
        assert!(!"abbceffg".is_valid());
        assert!(!"abbcegjk".is_valid());
    }

    #[test]
    fn test_generate_next_password() {
        assert_eq!("abcdffaa", generate_next_password("abcdefgh"));
        assert_eq!("ghjaabcc", generate_next_password("ghijklmn"));
    }

    #[test]
    fn part1() {
        assert_eq!("vzbxxyzz", day11_part1());
    }

    #[test]
    fn part2() {
        assert_eq!("vzcaabcc", day11_part2());
    }
}
