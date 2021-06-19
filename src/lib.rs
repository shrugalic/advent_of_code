fn product_of_joltage_diff_1_and_3_counts(numbers: &[String]) -> usize {
    let mut numbers: Vec<usize> = numbers.iter().map(|s| s.parse().unwrap()).collect();
    numbers.push(0); // outlet
    numbers.push(*numbers.iter().max().unwrap() + 3); // input
    numbers.sort_unstable();
    let diffs: Vec<_> = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    println!("sorted = {:?}", numbers);
    println!(" diffs = {:?}", diffs);

    let count_of_1 = diffs.iter().filter(|&&n| n == 1).count();
    let count_of_3 = diffs.iter().filter(|&&n| n == 3).count();

    count_of_1 * count_of_3
}

#[cfg(test)]
mod tests {
    use crate::product_of_joltage_diff_1_and_3_counts;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn part1_example_1() {
        assert_eq!(
            product_of_joltage_diff_1_and_3_counts(&read_str_to_lines(EXAMPLE_1)),
            7 * 5
        );
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(
            product_of_joltage_diff_1_and_3_counts(&read_str_to_lines(EXAMPLE_2)),
            22 * 10
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            product_of_joltage_diff_1_and_3_counts(&read_file_to_lines("input.txt")),
            2080
        );
    }
}
