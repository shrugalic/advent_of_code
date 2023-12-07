use std::cmp::Ordering;
use Ordering::*;

use Label::*;
use Type::*;

const INPUT: &str = include_str!("../input/day07.txt");

pub(crate) fn part1() -> usize {
    sum_of_winnings_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    sum_of_winnings_part2(INPUT)
}

fn sum_of_winnings_part1(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort_unstable_by(HandWithBid::ordering_without_jokers);
    sum_of_winnings(hands)
}

fn sum_of_winnings_part2(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort_unstable_by(HandWithBid::ordering_with_jokers);
    sum_of_winnings(hands)
}

fn parse(input: &str) -> Vec<HandWithBid> {
    input.trim().lines().map(HandWithBid::from).collect()
}

fn sum_of_winnings(hands: Vec<HandWithBid>) -> usize {
    let rank = |i| -> usize { i + 1 };
    let winnings = |(i, line): (usize, HandWithBid)| -> usize { rank(i) * line.bid };
    hands.into_iter().enumerate().map(winnings).sum()
}

#[derive(Debug, PartialEq)]
struct HandWithBid {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: Type,
    cards: Vec<Label>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Label {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<&str> for HandWithBid {
    fn from(s: &str) -> Self {
        let (hand, bid) = s.split_once(" ").expect("two parts per line");
        let hand = Hand::from(hand);
        let bid = bid.parse().expect("valid number as bid");
        HandWithBid { hand, bid }
    }
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        let cards: Vec<_> = hand.chars().map(Label::from).collect();
        let hand_type = Hand::type_without_jokers_of(&cards);
        Hand { cards, hand_type }
    }
}

impl From<char> for Label {
    fn from(label: char) -> Self {
        match label {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => T,
            'J' => J,
            'Q' => Q,
            'K' => K,
            'A' => A,
            _ => unreachable!("Unknown label '{label}'"),
        }
    }
}

impl HandWithBid {
    fn ordering_without_jokers(left: &HandWithBid, right: &HandWithBid) -> Ordering {
        left.hand.cmp(&right.hand)
    }
    fn ordering_with_jokers(left: &HandWithBid, right: &HandWithBid) -> Ordering {
        Hand::ordering_with_jokers(&left.hand, &right.hand)
    }
}

impl Hand {
    fn type_without_jokers(&self) -> Type {
        Hand::type_without_jokers_of(&self.cards)
    }
    fn type_without_jokers_of(cards: &[Label]) -> Type {
        let mut l: Vec<_> = cards.iter().cloned().collect();
        l.sort_unstable();

        if l[0] == l[4] {
            FiveOfAKind
        } else if l[0] == l[3] || l[1] == l[4] {
            FourOfAKind
        } else if l[0] == l[2] && l[3] == l[4] || l[0] == l[1] && l[2] == l[4] {
            FullHouse
        } else if l[0] == l[2] || l[1] == l[3] || l[2] == l[4] {
            ThreeOfAKind
        } else if l[0] == l[1] && (l[2] == l[3] || l[3] == l[4]) || l[1] == l[2] && l[3] == l[4] {
            TwoPair
        } else if l[0] == l[1] || l[1] == l[2] || l[2] == l[3] || l[3] == l[4] {
            OnePair
        } else {
            HighCard
        }
    }
    fn type_with_jokers(&self) -> Type {
        let joker_count = self.cards.iter().filter(|label| label == &&J).count();
        if joker_count == 0 {
            return self.type_without_jokers();
        }
        match self.type_without_jokers() {
            // 1 <= joker_count <= 5
            FiveOfAKind => FiveOfAKind,
            // 1 <= joker_count <= 4
            FourOfAKind => {
                if joker_count == 1 || joker_count == 4 {
                    FiveOfAKind
                } else {
                    unreachable!("Unexpected joker_count {joker_count} in a four-of-a-kind");
                }
            }
            // 1 <= joker_count <= 3
            FullHouse => {
                if joker_count == 2 || joker_count == 3 {
                    FiveOfAKind
                } else {
                    unreachable!("Unexpected joker_count {joker_count} in a full-house");
                }
            }
            ThreeOfAKind => {
                // joker_count cannot be 2, because that would make this a full-house
                if joker_count == 1 || joker_count == 3 {
                    FourOfAKind
                } else {
                    unreachable!("Unexpected joker_count {joker_count} in a three-of-a-kind");
                }
            }
            // 1 <= joker_count <= 2
            TwoPair => {
                if joker_count == 2 {
                    // One of the pairs is a joker, which can become the other pair's label
                    FourOfAKind
                } else if joker_count == 1 {
                    // The joker can make one of the pairs into a triple to form a full-house
                    FullHouse
                } else {
                    unreachable!("Unexpected joker_count {joker_count} in a two-pair");
                }
            }
            OnePair => {
                if joker_count == 2 {
                    // The joker pair can become any of the other cards for a three-of-a-kind
                    ThreeOfAKind
                } else if joker_count == 1 {
                    // The single joker can upgrade the pair to a three-of-a-kind
                    ThreeOfAKind
                } else {
                    unreachable!("Unexpected joker_count {joker_count} in a single pair");
                }
            }
            // joker_count == 1 -> make the high-card a pair
            HighCard => OnePair,
        }
    }
    fn ordering_with_jokers(left: &Hand, right: &Hand) -> Ordering {
        let ordering_by_type = left.type_with_jokers().cmp(&right.type_with_jokers());
        match ordering_by_type {
            Less | Greater => ordering_by_type,
            Equal => Hand::ordering_by_cards(left, right),
        }
    }
    fn ordering_by_cards(left: &Hand, right: &Hand) -> Ordering {
        left.cards
            .iter()
            .zip(right.cards.iter())
            .map(Label::compare_with_jokers)
            .filter(|o| o != &Equal)
            .next()
            .unwrap_or(Equal)
    }
}

impl Label {
    fn compare_with_jokers((left, right): (&Label, &Label)) -> Ordering {
        match (left == &J, right == &J) {
            (true, false) => Less,
            (false, true) => Greater,
            (false, false) => left.cmp(right),
            (true, true) => Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_label_order() {
        let mut labels: Vec<_> = "4T29J8Q7K6A35".chars().map(Label::from).collect();
        labels.sort_unstable();
        assert_eq!(
            labels,
            vec![Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A]
        );
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(6_440, sum_of_winnings_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(250_254_244, sum_of_winnings_part1(INPUT));
    }

    #[test]
    fn test_example_hands_without_jokers_have_base_type() {
        let hand1 = Hand::from("32T3K");
        assert_eq!(hand1.type_with_jokers(), hand1.type_without_jokers());

        let hand2 = Hand::from("KK677");
        assert_eq!(hand2.type_with_jokers(), hand2.type_without_jokers());
    }

    #[test]
    fn test_example_hands_with_jokers_are_four_of_a_kind() {
        assert_eq!(Hand::from("T55J5").type_with_jokers(), FourOfAKind);
        assert_eq!(Hand::from("KTJJT").type_with_jokers(), FourOfAKind);
        assert_eq!(Hand::from("QQQJA").type_with_jokers(), FourOfAKind);
    }

    #[test]
    fn test_part2_example_order() {
        let mut hands = parse(EXAMPLE);
        hands.sort_unstable_by(|a, b| Hand::ordering_with_jokers(&a.hand, &b.hand));
        assert_eq!(
            hands,
            vec![
                HandWithBid::from("32T3K 765"),
                HandWithBid::from("KK677 28"),
                HandWithBid::from("T55J5 684"),
                HandWithBid::from("QQQJA 483"),
                HandWithBid::from("KTJJT 220"),
            ]
        );
    }

    #[test]
    fn test_joker_counts_as_literal_when_breaking_ties() {
        assert_eq!(Hand::from("JKKK2").type_with_jokers(), FourOfAKind);
        assert_eq!(Hand::from("QQQQ2").type_with_jokers(), FourOfAKind);
        assert_eq!(
            Hand::ordering_with_jokers(&Hand::from("JKKK2"), &Hand::from("QQQQ2")),
            Less
        );
    }

    #[test]
    fn test_multiple_jokers() {
        assert_eq!(Hand::from("QJJQ2").type_with_jokers(), FourOfAKind);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(5_905, sum_of_winnings_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(250_087_440, sum_of_winnings_part2(INPUT));
    }
}
