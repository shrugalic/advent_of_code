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
    CamelCards::from(input).sum_of_winnings()
}

fn sum_of_winnings_part2(input: &str) -> usize {
    CamelCards::from(input)
        .jacks_replaced_with_jokers()
        .sum_of_winnings()
}

struct CamelCards {
    hands: Vec<HandWithBid>,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Label {
    Joker,
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

impl From<&str> for CamelCards {
    fn from(input: &str) -> Self {
        let hands = input.trim().lines().map(HandWithBid::from).collect();
        CamelCards { hands }
    }
}

impl From<&str> for HandWithBid {
    fn from(s: &str) -> Self {
        let (hand, bid) = s.split_once(' ').expect("two parts per line");
        let hand = Hand::from(hand);
        let bid = bid.parse().expect("valid number as bid");
        HandWithBid { hand, bid }
    }
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        let cards: Vec<_> = hand.chars().map(Label::from).collect();
        let hand_type = Hand::hand_type_of(&cards);
        Hand { hand_type, cards }
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

impl CamelCards {
    fn sum_of_winnings(&mut self) -> usize {
        self.hands.sort_unstable();
        let rank = |i| -> usize { i + 1 };
        let winnings = |(i, line): (usize, &HandWithBid)| -> usize { rank(i) * line.bid };
        self.hands.iter().enumerate().map(winnings).sum()
    }
    fn jacks_replaced_with_jokers(mut self) -> Self {
        self.hands.iter_mut().for_each(HandWithBid::use_jokers);
        self
    }
}

impl HandWithBid {
    fn use_jokers(&mut self) {
        self.hand.use_jokers();
    }
    #[cfg(test)]
    fn with_jokers(mut self) -> Self {
        self.hand = self.hand.with_jokers();
        self
    }
}

impl Hand {
    fn base_hand_type(cards: &[Label]) -> Type {
        let mut l: Vec<_> = cards.iter().collect();
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
    fn hand_type_of(cards: &[Label]) -> Type {
        let base_hand_type = Self::base_hand_type(cards);
        let joker_count = cards.iter().filter(|label| label == &&Joker).count();
        if joker_count == 0 {
            return base_hand_type;
        }
        match base_hand_type {
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
                if joker_count == 2 || joker_count == 1 {
                    // 2: The joker pair can become any of the other cards for a three-of-a-kind
                    // 1: The single joker can upgrade the pair to a three-of-a-kind
                    ThreeOfAKind
                } else {
                    unreachable!("Unexpected joker_count {joker_count} in a single pair");
                }
            }
            // joker_count == 1 -> make the high-card a pair
            HighCard => OnePair,
        }
    }
    #[cfg(test)]
    fn with_jokers(mut self) -> Self {
        self.use_jokers();
        self
    }
    fn use_jokers(&mut self) {
        self.cards
            .iter_mut()
            .filter(|label| label == &&J)
            .for_each(|jack| std::mem::swap(jack, &mut Joker));
        self.hand_type = Hand::hand_type_of(&self.cards);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::Less;

    use super::*;

    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_label_sort_order() {
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
        assert_eq!(Hand::from("32T3K"), Hand::from("32T3K").with_jokers());
        assert_eq!(Hand::from("KK677"), Hand::from("KK677").with_jokers());
    }

    #[test]
    fn test_replace_jacks_with_jokers() {
        assert_eq!(
            Hand::from("T55J5").with_jokers().cards,
            vec![T, Five, Five, Joker, Five]
        );
    }

    #[test]
    fn test_example_hands_with_jokers_are_four_of_a_kind() {
        ["T55J5", "KTJJT", "QQQJA"]
            .into_iter()
            .map(Hand::from)
            .map(Hand::with_jokers)
            .for_each(|hand| assert_eq!(hand.hand_type, FourOfAKind));
    }

    #[test]
    fn test_part2_example_order() {
        let mut hands: Vec<_> = CamelCards::from(EXAMPLE).jacks_replaced_with_jokers().hands;
        hands.sort_unstable();
        assert_eq!(
            hands,
            vec![
                HandWithBid::from("32T3K 765").with_jokers(),
                HandWithBid::from("KK677 28").with_jokers(),
                HandWithBid::from("T55J5 684").with_jokers(),
                HandWithBid::from("QQQJA 483").with_jokers(),
                HandWithBid::from("KTJJT 220").with_jokers(),
            ]
        );
    }

    #[test]
    fn test_joker_counts_as_literal_when_breaking_ties() {
        let hand1 = Hand::from("JKKK2").with_jokers();
        let hand2 = Hand::from("QQQQ2").with_jokers();
        assert_eq!(hand1.hand_type, FourOfAKind);
        assert_eq!(hand2.hand_type, FourOfAKind);
        assert_eq!(hand1.cmp(&hand2), Less);
    }

    #[test]
    fn test_multiple_jokers() {
        assert_eq!(Hand::from("QJJQ2").with_jokers().hand_type, FourOfAKind);
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
