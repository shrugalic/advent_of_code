use md5::Digest;
use rayon::prelude::*;

const PUZZLE_INPUT: &str = "yzbqklnj";

pub(crate) fn day04_part1() -> usize {
    smallest_i_where_hash_starts_with_5_zeroes(PUZZLE_INPUT)
}

pub(crate) fn day04_part2() -> usize {
    smallest_i_where_hash_starts_with_6_zeroes(PUZZLE_INPUT)
}

fn smallest_i_where_hash_starts_with_5_zeroes(secret_key: &str) -> usize {
    hash_until_filter_matches(secret_key, starts_with_5_leading_zeroes)
}
fn smallest_i_where_hash_starts_with_6_zeroes(secret_key: &str) -> usize {
    hash_until_filter_matches(secret_key, starts_with_6_leading_zeroes)
}

fn hash_until_filter_matches(secret_key: &str, filter: fn(Digest) -> bool) -> usize {
    let parallel = true;
    if parallel {
        let step_size = 16_000; // seems to work fine on my 9900K
        let mut start = 1;
        loop {
            if let Some(min) = (start..(start + step_size))
                .into_par_iter()
                .filter(|i| filter(md5::compute(format!("{}{}", secret_key, i))))
                .min()
            {
                return min;
            }
            start += step_size;
        }
    } else {
        let mut i = 1;
        while !filter(md5::compute(format!("{}{}", secret_key, i))) {
            i += 1;
        }
        i
    }
}

fn starts_with_5_leading_zeroes(digest: Digest) -> bool {
    // The 5 zeroes are made up of 5 hex digits
    // Two hex digits are made up of a single u8
    digest[0] == 0 && digest[1] == 0 && digest[2] < 8
}
fn starts_with_6_leading_zeroes(digest: Digest) -> bool {
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            609_043,
            smallest_i_where_hash_starts_with_5_zeroes("abcdef")
        );
    }
    #[test]
    fn part1_example2() {
        assert_eq!(
            1_048_970,
            smallest_i_where_hash_starts_with_5_zeroes("pqrstuv")
        );
    }

    #[test]
    fn part1() {
        assert_eq!(282749, day04_part1());
    }

    // 27s single-core, ~4.5s multi-core
    #[test]
    fn part2() {
        assert_eq!(9962624, day04_part2());
    }
}
