use md5::Digest;
use rayon::prelude::*;
use std::collections::HashMap;

const PUZZLE_INPUT: &str = "ahsbgdzn";

pub(crate) fn day14_part1() -> usize {
    index_of_64th_key_part1(PUZZLE_INPUT)
}

pub(crate) fn day14_part2() -> usize {
    index_of_64th_key_part2(PUZZLE_INPUT)
}

// First attempt used for part 1. It works but is really inefficient,
// because it calculates the same hash multiple times
#[allow(unused)]
fn index_of_64th_key_part_naive_brute_force(salt: &str) -> usize {
    let single_core = true;
    if single_core {
        (0..usize::MAX)
            .into_iter()
            .filter(|i| is_part1_key(salt, *i))
            .take(64) // too bad rayon can't also use take(64) ;)
            .last()
            .unwrap()
    } else {
        // On my 9900K (8-cores, 16-threads), a multiplier of 100 is about as fast as it gets.
        // Too little makes it slower, and too much would find unnecessarily many keys
        let step_size = 16 * 100;
        let mut start = 0;
        let mut all_keys = vec![];
        while all_keys.len() < 64 {
            let mut keys = (start..(start + step_size))
                .into_par_iter()
                .filter(|i| is_part1_key(salt, *i))
                .collect();
            all_keys.append(&mut keys);

            start += step_size;
        }
        all_keys[63]
    }
}

fn index_of_64th_key_part1(salt: &str) -> usize {
    index_of_64th_key(salt, true)
}
fn index_of_64th_key_part2(salt: &str) -> usize {
    index_of_64th_key(salt, false)
}
fn index_of_64th_key(salt: &str, part1: bool) -> usize {
    let hash_func = if part1 { md5 } else { stretched_md5 };
    let mut len5_indices: Vec<usize> = vec![];
    let single_core = false;
    if single_core {
        let mut len3_chars_by_index: HashMap<usize, char> = HashMap::new();
        let mut i5 = 0;
        while len5_indices.len() < 64 || len5_indices[63] + 1000 > i5 {
            let hash = hash_func(salt, i5);
            if let Some(c3) = first_char_appearing_3_times_in_a_row(hash) {
                len3_chars_by_index.insert(i5, c3);
            }
            if let Some(c5) = first_char_appearing_5_times_in_a_row(hash) {
                let mut i3s = index_of_matching(&len3_chars_by_index, c5, i5);
                len5_indices.append(&mut i3s);
            }
            i5 += 1;
            len5_indices.sort_unstable();
        }
    } else {
        let step_size = 16; // (Multiple of) 16 because i9 9900K has 8-cores with 16 threads
        let mut i5 = 0;
        let mut triples: HashMap<usize, char> = HashMap::new();
        while len5_indices.len() < 64 || len5_indices[63] + 1000 > i5 {
            let hashes3: Vec<_> = (i5..(i5 + step_size))
                .into_par_iter() // around 3 minutes single-core, or 23s in parallel
                .filter_map(|i5| {
                    let hash = if part1 {
                        md5(salt, i5)
                    } else {
                        stretched_md5(salt, i5)
                    };
                    first_char_appearing_3_times_in_a_row(hash).map(|c3| (i5, c3, hash))
                })
                .collect();
            let mut hashes5 = vec![];
            for (i5, c3, hash) in hashes3 {
                triples.insert(i5, c3);
                if let Some(c5) = first_char_appearing_5_times_in_a_row(hash) {
                    hashes5.push((i5, c5));
                }
            }
            hashes5.into_iter().for_each(|(i5, c5)| {
                let mut triples = index_of_matching(&triples, c5, i5);
                len5_indices.append(&mut triples);
            });
            i5 += step_size;
        }
    }
    len5_indices[63]
}

fn index_of_matching(triples: &HashMap<usize, char>, wanted: char, end: usize) -> Vec<usize> {
    let start = end.saturating_sub(1000);
    // println!(
    //     "5-tuple of '{}' @ {}, looking for triple in {:?}",
    //     wanted,
    //     end,
    //     start..end - 1
    // );
    (start..end)
        .into_iter()
        .filter_map(|i| {
            triples.get(&i).filter(|&&c| c == wanted).map(|_c| {
                // println!("- 3er '{}' @ {}", _c, i);
                i
            })
        })
        .collect()
}

fn is_part1_key(salt: &str, i: usize) -> bool {
    is_key(salt, i, &md5)
}

fn is_key(salt: &str, i: usize, hash_func: &dyn Fn(&str, usize) -> Digest) -> bool {
    if let Some(wanted) = first_char_appearing_3_times_in_a_row(hash_func(salt, i)) {
        wanted.appears_5_times_in_a_row_within_next_1000_hashes(i + 1, salt, hash_func)
    } else {
        false
    }
}

fn first_char_appearing_3_times_in_a_row(hash: Digest) -> Option<char> {
    first_char_appearing_n_times_in_a_row(hash, 3)
}

