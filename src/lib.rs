use std::collections::VecDeque;

#[cfg(test)]
mod tests;

type Deck = VecDeque<usize>;

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

pub fn winning_players_score(input: &[String]) -> usize {
    let mut player_decks = input.split(|line| line.is_empty());
    let mut deck1 = deck_from(player_decks.next().unwrap());
    let mut deck2 = deck_from(player_decks.next().unwrap());
    println!("player 1 deck {:?}\nplayer 2 deck {:?}", deck1, deck2);
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
