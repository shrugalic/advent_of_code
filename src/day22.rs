use crate::parse;
use Spell::*;
use Type::*;

const INPUT: &str = include_str!("../input/day22.txt");

pub(crate) fn day22_part1() -> usize {
    minimum_mana_cost_player_winning_fight(false)
}

pub(crate) fn day22_part2() -> usize {
    minimum_mana_cost_player_winning_fight(true)
}

fn minimum_mana_cost_player_winning_fight(hard_mode: bool) -> Mana {
    let mut min_cost = Mana::MAX;
    let spells: [Spell; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];
    let mut fights: Vec<_> = spells
        .iter()
        .map(|spell| (Fight::new(hard_mode), spell))
        .collect();
    while let Some((mut fight, spell)) = fights.pop() {
        if let Some(victor) = fight.one_round(spell) {
            if victor == Player {
                min_cost = usize::min(min_cost, fight.cost);
            }
        } else if fight.cost < min_cost {
            spells
                .iter()
                .filter(|&spell| spell.can_be_used_in(&fight))
                .for_each(|spell| fights.push((fight.clone(), spell)));
        }
    }

    min_cost
}

#[derive(Debug, Clone)]
struct Fight {
    cost: Mana,
    player: Character,
    boss: Character,
    hard_mode: bool,
}
impl Fight {
    fn new(hard_mode: bool) -> Self {
        Fight {
            cost: 0,
            player: Character::default(),
            boss: boss_from_input(),
            hard_mode,
        }
    }
    #[cfg(test)]
    fn until_a_character_dies(mut self, mut spells: Vec<Spell>) -> (Character, Mana) {
        while !spells.is_empty() {
            let spell = spells.remove(0);
            if let Some(victor) = self.one_round(&spell) {
                return match victor {
                    Player => (self.player, self.cost),
                    Boss => (self.player, self.cost),
                };
            }
        }
        // This is only called during testing, where we don't run out of spells
        unreachable!()
    }
    fn one_round(&mut self, spell: &Spell) -> Option<Type> {
        self.execute_players_turn(spell);
        if let Some(victor) = self.victor() {
            return Some(victor);
        }
        self.execute_bosses_turn();
        self.victor()
    }

    fn execute_players_turn(&mut self, spell: &Spell) {
        if self.hard_mode {
            self.player.hp -= 1;
            if self.player.is_dead() {
                return;
            }
        }
        self.player.apply_effects();
        self.boss.apply_effects();
        if self.boss.is_dead() {
            return;
        }
        self.player.cast_spell(&mut self.boss, spell);
        self.cost += spell.spell_cost();
    }

    fn execute_bosses_turn(&mut self) {
        self.player.apply_effects();
        self.boss.apply_effects();
        if self.boss.is_dead() {
            return;
        }
        self.boss.attack(&mut self.player);
    }
    fn victor(&self) -> Option<Type> {
        if self.boss.is_dead() {
            Some(Player)
        } else if self.player.is_dead() {
            Some(Boss)
        } else {
            None
        }
    }
}

type HitPoints = usize;
type Mana = usize;
type Damage = usize;
type Time = usize;

