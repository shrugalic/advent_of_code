#[cfg(test)]
mod tests;

pub fn cumulate_frequency_adjustments(input: &[String]) -> isize {
    input.iter().map(|s| s.parse::<isize>().unwrap()).sum()
}
