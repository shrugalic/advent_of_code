#[cfg(test)]
use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

#[test]
fn test_join_sequences_with_example_rule4() {
    // rule 4: "a"
    assert_eq!(Resolver::multiply(&[&vec!["a".to_string()]]), vec!["a"]);
}

#[test]
fn test_join_sequences_with_simple_rule() {
    assert_eq!(
        Resolver::join_sequences(&[&vec!["a".to_string()], &vec!["b".to_string()]]),
        vec!["a", "b"]
    );
}

#[test]
fn test_join_sequences_with_example_rule2() {
    assert_eq!(
        // rule 2: 4 4 | 5 5
        Resolver::join_sequences(&[
            &vec!["a".to_string(), "a".to_string()],
            &vec!["b".to_string(), "b".to_string()],
        ]),
        vec!["aa", "bb",]
    );
}

#[test]
fn test_join_sequences_with_example_rule3() {
    assert_eq!(
        // rule 3: 4 5 | 5 4
        Resolver::join_sequences(&[
            &vec!["a".to_string(), "b".to_string()],
            &vec!["b".to_string(), "a".to_string()]
        ]),
        vec!["ab", "ba"]
    );
}

#[test]
fn test_join_sequences_with_example_rule1() {
    assert_eq!(
        // rule 1: 2 3 | 3 2
        Resolver::join_sequences(&[
            // 2 3:
            &vec!["aa".to_string(), "ab".to_string(),],
            &vec!["aa".to_string(), "ba".to_string(),],
            &vec!["bb".to_string(), "ab".to_string(),],
            &vec!["bb".to_string(), "ba".to_string(),],
            // 3 2:
            &vec!["ab".to_string(), "aa".to_string(),],
            &vec!["ab".to_string(), "bb".to_string(),],
            &vec!["ba".to_string(), "aa".to_string(),],
            &vec!["ba".to_string(), "bb".to_string(),],
        ]),
        vec!["aaab", "aaba", "bbab", "bbba", "abaa", "abbb", "baaa", "babb"]
    );
}

#[test]
fn test_create_allowed_strings_with_example_rule2() {
    assert_eq!(
        // rule 2: 4 4 | 5 5
        Resolver::create_allowed_strings(
            &[vec!["a".to_string(), "a".to_string()]],
            &[vec!["b".to_string(), "b".to_string()]],
        ),
        vec!["aa", "bb",]
    );
}

#[test]
fn test_create_allowed_strings_with_example_rule3() {
    assert_eq!(
        // rule 3: 4 5 | 5 4
        Resolver::create_allowed_strings(
            &[vec!["a".to_string(), "b".to_string()]],
            &[vec!["b".to_string(), "a".to_string()]]
        ),
        vec!["ab", "ba"]
    );
}

// #[test]
// fn test_create_allowed_strings_with_example_rule1() {
//     // rule 1: 2 3 | 3 2
//     assert_eq!(
//         Resolver::create_allowed_strings(
//             // 2
//             &[vec!["aa".to_string(), "bb".to_string()]],
//             // 3
//             &[vec!["ab".to_string(), "ba".to_string()]],
//             // | 3
//             &[vec!["ab".to_string(), "ba".to_string()]],
//             // 2
//             &[vec!["aa".to_string(), "bb".to_string()]],
//         ),
//         vec!["aaab", "aaba", "bbab", "bbba", "abaa", "abbb", "baaa", "babb"]
//     );
// }

#[test]
fn test_multiply_with_example_rules() {
    assert_eq!(
        Resolver::multiply(&[
            &vec!["aa".to_string(), "bb".to_string()],
            &vec!["ab".to_string(), "ba".to_string()],
        ]),
        vec!["aaab", "aaba", "bbab", "bbba"]
    );
}

#[test]
fn test_multiply_more() {
    assert_eq!(
        Resolver::multiply(&[
            &vec!["a".to_string(), "b".to_string()],
            &vec!["c".to_string()]
        ]),
        vec!["ac", "bc"]
    );
    assert_eq!(
        Resolver::multiply(&[
            &vec!["a".to_string(), "b".to_string()],
            &vec!["c".to_string(), "d".to_string()]
        ]),
        vec!["ac", "ad", "bc", "bd"]
    );
    assert_eq!(
        Resolver::multiply(&[
            &vec!["a".to_string(), "b".to_string()],
            &vec!["c".to_string(), "d".to_string()],
            &vec!["e".to_string(), "f".to_string()]
        ]),
        vec!["ace", "acf", "ade", "adf", "bce", "bcf", "bde", "bdf"]
    );
}

#[test]
fn test_multiply_example_rule0() {
    assert_eq!(
        Resolver::multiply(&[
            &vec!["a".to_string()],
            &vec![
                "aaab".to_string(),
                "aaba".to_string(),
                "bbab".to_string(),
                "bbba".to_string(),
                "abaa".to_string(),
                "abbb".to_string(),
                "baaa".to_string(),
                "babb".to_string()
            ],
            &vec!["b".to_string()]
        ]),
        vec!["aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb"]
    );
}

#[test]
fn example1() {
    assert_eq!(
        number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE)),
        2
    );
}

// #[test]
// fn another_example() {
//     let rules = read_str_to_lines(
//         "0: 1 2 | 3 4 | 5 6
// 1: \"a\"
// 2: \"b\"
// 3: \"c\"
// 4: \"d\"
// 5: \"e\"
// 6: \"f\"",
//     );
//     assert_eq!(
//         Resolver::from(rules.as_slice()).resolve(),
//         Validator {
//             valid_messages: HashSet::new()
//         }
//     );
// }

#[test]
fn part1() {
    assert_eq!(
        number_of_messages_matching_rule_0(&read_file_to_lines("input.txt")),
        2
    );
}
