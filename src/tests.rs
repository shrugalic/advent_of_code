#[cfg(test)]
use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

#[test]
fn test_resolve_choices_with_example_rule2() {
    assert_eq!(
        // rule 2: 4 4 | 5 5
        Resolver::concatenate(vec!["aa".to_string()], vec!["bb".to_string()],),
        vec!["aa", "bb",]
    );
}

#[test]
fn test_resolve_choices_with_example_rule3() {
    assert_eq!(
        // rule 3: 4 5 | 5 4
        Resolver::concatenate(vec!["ab".to_string()], vec!["ba".to_string()]),
        vec!["ab", "ba"]
    );
}

#[test]
fn test_multiply_with_example_rules() {
    assert_eq!(
        Resolver::generate_allowed_strings(vec![
            vec!["aa".to_string(), "bb".to_string()],
            vec!["ab".to_string(), "ba".to_string()],
        ]),
        vec!["aaab", "aaba", "bbab", "bbba"]
    );
}

#[test]
fn test_multiply_more() {
    assert_eq!(
        Resolver::generate_allowed_strings(vec![
            vec!["b".to_string()],
            vec!["ba".to_string(), "aa".to_string()]
        ]),
        vec!["bba", "baa"]
    );
    assert_eq!(
        Resolver::generate_allowed_strings(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()]
        ]),
        vec!["ac", "bc"]
    );
    assert_eq!(
        Resolver::generate_allowed_strings(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()]
        ]),
        vec!["ac", "ad", "bc", "bd"]
    );
    assert_eq!(
        Resolver::generate_allowed_strings(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
            vec!["e".to_string(), "f".to_string()]
        ]),
        vec!["ace", "acf", "ade", "adf", "bce", "bcf", "bde", "bdf"]
    );
}

#[test]
fn test_multiply_example_rule0() {
    assert_eq!(
        Resolver::generate_allowed_strings(vec![
            vec!["a".to_string()],
            vec![
                "aaab".to_string(),
                "aaba".to_string(),
                "bbab".to_string(),
                "bbba".to_string(),
                "abaa".to_string(),
                "abbb".to_string(),
                "baaa".to_string(),
                "babb".to_string()
            ],
            vec!["b".to_string()]
        ]),
        vec!["aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb"]
    );
}

const EXAMPLE_1: &str = "0: 4 1 5
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
fn example1() {
    assert_eq!(
        number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE_1)),
        2
    );
}
#[test]
fn example1_alternate() {
    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE_1)),
        2
    );
}

#[test]
fn direct_match() {
    let rules = Rules::from(vec!["0: \"a\""].as_slice());
    let messages = vec!["a", "b"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 1);
}

#[test]
fn direct_match_no_remaining_suffix() {
    let rules = Rules::from(vec!["0: \"a\""].as_slice());
    let messages = vec!["a", "aa"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 1);
}

#[test]
fn index_of_direct_match() {
    let rules = Rules::from(vec!["0: 1", "1: \"a\""].as_slice());
    let messages = vec!["a", "b", "aa"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 1);
}

#[test]
fn choice_of_direct_match() {
    let rules = Rules::from(vec!["0: 1 | 2", "1: \"a\"", "2: \"b\""].as_slice());
    let messages = vec!["a", "b", "c", "ab", "aa"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 2);
}

#[test]
fn pair_of_direct_match() {
    let rules = Rules::from(vec!["0: 1 2", "1: \"a\"", "2: \"b\""].as_slice());
    let messages = vec!["a", "b", "c", "ab", "ba", "aa"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 1);
}

#[test]
fn triple_of_direct_match() {
    let rules = Rules::from(vec!["0: 1 2 3", "1: \"a\"", "2: \"b\"", "3: \"c\""].as_slice());
    let messages = vec!["a", "b", "c", "d", "ab", "bc", "ac", "abc", ""];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 1);
}

#[test]
fn loop_8() {
    let rules = Rules::from(vec!["0: 8", "8: 42 | 42 8", "42: \"a\""].as_slice());
    let messages = vec!["a", "aa"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 2);
}

#[test]
fn loop_11() {
    let rules =
        Rules::from(vec!["0: 11", "11: 42 31 | 42 11 31", "42: \"a\"", "31: \"b\""].as_slice());
    let messages = vec!["ab", "aabb", "aaabbb", "a", "b", "abab"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 3);
}

#[test]
fn loop_8_and_11() {
    let rules = Rules::from(
        vec![
            "0: 8 11",
            "8: 42 | 42 8",
            "11: 42 31 | 42 11 31",
            "42: \"a\"",
            "31: \"b\"",
        ]
        .as_slice(),
    );
    let messages = vec!["aab"];
    assert_eq!(count_messages_matching_rules(messages.as_slice(), rules), 1);
}

// Slow: ~4s
// #[test]
fn part1() {
    assert_eq!(
        number_of_messages_matching_rule_0(&read_file_to_lines("input.txt")),
        156
    );
}

#[test]
fn part1_alternate() {
    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_file_to_lines("input.txt")),
        156
    );
}

#[test]
fn part2_alternate() {
    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_file_to_lines("input2.txt")),
        363
    );
}

const EXAMPLE_2_NO_LOOPS: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

#[test]
fn example2_no_loops() {
    assert_eq!(
        number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE_2_NO_LOOPS)),
        3
    );
}

#[test]
fn example2_no_loops_alternate() {
    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE_2_NO_LOOPS)),
        3
    );
}

const EXAMPLE_2_WITH_LOOPS: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

// Infinite loop
// #[test]
fn example2_with_loops() {
    assert_eq!(
        number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE_2_WITH_LOOPS)),
        12
    );
}

#[test]
fn example2_with_loops_alternate() {
    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_str_to_lines(EXAMPLE_2_WITH_LOOPS)),
        12
    );
}
