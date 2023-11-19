use crate::parse;
use std::ops::{Add, AddAssign};
use Type::*;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn day21_part1() -> usize {
    // Minimum cost of equipment that lets the player win
    all_item_combinations(
        &parse_items(WEAPONS),
        &parse_items(ARMOR),
        &parse_items(RINGS),
    )
    .into_iter()
    .filter_map(|items| item_cost_of_fight_with_winner(&items, Player))
    .min()
    .unwrap()
}

pub(crate) fn day21_part2() -> usize {
    // Maximum cost of equipment that still has the player lose
    all_item_combinations(
        &parse_items(WEAPONS),
        &parse_items(ARMOR),
        &parse_items(RINGS),
    )
    .into_iter()
    .filter_map(|items| item_cost_of_fight_with_winner(&items, Boss))
    .max()
    .unwrap()
}

fn all_item_combinations<'a>(
    weapons: &'a [Item],
    armors: &'a [Item],
    rings: &'a [Item],
) -> Vec<Vec<&'a Item>> {
    let mut combinations = vec![];

    // Mandatory weapon
    for weapon in weapons.iter() {
        let items = vec![weapon];
        combinations.push(items.clone());

        // Optional armor
        for armor in armors.iter() {
            let mut items = items.clone();
            items.push(armor);
            combinations.push(items.clone());

            // Optional rings with armor
            add_rings(&mut combinations, rings, &items)
        }

        // Optional rings without armor
        add_rings(&mut combinations, rings, &items);
    }
    combinations
}

fn add_rings<'a>(
    item_combinations: &mut Vec<Vec<&'a Item>>,
    rings: &'a [Item],
    items: &[&'a Item],
) {
    for ring1 in rings.iter() {
        let mut items = items.to_owned();
        items.push(ring1);
        item_combinations.push(items.clone());

        for ring2 in rings.iter() {
            if ring1 != ring2 {
                let mut items = items.clone();
                items.push(ring2);
                item_combinations.push(items);
            }
        }
    }
}

fn item_cost_of_fight_with_winner(items: &[&Item], winner: Type) -> Option<Cost> {
    let mut player = Character::default();
    player.add_items(items);
    if fight(&mut player, &mut boss_from_input()).kind == winner {
        Some(total_cost_of(items))
    } else {
        None
    }
}

fn total_cost_of(items: &[&Item]) -> usize {
    items.iter().map(|item| item.cost).sum()
}

fn parse_items(items: &'static str) -> Vec<Item> {
    parse(items)
        .into_iter()
        .skip(1)
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            Item::new(
                parts[parts.len() - 3].parse().unwrap(),
                parts[parts.len() - 2].parse().unwrap(),
                parts[parts.len() - 1].parse().unwrap(),
            )
        })
        .collect()
}

type Hitpoints = usize;
type Cost = usize;
type Damage = usize;
type Armor = usize;

const WEAPONS: &str = "\
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0";

const ARMOR: &str = "\
Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5";

const RINGS: &str = "\
Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

fn boss_from_input() -> Character {
    let input = parse(INPUT);
    Character::from(input)
}

fn fight<'a>(player: &'a mut Character, boss: &'a mut Character) -> &'a Character {
    loop {
        player.attacks(boss);
        if boss.is_dead() {
            // println!("Player wins with {} HP left", player.hp);
            return player;
        }
        boss.attacks(player);
        if player.is_dead() {
            // println!("Boss wins with {} HP left", boss.hp);
            return boss;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    Player,
    Boss,
}

#[derive(Debug, PartialEq, Clone)]
struct Item {
    cost: Cost,
    stats: Stats,
}
impl Item {
    fn new(cost: Cost, damage: Damage, armor: Armor) -> Self {
        Item {
            cost,
            stats: Stats::new(damage, armor),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Character {
    hp: Hitpoints,
    kind: Type,
    stats: Stats,
}
impl Default for Character {
    fn default() -> Self {
        Character {
            hp: 100,
            kind: Player,
            stats: Stats::default(),
        }
    }
}
impl From<Vec<&str>> for Character {
    fn from(s: Vec<&str>) -> Self {
        let extract_number = |line: &str| {
            line.split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        };

        Character {
            hp: extract_number(&s[0]),
            kind: Boss,
            stats: Stats::new(extract_number(&s[1]), extract_number(&s[2])),
        }
    }
}
impl Character {
    #[cfg(test)]
    fn player(hp: Hitpoints, damage: Damage, armor: Armor) -> Self {
        Character {
            hp,
            kind: Player,
            stats: Stats::new(damage, armor),
        }
    }
    #[cfg(test)]
    fn boss(hp: Hitpoints, damage: Damage, armor: Armor) -> Self {
        Character {
            hp,
            kind: Boss,
            stats: Stats::new(damage, armor),
        }
    }
    fn is_dead(&self) -> bool {
        self.hp == 0
    }
    fn attacks(&self, other: &mut Character) {
        let damage_dealt = usize::max(1, self.stats.damage.saturating_sub(other.stats.armor));
        other.hp = other.hp.saturating_sub(damage_dealt);
    }
    fn add_items(&mut self, items: &[&Item]) {
        items.iter().for_each(|item| {
            self.stats += item.stats;
        });
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Stats {
    damage: Damage,
    armor: Armor,
}
impl Default for Stats {
    fn default() -> Self {
        Stats {
            damage: 0,
            armor: 0,
        }
    }
}
impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.damage += rhs.damage;
        self.armor += rhs.armor;
    }
}
impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Stats::new(self.damage + rhs.damage, self.armor + rhs.armor)
    }
}
impl Stats {
    fn new(damage: usize, armor: usize) -> Self {
        Stats { damage, armor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let mut player = Character::player(8, 5, 5);
        let mut boss = Character::boss(12, 7, 2);
        let winner = fight(&mut player, &mut boss);
        assert_eq!(Player, winner.kind);
        assert_eq!(2, winner.hp);
    }

    #[test]
    fn part1() {
        assert_eq!(91, day21_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(158, day21_part2());
    }
}
