use crate::parse;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day04.txt");

pub(crate) fn day4_part1() -> usize {
    parse(INPUT)
        .into_iter()
        .filter(|phrase| contains_only_unique_words(phrase))
        // The linter suggests the to change the above line to the following:
        // .filter(contains_only_unique_words)
        // but unfortunately this is incompatible with the .count() below:
        .count()
}

pub(crate) fn day4_part2() -> usize {
    parse(INPUT)
        .into_iter()
        // Same deal as above on line 8
        .filter(|phrase| contains_no_anagrams(phrase))
        .count()
}

fn contains_only_unique_words<T: AsRef<str>>(phrase: T) -> bool {
    let words = split_words(&phrase);
    let unique_words = words.iter().cloned().collect::<HashSet<_>>();
    unique_words.len() == words.len()
}

fn split_words<T: AsRef<str>>(phrase: T) -> Vec<String> {
    phrase
        .as_ref()
        .split_ascii_whitespace()
        .map(str::to_string)
        .collect()
}

fn contains_no_anagrams<T: AsRef<str>>(phrase: T) -> bool {
    let words = split_words(&phrase);
    let unique_words = words
        .iter()
        .map(|phrase| {
            let mut chars: Vec<char> = phrase.chars().collect();
            chars.sort_unstable();
            chars.iter().collect::<String>()
        })
        .collect::<HashSet<_>>();
    unique_words.len() == words.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert!(contains_only_unique_words("aa bb cc dd ee"));
        assert!(!contains_only_unique_words("aa bb cc dd aa"));
        assert!(contains_only_unique_words("aa bb cc dd aaa"));
    }

    #[test]
    fn part1() {
        assert_eq!(451, day4_part1());
    }

    #[test]
    fn examples_part2() {
        assert!(contains_no_anagrams("abcde fghij"));
        assert!(!contains_no_anagrams("abcde xyz ecdab"));
        assert!(contains_no_anagrams("a ab abc abd abf abj"));
        assert!(contains_no_anagrams("iiii oiii ooii oooi oooo"));
        assert!(!contains_no_anagrams("oiii ioii iioi iiio"));
    }

    #[test]
    fn part2() {
        assert_eq!(223, day4_part2());
    }
}
