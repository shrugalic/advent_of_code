type PlayerCount = usize;
type Score = usize;
type Marble = usize;

struct Neighbors {
    prev: Marble,
    next: Marble,
}
impl Neighbors {
    fn new(prev: Marble, next: Marble) -> Self {
        Neighbors { prev, next }
    }
}

pub(crate) fn high_score(players: PlayerCount, last_marble: Score) -> Score {
    let mut current: Marble = 0;
    // linked_marbles works sort of like a doubly linked list, to keep track of the neighboring
    // marbles in both directions (counter-clockwise and clockwise). Its index corresponds to the
    // marble value, and the value contains indices of the previous and next marbles respectively.
    let mut linked_marbles: Vec<Neighbors> = vec![Neighbors::new(current, current)];
    let mut scores = vec![0; players];
    for placed in 1..=last_marble {
        if placed % 23 == 0 {
            // Set current to be clockwise of the marble to remove
            for _ in 1..7 {
                current = linked_marbles[current].prev;
            }
            // It will not actually be removed, just unlinked and never be linked to again
            let removed: Marble = linked_marbles[current].prev;
            let previous: Marble = linked_marbles[removed].prev;
            linked_marbles[previous].next = current;
            linked_marbles[current].prev = previous;

            let player = placed % players;
            scores[player] += removed;
            scores[player] += placed;

            // Insert dummy value to keep index-equals-marble-value property
            linked_marbles.push(Neighbors::new(placed, placed));
        } else {
            let next: Marble = linked_marbles[current].next;
            let after_next: Marble = linked_marbles[next].next;
            // The new marble goes in-between these two

            current = placed;
            linked_marbles[next].next = current;
            linked_marbles[after_next].prev = current;
            linked_marbles.push(Neighbors::new(next, after_next));
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

    #[test]
    fn part_2() {
        assert_eq!(3_009_951_158, high_score(477, 7_085_100));
    }
}
