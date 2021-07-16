use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub fn cumulate_frequency_adjustments(input: &[String]) -> isize {
    input.iter().map(|s| s.parse::<isize>().unwrap()).sum()
}

pub fn find_first_repeated_frequency(input: &[String]) -> isize {
    let adjustments: Vec<isize> = input.iter().map(|s| s.parse().unwrap()).collect();
    let mut seen: HashSet<isize> = HashSet::new();
    let mut freq = 0;
    loop {
        for adj in &adjustments {
            freq += adj;
            if seen.contains(&freq) {
                return freq;
            } else {
                seen.insert(freq);
            }
        }
    }
}
