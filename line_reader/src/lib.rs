use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads the content of a file into a Vec<String>, where each line will become a String
pub fn read_file_to_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().filter_map(|l| l.ok()).collect()
}

/// Splits the given str into a Vec<String>, where each line will become a String
pub fn read_str_to_lines(s: &'static str) -> Vec<String> {
    s.split('\n').map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use crate::{read_file_to_lines, read_str_to_lines};

    const EXPECTED_LINES: [&str; 3] = ["one", "two", "three"];

    #[test]
    fn read_file_to_lines_works() {
        let lines = read_file_to_lines("input.txt");
        assert_eq!(lines, EXPECTED_LINES);
    }

    #[test]
    fn read_str_to_lines_works() {
        let lines = read_str_to_lines(
            "one
two
three",
        );
        assert_eq!(lines, EXPECTED_LINES);
    }
}
