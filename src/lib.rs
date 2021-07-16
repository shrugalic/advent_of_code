use std::cmp::min;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn product_of_2_and_3_counts(input: &[String]) -> usize {
    let (twos, threes) = input
        .iter()
        .map(crate::count_2_and_3_identical_letters)
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    twos * threes
}

fn count_2_and_3_identical_letters(line: &String) -> (usize, usize) {
    let mut count_by_letter: HashMap<char, usize> = HashMap::new();
    line.chars().for_each(|c| {
        *count_by_letter.entry(c).or_insert(0) += 1;
    });
    let count_of = |target| {
        count_by_letter
            .iter()
            .filter(|(_, count)| count == &&target)
            .count()
    };
    (min(1, count_of(2)), min(1, count_of(3)))
}
