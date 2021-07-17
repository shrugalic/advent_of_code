use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub fn remaining_units_after_reaction(input: &str) -> usize {
    let polymer: Vec<_> = input.chars().map(Some).collect();
    remaining_units_after_fully_reacting(polymer)
}

pub fn length_of_shortest_possible_polymer(input: &str) -> usize {
    let polymer: Vec<_> = input.chars().map(Some).collect();
    let unique_chars: HashSet<char> = polymer.iter().map(|c| c.unwrap()).collect();
    println!("{} unique chars {:?}", unique_chars.len(), unique_chars);
    let mut min = usize::MAX;
    for i in 65..=90 {
        let cleaned = polymer
            .iter()
            .filter(|o| o.unwrap() != (i as char) && o.unwrap() != (i + CAPS_DIFF) as char)
            .cloned()
            .collect();
        min = usize::min(min, remaining_units_after_fully_reacting(cleaned));
    }
    min
}

fn remaining_units_after_fully_reacting(mut polymer: Vec<Option<char>>) -> usize {
    let mut removed_one = true;
    while removed_one {
        removed_one = false;
        for c in 1..polymer.len() {
            if let Some(curr) = polymer[c] {
                let mut p = c - 1;
                while p > 0 && polymer[p].is_none() {
                    p -= 1;
                }
                if let Some(prev) = polymer[p] {
                    if are_same_char_different_case(prev, curr) {
                        polymer[c] = None;
                        polymer[p] = None;
                        removed_one = true;
                    }
                }
            }
        }
    }
    polymer.iter().filter(|c| c.is_some()).count()
}

const CAPS_DIFF /* 32 */ : u8 = b'a' /* 97 */ - b'A' /* 65 */;

fn are_same_char_different_case(a: char, b: char) -> bool {
    CAPS_DIFF == u8::max(a as u8, b as u8) - u8::min(a as u8, b as u8)
}
