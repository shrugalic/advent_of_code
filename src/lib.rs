use std::collections::HashSet;

fn valid_entries(lines: &[String]) -> usize {
    lines
        .split(|line| line.is_empty())
        .map(|parts| parts.to_vec().join(" "))
        .filter(|entry| is_entry_valid(entry))
        .count()
}
const REQUIRED_FIELDS: [&'static str; 7] = [
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
];

fn is_entry_valid(entry: &String) -> bool {
    // println!("{:?}", entry);
    let actual_fields: HashSet<&str> = entry
        .split(' ')
        .map(|field| field.split(':').next().unwrap())
        .collect();

    let required_fields: HashSet<&str> = REQUIRED_FIELDS.iter().cloned().collect();
    required_fields.difference(&actual_fields).count() == 0
}

#[cfg(test)]
mod tests {
    use crate::valid_entries;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn part1_example() {
        assert_eq!(valid_entries(&read_str_to_lines(EXAMPLE)), 2);
    }

    #[test]
    fn part1() {
        assert_eq!(valid_entries(&read_file_to_lines("input.txt")), 226);
    }
}