fn first_char_appearing_5_times_in_a_row(hash: Digest) -> Option<char> {
    first_char_appearing_n_times_in_a_row(hash, 5)
}

fn first_char_appearing_n_times_in_a_row(hash: Digest, n: usize) -> Option<char> {
    hash.to_char_vec()
        .windows(n)
        .find(are_all_equal)
        .map(|w| w[0])
}

fn md5(salt: &str, i: usize) -> Digest {
    md5::compute(format!("{}{}", salt, i))
}

fn stretched_md5(salt: &str, i: usize) -> Digest {
    let mut hash = md5(salt, i);
    for _ in 0..2016 {
        hash = md5::compute(format!("{:x}", hash));
    }
    hash
}

trait ToCharVec {
    fn to_char_vec(&self) -> Vec<char>;
}
impl ToCharVec for Digest {
    fn to_char_vec(&self) -> Vec<char> {
        format!("{:x}", self).chars().collect()
    }
}

trait Appears5TimesInARowWithinNext1000Hashes {
    fn appears_5_times_in_a_row_within_next_1000_hashes(
        &self,
        start: usize,
        salt: &str,
        hash_func: &dyn Fn(&str, usize) -> Digest,
    ) -> bool;
}
impl Appears5TimesInARowWithinNext1000Hashes for char {
    fn appears_5_times_in_a_row_within_next_1000_hashes(
        &self,
        start: usize,
        salt: &str,
        hash: &dyn Fn(&str, usize) -> Digest,
    ) -> bool {
        (start..start + 1000)
            .into_iter()
            .any(|i| hash(salt, i).contains_5_in_a_row(*self))
    }
}

trait Contains5InARow<T> {
    fn contains_5_in_a_row(&self, wanted: T) -> bool;
}
impl Contains5InARow<char> for Digest {
    fn contains_5_in_a_row(&self, wanted: char) -> bool {
        self.to_char_vec()
            .windows(5)
            .any(|w| wanted == w[0] && are_all_equal(&w))
    }
}

fn are_all_equal(w: &&[char]) -> bool {
    w.iter().skip(1).all(|c| c == &w[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SALT: &str = "abc";

    #[test]
    fn part1_example_index_18_is_a_triple_but_no_key() {
        let index = 18;
        let hash = md5(SALT, index);
        assert_eq!(Some('8'), first_char_appearing_3_times_in_a_row(hash));
        assert!(!'8'.appears_5_times_in_a_row_within_next_1000_hashes(index + 1, SALT, &md5));
    }

    #[test]
    fn part1_example_index_39_is_a_triple_and_a_key() {
        let index = 39;
        let hash = md5(SALT, index);
        assert_eq!(Some('e'), first_char_appearing_3_times_in_a_row(hash));
        assert!('e'.appears_5_times_in_a_row_within_next_1000_hashes(index + 1, SALT, &md5));
        assert!(is_part1_key(SALT, index));
    }

    #[test]
    fn part1_example_third_and_last_keys() {
        assert!(is_part1_key(SALT, 92));
        assert!(is_part1_key(SALT, 22728));
    }

    // Simple brute force approach takes around ~20s single core, < 3s multi-core
    // With the better part 2 approach it's less than 500ms
    #[test]
    fn part1_example() {
        assert_eq!(22728, index_of_64th_key_part1(SALT));
    }

    // Simple brute force approach takes around ~22s single core, < 3s multi-core
    // With the better part 2 approach it's less than 500ms
    #[test]
    fn part1() {
        assert_eq!(23_890, day14_part1());
    }

    #[test]
    fn part2_example_stretched_hash() {
        assert_eq!(
            "a107ff634856bb300138cac6568c0f24",
            format!("{:x}", stretched_md5(SALT, 0))
        );
    }

    #[test]
    fn part2_example_index_5_is_a_triple_but_no_key() {
        let index = 5;
        let hash = stretched_md5(SALT, index);
        assert_eq!(Some('2'), first_char_appearing_3_times_in_a_row(hash));
        assert!(!'2'.appears_5_times_in_a_row_within_next_1000_hashes(
            index + 1,
            SALT,
            &stretched_md5
        ));
    }

    #[test]
    fn part2_example_index_10_is_a_triple_and_a_key() {
        let index = 10;
        let hash = stretched_md5(SALT, index);
        assert_eq!(Some('e'), first_char_appearing_3_times_in_a_row(hash));
        assert!('e'.appears_5_times_in_a_row_within_next_1000_hashes(
            index + 1,
            SALT,
            &stretched_md5
        ));
        assert!(is_part2_key(SALT, index));
    }

    fn is_part2_key(salt: &str, i: usize) -> bool {
        is_key(salt, i, &stretched_md5)
    }

    #[test]
    fn part2_example() {
        assert_eq!(22_551, index_of_64th_key_part2(SALT));
    }

    #[test]
    fn part2() {
        assert_eq!(22_696, day14_part2());
    }
}
