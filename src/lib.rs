use std::collections::HashSet;

fn count_entries_with_required_fields(lines: &[String]) -> usize {
    count_valid_entries(lines, false)
}

fn count_entries_with_required_fields_and_valid_values(lines: &[String]) -> usize {
    count_valid_entries(lines, true)
}

fn count_valid_entries(lines: &[String], check_values: bool) -> usize {
    lines
        .split(|line| line.is_empty())
        .map(|parts| parts.to_vec().join(" "))
        .filter(|entry| is_entry_valid(entry, check_values))
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

fn is_entry_valid(entry: &String, check_values: bool) -> bool {
    // println!("{:?}", entry);
    let field_kvs: Vec<(&str, &str)> = entry
        .split(' ')
        .map(|field| {
            let kv = field.split(':').collect::<Vec<&str>>();
            (kv[0], kv[1])
        })
        .collect();
    let actual_keys: HashSet<&str> = field_kvs.iter().map(|(k, _v)| *k).collect();
    let required_keys: HashSet<&str> = REQUIRED_FIELDS.iter().cloned().collect();
    required_keys.difference(&actual_keys).count() == 0 && (!check_values || are_valid(&field_kvs))
}

fn are_valid(kvs: &[(&str, &str)]) -> bool {
    for (k, v) in kvs {
        match *k {
            "byr" => {
                if let Ok(yyyy) = v.parse::<isize>() {
                    if !(1920..=2002).contains(&yyyy) {
                        return false;
                    }
                }
            }
            "iyr" => {
                if let Ok(yyyy) = v.parse::<isize>() {
                    if !(2010..=2020).contains(&yyyy) {
                        return false;
                    }
                }
            }
            "eyr" => {
                if let Ok(yyyy) = v.parse::<isize>() {
                    if !(2020..=2030).contains(&yyyy) {
                        return false;
                    }
                }
            }
            "hgt" => {
                let number = v
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let suffix: &str = &v.chars().skip_while(|c| c.is_numeric()).collect::<String>();
                match suffix {
                    "in" => {
                        if !(59..=76).contains(&number) {
                            return false;
                        }
                    }
                    "cm" => {
                        if !(150..=193).contains(&number) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            "hcl" => {
                if v.len() != 7
                    || !v.starts_with('#')
                    || v.chars().skip(1).any(|c| !c.is_ascii_hexdigit())
                {
                    return false;
                }
            }
            "ecl" => match *v {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return false,
            },
            "pid" => {
                if v.len() != 9 || v.chars().any(|c| !c.is_numeric()) {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        count_entries_with_required_fields, count_entries_with_required_fields_and_valid_values,
    };
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
        assert_eq!(
            count_entries_with_required_fields(&read_str_to_lines(EXAMPLE)),
            2
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            count_entries_with_required_fields(&read_file_to_lines("input.txt")),
            226
        );
    }

    #[test]
    fn part2_valid_passports() {
        assert_eq!(
            count_entries_with_required_fields_and_valid_values(&read_str_to_lines(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            )),
            4
        );
    }

    #[test]
    fn part2_invalid_passports() {
        assert_eq!(
            count_entries_with_required_fields_and_valid_values(&read_str_to_lines(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            )),
            0
        );
    }
    #[test]
    fn part2() {
        assert_eq!(
            count_entries_with_required_fields_and_valid_values(&read_file_to_lines("input.txt")),
            160
        );
    }
}
