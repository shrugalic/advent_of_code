use crate::parse;
use std::mem::swap;

const INPUT: &str = include_str!("../input/day22.txt");

const PART1_DECK_SIZE: usize = 10_007;
const PART2_DECK_SIZE: usize = 119_315_717_514_047;
const PART2_SHUFFLE_COUNT: usize = 101_741_582_076_661;

pub(crate) fn day22_part1() -> usize {
    let input = parse(INPUT);
    let track_single_card_only = true;
    if track_single_card_only {
        let mut deck = SingleCardPosTrackingDeck::new(2019, PART1_DECK_SIZE);
        let techniques = parse_shuffle_techniques(input);
        deck.shuffle_with(&techniques);
        deck.pos
    } else {
        let deck = shuffle_deck(PART1_DECK_SIZE, input);
        deck.iter().position(|&v| v == 2019).unwrap()
    }
}

pub(crate) fn day22_part2() -> usize {
    // To determine what the value of the card ending up at position 2020 is, start at the end:
    // Start with position 2020, and apply all shuffles backwards/inverted.
    // The position will end up as the initial position, which is equal to the original value.

    // This is impossible to calculate directly because of the large number of shuffles.
    // (BTW: both the deck size and shuffle count are (large) prime numbers)

    // From <https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnifwk/>:
    // All the operations applied to the deck are linear. Their composition is also linear.
    // Thus there exist integers A and B such that f(i) = (A * i + B) % M, where M is the deck_size.

    // To determine the coefficients a and b, we can apply the shuffle twice and solve equations:
    // X = 2020, f(X) = Y, f(Z) = f(Y) = f(f(X)), with Y = A * X + B and Z = A * Y + B
    // Subtracting the latter from the former yields Y - Z = A * X + B - (A * Y + B) = A (X - Y)
    // Thus A = (Y - Z) / (X - Y) = (Y - Z) * modular_inverse(X - Y)
    // and B = Y - (A * X) or B = Z - (A * Y)
    let initial = 2020;
    let size = PART2_DECK_SIZE;

    // Let's shuffle twice to get the values for y (once) and z (twice)
    let results = shuffle_n_times(initial, size, 2);
    let (a, b) = determine_coefficients(initial, results[1], results[2], size);

    apply_linear_function_n_times(initial, size, a, b, PART2_SHUFFLE_COUNT)
}

fn shuffle_n_times(initial: usize, size: usize, n: usize) -> Vec<usize> {
    let mut deck = SingleCardPosTrackingDeck::new(initial, size);
    let input = parse(INPUT);
    let techniques = parse_shuffle_techniques(input);
    let inverted = invert(deck.size, techniques);

    let mut result = vec![initial];

    for _ in 0..n {
        deck.shuffle_with(&inverted);
        result.push(deck.pos);
    }

    result
}

fn determine_coefficients(x: usize, y: usize, z: usize, size: usize) -> (usize, usize) {
    // let a = (y - z) / (x - y); -> (y - z) * modular_inverse(x - y)
    let y_minus_z = modular_subtract(y, z, size);
    let x_minus_y = modular_subtract(x, y, size);
    let mod_inv_x_minus_y = modular_inverse(x_minus_y, size);
    let a = modular_multiply(y_minus_z, mod_inv_x_minus_y, size);

    // let b = y - (a * x)
    let a_times_x = modular_multiply(a, x, size);
    let b = modular_subtract(y, a_times_x, size);

    (a, b)
}

fn apply_linear_function_n_times(init: usize, size: usize, a: usize, b: usize, n: usize) -> usize {
    let a_pow_n = modular_power(a, n, size);
    (modular_multiply(a_pow_n, init, size)
        + modular_multiply(
            a_pow_n - 1,
            modular_multiply(modular_inverse(a - 1, size), b, size),
            size,
        ))
        % size
}

