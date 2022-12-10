use crate::parse;

const INPUT: &str = include_str!("../input/day07.txt");

pub(crate) fn day07_part1() -> usize {
    parse(INPUT).into_iter().filter(|s| supports_tls(s)).count()
}

pub(crate) fn day07_part2() -> usize {
    parse(INPUT).into_iter().filter(|s| supports_ssl(s)).count()
}

fn supports_tls(ip: &str) -> bool {
    let (must_have_an_abba, must_not_have_any_abbas) = split(ip);

    must_have_an_abba.iter().any(|s| has_abba(s))
        && must_not_have_any_abbas.iter().all(|s| !has_abba(s))
}

fn supports_ssl(ip: &str) -> bool {
    let (supernet, hypernet) = split(ip);

    let babs: Vec<String> = supernet
        .into_iter()
        .flat_map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .windows(3)
                .filter(|w| w[0] == w[2] && w[1] != w[0])
                .map(|w| format!("{}{}{}", w[1], w[0], w[1]))
                .collect::<Vec<_>>()
        })
        .collect();

    hypernet.iter().any(|s| babs.iter().any(|b| s.contains(b)))
}

fn split(ip: &str) -> (Vec<&str>, Vec<&str>) {
    (
        ip.split(is_bracket).step_by(2).collect(),
        ip.split(is_bracket).skip(1).step_by(2).collect(),
    )
}

fn is_bracket(c: char) -> bool {
    c == '[' || c == ']'
}

fn has_abba(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|w| w[0] != w[1] && w[2] == w[1] && w[3] == w[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_abba() {
        assert!(has_abba("abba"));
        assert!(!has_abba("abcd"));
        assert!(!has_abba("qwer"));
        assert!(has_abba("ioxxoj"));
    }

    #[test]
    fn part1_examples() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn part1() {
        assert_eq!(105, day07_part1());
    }

    #[test]
    fn part2_examples() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn part2() {
        assert_eq!(258, day07_part2());
    }
}
