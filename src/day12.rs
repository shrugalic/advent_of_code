use line_reader::read_file_to_lines;

pub(crate) fn day12_part1() -> isize {
    sum_of_numbers(read_file_to_lines("input/day12.txt"))
}

pub(crate) fn day12_part2() -> isize {
    sum_of_numbers_without_red(read_file_to_lines("input/day12.txt"))
}

fn sum_of_numbers(input: Vec<String>) -> isize {
    input
        .iter()
        .map(|line| {
            line.split(|c| [',', '[', ']', '{', '}', ':'].contains(&c))
                .filter_map(|s| s.parse::<isize>().ok())
                .sum::<isize>()
        })
        .sum()
}

fn sum_of_numbers_without_red(input: Vec<String>) -> isize {
    input.iter().map(|s| sum_without_red(s)).sum()
}

fn sum_without_red(line: &str) -> isize {
    // let parts: Vec<_> = line.split(|c| c == ',').collect();
    let line: Vec<_> = line.chars().collect();
    let (sum, rest) = parse_sum_without_red(line, None);
    assert!(rest.is_empty());
    sum
}

fn parse_sum_without_red(line: Vec<char>, terminator: Option<char>) -> (isize, Vec<char>) {
    let mut sum = 0;
    let mut rest = line;
    let mut element_chars = vec![];
    let mut contains_red = false;
    while !rest.is_empty() {
        let c = rest.remove(0);
        match c {
            '[' | '{' => {
                let term = if c == '[' { ']' } else { '}' };
                let (child_sum, child_rest) = parse_sum_without_red(rest, Some(term));
                sum += child_sum;
                rest = child_rest;
            }
            ',' | ']' | '}' => {
                if !element_chars.is_empty() {
                    let element: String = element_chars.iter().collect();
                    if let Some((_key, value)) = element.split_once(':') {
                        // object element
                        if value == "\"red\"" {
                            contains_red = true;
                        }
                        if let Ok(value) = value.parse::<isize>() {
                            sum += value;
                        }
                    } else {
                        // array element
                        if let Ok(value) = element.parse::<isize>() {
                            sum += value;
                        }
                    }
                    element_chars.clear();
                }
                if let Some(term) = terminator {
                    if c == term {
                        // this array/object is done
                        break;
                    }
                }
            }
            c => {
                element_chars.push(c);
            }
        }
    }
    if contains_red {
        sum = 0;
    }
    // println!(
    //     "Done. sum {} rest = '{}'",
    //     sum,
    //     rest.iter().collect::<String>()
    // );
    (sum, rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    #[test]
    fn part1_examples() {
        assert_eq!(6, sum_of_numbers(read_str_to_lines("[1,2,3]")));
        assert_eq!(6, sum_of_numbers(read_str_to_lines("{\"a\":2,\"b\":4}")));
        assert_eq!(3, sum_of_numbers(read_str_to_lines("[[[3]]]")));
        assert_eq!(
            3,
            sum_of_numbers(read_str_to_lines("{\"a\":{\"b\":4},\"c\":-1}"))
        );
        assert_eq!(0, sum_of_numbers(read_str_to_lines("{\"a\":[-1,1]}")));
        assert_eq!(0, sum_of_numbers(read_str_to_lines("[-1,{\"a\":1}]")));
        assert_eq!(0, sum_of_numbers(read_str_to_lines("[]")));
        assert_eq!(0, sum_of_numbers(read_str_to_lines("{}")));
    }

    #[test]
    fn part1() {
        assert_eq!(119433, day12_part1());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(6, sum_of_numbers_without_red(read_str_to_lines("[1,2,3]")));
        assert_eq!(
            4,
            sum_of_numbers_without_red(read_str_to_lines("[1,{\"c\":\"red\",\"b\":2},3]"))
        );
        assert_eq!(
            0,
            sum_of_numbers_without_red(read_str_to_lines(
                "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"
            ))
        );
        assert_eq!(
            6,
            sum_of_numbers_without_red(read_str_to_lines("[1,\"red\",5]"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(68466, day12_part2());
    }
}