/// Invert the shuffle techniques
fn invert(deck_size: usize, techniques: Vec<ShuffleTechnique>) -> Vec<ShuffleTechnique> {
    techniques
        .into_iter()
        .rev()
        .map(|tech| tech.invert(deck_size))
        .collect()
}

/// Calculate x - y % size
fn modular_subtract(x: usize, y: usize, size: usize) -> usize {
    if x >= y {
        x - y
    } else {
        x + size - y
    }
}

/// calculate (x * y) % size, handling overflow
fn modular_multiply(x: usize, y: usize, size: usize) -> usize {
    if let Some(product) = x.checked_mul(y) {
        product % size
    } else {
        let (x, y, size) = (x as u128, y as u128, size as u128);
        ((x * y) % size) as usize
    }
}

/// Calculate f2 = 1 / f1 in a modular way. f2 = modular_inverse(f1, size) means
/// finding an f2 such that f1 * f2 % size == 1.
fn modular_inverse(f1: usize, size: usize) -> usize {
    if false {
        // Naive version
        let mut f2 = 1;
        while f2 % f1 != 0 {
            // Easy to run into overflow here with large numbers like 106461317257343.
            // Tried using u128 but was way too slow for this large number
            f2 += size;
        }
        f2 /= f1;
        f2
    } else {
        // Adapted from https://stackoverflow.com/q/68338719
        if size == 1 {
            return 1;
        }
        let p = size as i128;
        let (mut o, mut m) = (f1 as i128, p as i128);
        while o < 0 {
            o += m;
        }
        let mut x = 0;
        let mut inv = 1;
        while o > 1 {
            let div = o / m;
            o %= m;
            inv -= div * x;
            swap(&mut o, &mut m);
            swap(&mut x, &mut inv);
        }
        if inv < 0 {
            inv += p;
        }

        inv as usize
    }
}

/// Calculate base ^ exp % size
fn modular_power(base: usize, exp: usize, size: usize) -> usize {
    let mut bits: Vec<_> = format!("{:b}", exp)
        .chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => unreachable!(),
        })
        .collect();

    let mut result = 1;
    let mut power_of_2 = base; // 2 ^ 1
    while let Some(bit) = bits.pop() {
        if bit {
            result = modular_multiply(result, power_of_2, size);
        }
        power_of_2 = modular_multiply(power_of_2, power_of_2, size);
    }
    result
}

fn shuffle_deck(size: usize, input: Vec<&str>) -> FullDeck {
    let mut deck: Vec<_> = (0..size).into_iter().collect();
    let techniques = parse_shuffle_techniques(input);
    deck.shuffle_with(&techniques);
    deck
}

fn parse_shuffle_techniques(input: Vec<&str>) -> Vec<ShuffleTechnique> {
    input.into_iter().map(ShuffleTechnique::from).collect()
}

#[derive(Debug, Copy, Clone)]
enum ShuffleTechnique {
    DealIntoNewStack,
    CutNCards(isize),
    DealWithIncrement(usize),
}
impl ShuffleTechnique {
    fn invert(&self, size: usize) -> Self {
        match self {
            ShuffleTechnique::DealIntoNewStack => *self,
            ShuffleTechnique::CutNCards(n) => ShuffleTechnique::CutNCards(-*n),
            ShuffleTechnique::DealWithIncrement(i) => {
                // The correct inverted offset o is at index 1, where i * o % size == 1
                // For size 10: i * o could be 1, 11, 21, 31, 41, 51, 61, 71, 81, 91.
                // Such as i * o like 1 * 1 = 1, 7 * 3 or 3 * 7 = 21, 9 * 9 = 81

                // original     0 1 2 3 4 5 6 7 8 9
                // increment 3: 0 7 4 1 8 5 2 9 6 3
                //                ^

                // original     0 1 2 3 4 5 6 7 8 9
                // increment 7: 0 3 6 9 2 5 8 1 4 7
                //                ^

                // original     0 1 2 3 4 5 6 7 8 9
                // increment 9: 0 9 8 7 6 5 4 3 2 1
                //                ^

                // Apparently o is called the "modular inverse" of i.
                let o = modular_inverse(*i, size);
                ShuffleTechnique::DealWithIncrement(o)
            }
        }
    }
}
impl From<&str> for ShuffleTechnique {
    fn from(s: &str) -> Self {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        if parts[0] == "deal" {
            if parts[1] == "with" {
                ShuffleTechnique::DealWithIncrement(parts[3].parse().unwrap())
            } else {
                ShuffleTechnique::DealIntoNewStack
            }
        } else {
            ShuffleTechnique::CutNCards(parts[1].parse().unwrap())
        }
    }
}

