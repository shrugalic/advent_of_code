type PlayerCount = usize;
type Score = usize;

pub(crate) fn high_score(players: PlayerCount, last_marble: Score) -> Score {
    let mut marbles = vec![0];
    let mut scores = vec![0; players];
    let mut curr = 0;
    for marble in 1..=last_marble {
        // println!("marble {}, curr = {}", marble, curr);
        if marble % 23 == 0 {
            let player = marble % players;
            scores[player] += marble;
            curr = (curr + marbles.len() - 7) % marbles.len();
            scores[player] += marbles.remove(curr);
        } else {
            curr = (curr + 2) % marbles.len();
            marbles.insert(curr, marble);
        }
    }
    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_example() {
        assert_eq!(32, high_score(9, 25));
    }

    #[test]
    fn longer_examples() {
        assert_eq!(8_317, high_score(10, 1_618));
        assert_eq!(146_373, high_score(13, 7_999));
        assert_eq!(2_764, high_score(17, 1_104));
        assert_eq!(54_718, high_score(21, 6_111));
        assert_eq!(37_305, high_score(30, 5_807));
    }

    #[test]
    fn part_1() {
        assert_eq!(374_690, high_score(477, 70_851));
    }
}