fn boss_from_input() -> Character {
    let input = parse(INPUT);
    Character::from(input)
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Player,
    Boss,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spell {
    MagicMissile, // instantly deal 4 damage
    Drain,        // instantly deal 2 damage to boss and heal player hp by 2
    Shield,       // effect for 6 turns: +7 armor while active
    Poison,       // effect for 6 turns: deal 3 damage to boss
    Recharge,     // effect for 5 turns: +101 new mana on each turn
}
impl Spell {
    fn spell_cost(&self) -> Mana {
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }
    fn can_be_used_in(&self, fight: &Fight) -> bool {
        fight.player.mana > self.spell_cost()
            && match self {
                MagicMissile | Drain => true, // Don't have any effect and can always be used
                Shield | Recharge => !fight
                    .player
                    .active_effects
                    .iter()
                    .any(|effect| &effect.spell == self && effect.time_left > 1),
                Poison => !fight
                    .boss
                    .active_effects
                    .iter()
                    .any(|effect| &effect.spell == self && effect.time_left > 1),
            }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Effect {
    spell: Spell,
    time_left: Time,
}
impl From<Spell> for Effect {
    fn from(spell: Spell) -> Self {
        match spell {
            Shield | Poison => Effect::new(spell, 6),
            Recharge => Effect::new(spell, 5),
            MagicMissile | Drain => unreachable!(),
        }
    }
}
impl Effect {
    fn new(spell: Spell, time_left: Time) -> Self {
        Effect { spell, time_left }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Character {
    hp: HitPoints,
    kind: Type,
    mana: Mana,
    damage: Damage,
    active_effects: Vec<Effect>,
}
impl Default for Character {
    fn default() -> Self {
        Character {
            hp: 50,
            kind: Player,
            mana: 500,
            damage: 0,
            active_effects: vec![],
        }
    }
}
impl From<Vec<&str>> for Character {
    fn from(lines: Vec<&str>) -> Self {
        let extract_number = |line: &str| {
            line.split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        };
        Character {
            hp: extract_number(&lines[0]),
            kind: Boss,
            mana: 500,
            damage: extract_number(&lines[1]),
            active_effects: vec![],
        }
    }
}
impl Character {
    #[cfg(test)]
    fn player(hp: HitPoints, mana: Mana) -> Self {
        Character {
            hp,
            kind: Player,
            mana,
            damage: 0,
            active_effects: vec![],
        }
    }
    #[cfg(test)]
    fn boss(hp: HitPoints, damage: Damage) -> Self {
        Character {
            hp,
            kind: Boss,
            mana: 250,
            damage,
            active_effects: vec![],
        }
    }
    fn is_dead(&self) -> bool {
        self.hp == 0
    }
    fn apply_effects(&mut self) {
        let effects: Vec<_> = self.active_effects.drain(..).collect();
        for mut effect in effects {
            match effect.spell {
                Poison => self.take(3),
                Recharge => self.mana += 101,
                Shield => {} // Shield is considered by the armor() calculation in take(damage)
                Drain | MagicMissile => unreachable!(),
            }
            effect.time_left -= 1;
            if effect.time_left > 0 {
                self.active_effects.push(effect);
            }
        }
    }
    fn cast_spell(&mut self, boss: &mut Character, spell: &Spell) {
        assert_eq!(self.kind, Player);
        self.mana -= spell.spell_cost();
        // println!("Players casts {:?}", spell);
        match spell {
            MagicMissile => boss.take(4),
            Drain => {
                boss.take(2);
                self.hp += 2;
            }
            Shield | Recharge => {
                if !self.active_effects.iter().any(|e| &e.spell == spell) {
                    self.active_effects.push(Effect::from(*spell));
                }
            }
            Poison => {
                if !boss.active_effects.iter().any(|e| &e.spell == spell) {
                    boss.active_effects.push(Effect::from(*spell));
                }
            }
        }
    }
    fn attack(&mut self, other: &mut Character) {
        assert_eq!(self.kind, Boss);
        other.take(self.damage);
    }
    fn take(&mut self, damage: usize) {
        let damage_dealt = usize::max(1, damage.saturating_sub(self.armor()));
        self.hp = self.hp.saturating_sub(damage_dealt);
        // println!(
        //     "{:?} took {} damage, {} HP left",
        //     self.kind, damage_dealt, self.hp
        // );
    }
    fn armor(&self) -> usize {
        self.active_effects
            .iter()
            .filter(|e| e.spell == Spell::Shield)
            .map(|_| 7)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let fight = Fight {
            cost: 0,
            player: Character::player(10, 250),
            boss: Character::boss(13, 8),
            hard_mode: false,
        };
        let spells = vec![Poison, MagicMissile];
        let (winner, _) = fight.until_a_character_dies(spells);
        assert_eq!(Player, winner.kind);
        assert_eq!(2, winner.hp);
        assert_eq!(24, winner.mana);
    }

    #[test]
    fn part1_example2() {
        let fight = Fight {
            cost: 0,
            player: Character::player(10, 250),
            boss: Character::boss(14, 8),
            hard_mode: false,
        };
        let spells = vec![Recharge, Shield, Drain, Poison, MagicMissile];
        let (winner, _) = fight.until_a_character_dies(spells);
        assert_eq!(Player, winner.kind);
        assert_eq!(1, winner.hp);
        assert_eq!(114, winner.mana);
    }

    #[test]
    fn part1() {
        assert_eq!(953, day22_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1289, day22_part2());
    }
}