trait Shuffle {
    fn shuffle_with(&mut self, techniques: &[ShuffleTechnique]) {
        for tech in techniques {
            self.apply(tech);
        }
    }
    fn apply(&mut self, technique: &ShuffleTechnique);
}

#[derive(Debug)]
struct SingleCardPosTrackingDeck {
    // The position to track
    pos: usize,
    // The size of the deck
    size: usize,
}
impl SingleCardPosTrackingDeck {
    fn new(pos: usize, size: usize) -> Self {
        SingleCardPosTrackingDeck { pos, size }
    }
}
impl Shuffle for SingleCardPosTrackingDeck {
    fn apply(&mut self, technique: &ShuffleTechnique) {
        match technique {
            ShuffleTechnique::DealIntoNewStack => self.pos = self.size - self.pos - 1,
            ShuffleTechnique::CutNCards(n) => {
                if n.is_positive() {
                    self.pos = (self.pos + self.size - *n as usize) % self.size
                } else if n.is_negative() {
                    self.pos = (self.pos + n.abs() as usize) % self.size
                }
            }
            ShuffleTechnique::DealWithIncrement(i) => {
                self.pos = modular_multiply(self.pos, *i, self.size);
            }
        }
    }
}

type FullDeck = Vec<usize>;
impl Shuffle for FullDeck {
    fn apply(&mut self, technique: &ShuffleTechnique) {
        match technique {
            ShuffleTechnique::DealIntoNewStack => self.reverse(),
            ShuffleTechnique::CutNCards(n) => {
                if n.is_positive() {
                    self.rotate_left(*n as usize);
                } else if n.is_negative() {
                    self.rotate_right(n.abs() as usize);
                }
            }
            ShuffleTechnique::DealWithIncrement(i) => {
                let old = self.clone();
                let mut curr = 0;
                for v in old {
                    self[curr] = v;
                    curr = (curr + i) % self.len();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn deal_into_new_stack() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        deck.apply(&ShuffleTechnique::DealIntoNewStack);
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], deck);
    }

    #[test]
    fn cut_3_cards() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        deck.apply(&ShuffleTechnique::CutNCards(3));
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], deck);
    }

    #[test]
    fn cut_minus_4_cards() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        deck.apply(&ShuffleTechnique::CutNCards(-4));
        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], deck);
    }

    #[test]
    fn deal_with_increment_3() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        deck.apply(&ShuffleTechnique::DealWithIncrement(3));
        assert_eq!(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], deck);
    }

    const EXAMPLE1: &str = "\
deal with increment 7
deal into new stack
deal into new stack";

    const EXAMPLE2: &str = "\
cut 6
deal with increment 7
deal into new stack";

    const EXAMPLE3: &str = "\
deal with increment 7
deal with increment 9
cut -2";

    const EXAMPLE4: &str = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

    #[test]
    fn example_1_full_deck() {
        let deck = shuffle_deck(10, parse(EXAMPLE1));
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], deck);
    }

    #[test]
    fn example_1_single_card_tracking_decks() {
        let decks = shuffle_10_single_card_tracking_decks(EXAMPLE1);
        verify_10_single_card_tracking_decks(decks, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn example_1_reverse_test() {
        test_reverse(EXAMPLE1);
    }
    #[test]
    fn example_2() {
        let deck = shuffle_deck(10, parse(EXAMPLE2));
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], deck);
    }

    #[test]
    fn example_2_single_card_tracking_decks() {
        let decks = shuffle_10_single_card_tracking_decks(EXAMPLE2);
        verify_10_single_card_tracking_decks(decks, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn example_2_reverse_test() {
        test_reverse(EXAMPLE2);
    }

    #[test]
    fn example_3() {
        let deck = shuffle_deck(10, parse(EXAMPLE3));
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], deck);
    }

    #[test]
    fn example_3_single_card_tracking_decks() {
        let decks = shuffle_10_single_card_tracking_decks(EXAMPLE3);
        verify_10_single_card_tracking_decks(decks, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn example_3_reverse_test() {
        test_reverse(EXAMPLE3);
    }

    #[test]
    fn example_4() {
        let deck = shuffle_deck(10, parse(EXAMPLE4));
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], deck);
    }

    #[test]
    fn example_4_single_card_tracking_decks() {
        let decks = shuffle_10_single_card_tracking_decks(EXAMPLE4);
        verify_10_single_card_tracking_decks(decks, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn example_4_reverse_test() {
        test_reverse(EXAMPLE4);
    }

    #[test]
    fn deal_into_new_stack_forward_and_reverse() {
        test_reverse("deal into new stack");
    }

    #[test]
    fn cut_3_cards_forward_and_reverse() {
        test_reverse("cut 3");
    }

    #[test]
    fn cut_minus_4_cards_forward_and_reverse() {
        test_reverse("cut -4");
    }

    #[test]
    fn deal_with_increment_1_reverse_test() {
        test_reverse("deal with increment 1");
    }
    #[test]
    fn deal_with_increment_3_reverse_test() {
        test_reverse("deal with increment 3");
    }
    #[test]
    fn deal_with_increment_7_reverse_test() {
        test_reverse("deal with increment 7");
    }
    #[test]
    fn deal_with_increment_9_reverse_test() {
        test_reverse("deal with increment 9");
    }

    fn test_reverse(input: &'static str) {
        let techniques = parse_shuffle_techniques(parse(input));
        // println!("tech {:?}", techniques);
        let mut decks = create_single_card_tracking_test_decks();
        shuffle(&mut decks, &techniques);
        // println!("forward: {:?}", decks);
        let expected_result = convert_to_deck_by_idx_as_value_at_pos_as_idx(decks);
        // println!("forward: {:?}", expected_result);

        let mut decks = create_single_card_tracking_test_decks();
        let inverted_techniques = invert(10, techniques);
        // println!("inverted tech {:?}", inverted_techniques);
        shuffle(&mut decks, &inverted_techniques);
        // println!("backward: {:?}", decks);
        let predicted_result = convert_to_deck_by_pos_as_value_at_idx(decks);
        // println!("backward: {:?}", predicted_result);

        assert_eq!(expected_result, predicted_result);
    }

    fn shuffle_10_single_card_tracking_decks(
        input: &'static str,
    ) -> Vec<SingleCardPosTrackingDeck> {
        let techniques = parse_shuffle_techniques(parse(input));
        let mut decks = create_single_card_tracking_test_decks();
        shuffle(&mut decks, &techniques);
        decks
    }

    fn shuffle(decks: &mut Vec<SingleCardPosTrackingDeck>, techniques: &[ShuffleTechnique]) {
        decks
            .iter_mut()
            .for_each(|deck| deck.shuffle_with(techniques));
    }

    fn initial_test_deck() -> Vec<usize> {
        (0..=9).into_iter().collect()
    }
    fn create_single_card_tracking_test_decks() -> Vec<SingleCardPosTrackingDeck> {
        create_10_single_card_tracking_decks_from(initial_test_deck())
    }

    fn create_10_single_card_tracking_decks_from(
        initial: Vec<usize>,
    ) -> Vec<SingleCardPosTrackingDeck> {
        initial
            .into_iter()
            .map(|i| SingleCardPosTrackingDeck::new(i, 10))
            .collect()
    }

    fn verify_10_single_card_tracking_decks(
        decks: Vec<SingleCardPosTrackingDeck>,
        expected: Vec<usize>,
    ) {
        let actual = convert_to_deck_by_idx_as_value_at_pos_as_idx(decks);
        assert_eq!(actual, expected);
    }

    fn convert_to_deck_by_idx_as_value_at_pos_as_idx(
        decks: Vec<SingleCardPosTrackingDeck>,
    ) -> Vec<usize> {
        let mut actual = vec![0; 10];
        decks
            .into_iter()
            .enumerate()
            .for_each(|(i, d)| actual[d.pos] = i);
        actual
    }
    fn convert_to_deck_by_pos_as_value_at_idx(decks: Vec<SingleCardPosTrackingDeck>) -> Vec<usize> {
        decks.into_iter().map(|d| d.pos).collect()
    }

    #[test]
    fn part1() {
        assert_eq!(2519, day22_part1());
    }

    #[test]
    fn part1_reverse() {
        let mut deck = SingleCardPosTrackingDeck::new(2519, PART1_DECK_SIZE);
        let input = parse(INPUT);
        let techniques = parse_shuffle_techniques(input);
        let inverted_techniques = invert(deck.size, techniques);
        deck.shuffle_with(&inverted_techniques);
        assert_eq!(2019, deck.pos);
    }

    #[test]
    fn part2() {
        assert_eq!(58966729050483, day22_part2());
    }

    #[test]
    fn test_modular_multiply() {
        assert_eq!(
            modular_multiply(53029207790359, 93748063761037, 119315717514047),
            46400556811471
        );
        // The following just fits within 64-bits:
        // u64::MAX = 3 * 5 * 17 * 257 * 641 * 65537 * 6700417
        assert_eq!(
            modular_multiply(usize::MAX / 6700417, 6700417, usize::MAX),
            0
        );
    }

    #[test]
    fn test_modular_inverse() {
        assert_eq!(modular_inverse(9, 10), 9);
    }

    #[test]
    fn test_determine_coefficients() {
        let initial = 2020;
        let size = PART2_DECK_SIZE;
        let results = shuffle_n_times(initial, size, 2);
        let (a, b) = determine_coefficients(initial, results[1], results[2], size);

        assert_eq!(results[1], (modular_multiply(a, initial, size) + b) % size);
        assert_eq!(
            results[2],
            (modular_multiply(a, results[1], size) + b) % size
        );
    }

    #[test]
    fn test_modular_power() {
        let base = 12854400258724;
        let size = PART2_DECK_SIZE;

        let mut res = base;
        assert_eq!(res, modular_power(base, 1, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 2, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 3, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 4, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 5, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 6, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 7, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 8, size));

        res = modular_multiply(res, base, size);
        assert_eq!(res, modular_power(base, 9, size));
    }
    #[test]
    fn test_apply_linear_function_n_times() {
        let init = 2020;
        let size = PART2_DECK_SIZE;

        let res = shuffle_n_times(init, size, 5);
        let (a, b) = determine_coefficients(init, res[1], res[2], size);

        apply_linear_function_n_times(init, size, a, b, 1);

        assert_eq!(res[1], apply_linear_function_n_times(init, size, a, b, 1));
        assert_eq!(res[2], apply_linear_function_n_times(init, size, a, b, 2));
        assert_eq!(res[3], apply_linear_function_n_times(init, size, a, b, 3));
        assert_eq!(res[4], apply_linear_function_n_times(init, size, a, b, 4));
        assert_eq!(res[5], apply_linear_function_n_times(init, size, a, b, 5));
    }
}
