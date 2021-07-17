#[cfg(test)]
mod tests;

pub fn remaining_units_after_reaction(input: &str) -> usize {
    let mut polymer: Vec<_> = input.chars().map(Some).collect();
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

const CAPS_DIFF /* 32 */ : isize = 'a' as isize /* 97 */ - 'A' as isize /* 65 */;

fn are_same_char_different_case(c1: char, c2: char) -> bool {
    isize::abs(c1 as isize - c2 as isize) == CAPS_DIFF
}
