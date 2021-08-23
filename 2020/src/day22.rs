use std::collections::{HashSet, VecDeque};

type Card = usize;
type Deck = VecDeque<usize>;

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

pub fn winning_recursive_combat_players_score(input: &[String]) -> usize {
    let (deck1, deck2) = decks_from_input(input);
    // println!("player 1 deck {:?}\nplayer 2 deck {:?}", deck1, deck2);
    let (score1, score2) = play_recursive_combat(deck1, deck2);
    usize::max(score1, score2)
}

fn decks_from_input(input: &[String]) -> (Deck, Deck) {
    let mut player_decks = input.split(|line| line.is_empty());
    let deck1 = deck_from(player_decks.next().unwrap());
    let deck2 = deck_from(player_decks.next().unwrap());
    (deck1, deck2)
}

fn play_recursive_combat(mut d1: Deck, mut d2: Deck) -> (usize, usize) {
    let mut played: HashSet<Deck> = HashSet::new();
    while !d1.is_empty() && !d2.is_empty() {
        if played.contains(&d1) || played.contains(&d2) {
            // In this case player 1 wins
            return (score(&d1), 0);
        } else {
            played.insert(d1.clone());
            played.insert(d2.clone());
        }
        let card1 = d1.pop_front().unwrap();
        let card2 = d2.pop_front().unwrap();
        if d1.len() >= card1 && d2.len() >= card2 {
            let rec_d1 = VecDeque::from(d1.iter().take(card1).cloned().collect::<Vec<_>>());
            let rec_d2 = VecDeque::from(d2.iter().take(card2).cloned().collect::<Vec<_>>());
            let (score1, score2) = play_recursive_combat(rec_d1, rec_d2);
            add_tops_to_winning_deck(&mut d1, &mut d2, card1, card2, score1 > score2);
        } else {
            add_tops_to_winning_deck(&mut d1, &mut d2, card1, card2, card1 > card2);
        }
    }
    (score(&d1), score(&d2))
}

fn add_tops_to_winning_deck(d1: &mut Deck, d2: &mut Deck, c1: Card, c2: Card, p1_wins: bool) {
    if p1_wins {
        d1.push_back(c1);
        d1.push_back(c2);
    } else {
        d2.push_back(c2);
        d2.push_back(c1);
    }
}

pub(crate) fn winning_players_score(input: &[String]) -> usize {
    let mut player_decks = input.split(|line| line.is_empty());
    let mut deck1 = deck_from(player_decks.next().unwrap());
    let mut deck2 = deck_from(player_decks.next().unwrap());
    // println!("player 1 deck {:?}\nplayer 2 deck {:?}", deck1, deck2);
    while !deck1.is_empty() && !deck2.is_empty() {
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();
        if top1 > top2 {
            deck1.push_back(top1);
            deck1.push_back(top2);
        } else {
            deck2.push_back(top2);
            deck2.push_back(top1);
        }
        // println!("player 1 deck {:?}\nplayer 2 deck {:?}", deck1, deck2);
    }
    if deck1.is_empty() {
        score(&deck2)
    } else {
        score(&deck1)
    }
}

fn deck_from(players: &[String]) -> Deck {
    players.iter().skip(1).map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::*;

    const EXAMPLE_1: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn score_deck() {
        let deck: Deck = VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(score(&deck), 306);
    }

    #[test]
    fn score_deck2() {
        let deck: Deck = VecDeque::from(vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
        assert_eq!(score(&deck), 291);
    }

    #[test]
    fn part_1_example_1() {
        assert_eq!(winning_players_score(&read_str_to_lines(EXAMPLE_1)), 306);
    }

    #[test]
    fn part_1() {
        assert_eq!(
            winning_players_score(&read_file_to_lines("input/day22.txt")),
            35299
        );
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(
            winning_recursive_combat_players_score(&read_str_to_lines(EXAMPLE_1)),
            291
        );
    }

    const RECURSIVE_EXAMPLE: &str = "Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn part_2_recursive_example() {
        assert_eq!(
            winning_recursive_combat_players_score(&read_str_to_lines(RECURSIVE_EXAMPLE)),
            105
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            winning_recursive_combat_players_score(&read_file_to_lines("input/day22.txt")),
            33266
        );
    }
}
