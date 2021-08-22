use std::cmp::Ordering;

type GroupIdx = usize;
type ArmyName = String;

#[derive(Debug, PartialEq, Clone)]
enum DamageType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}
impl From<&str> for DamageType {
    fn from(s: &str) -> Self {
        match s {
            "bludgeoning" => DamageType::Bludgeoning,
            "cold" => DamageType::Cold,
            "fire" => DamageType::Fire,
            "radiation" => DamageType::Radiation,
            "slashing" => DamageType::Slashing,
            _ => panic!("Unknown damage type '{}'", s),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Group {
    count: usize,
    hp: usize,
    immunities: Vec<DamageType>,
    weaknesses: Vec<DamageType>,
    attack_damage: usize,
    attack_type: DamageType,
    initiative: usize,
}

impl<T: AsRef<str>> From<T> for Group {
    fn from(s: T) -> Self {
        // 89 units each with
        let (count_str, rest) = s.as_ref().split_once(" units each with ").unwrap();
        let count = count_str.parse().unwrap();
        // 11269 hit points (
        let (hp_str, rest_str) = rest.split_once(" hit points").unwrap();
        let hp = hp_str.parse().unwrap();
        // (immune to fire, bludgeoning; weak to fire, radiation)
        let (immunities, weaknesses, rest) = Group::parse_lists(rest_str);
        // with an attack that does 1018 slashing damage at initiative 7
        let rest = rest
            .trim_start_matches(" with an attack that does ")
            .split(' ')
            .collect::<Vec<_>>();
        let attack_damage = rest[0].parse().unwrap();
        let attack_type = DamageType::from(rest[1]);
        let initiative = rest[5].parse().unwrap();
        Group {
            count,
            hp,
            immunities,
            weaknesses,
            attack_damage,
            attack_type,
            initiative,
        }
    }
}

impl Group {
    fn parse_lists(mut rest: &str) -> (Vec<DamageType>, Vec<DamageType>, &str) {
        let mut immunity_list = "";
        let mut weakness_list = "";
        if let Some((lists, after)) = rest.split_once(")") {
            rest = after;
            let lists = lists.trim_start_matches(" (");
            if let Some((list1, list2)) = lists.split_once("; ") {
                if list1.starts_with("immune to ") {
                    immunity_list = list1;
                    weakness_list = list2;
                } else {
                    assert!(list2.starts_with("immune to "));
                    weakness_list = list1;
                    immunity_list = list2;
                }
            } else if lists.starts_with("immune to ") {
                immunity_list = lists;
            } else {
                assert!(lists.starts_with("weak to "));
                weakness_list = lists;
            }
        }
        let immunities = Group::parse_list(immunity_list, "immune to ");
        let weaknesses = Group::parse_list(weakness_list, "weak to ");
        (immunities, weaknesses, rest)
    }
    fn parse_list(list: &str, prefix: &str) -> Vec<DamageType> {
        if list.trim_start_matches(prefix).is_empty() {
            vec![]
        } else {
            list.trim_start_matches(prefix)
                .split(", ")
                .map(DamageType::from)
                .collect()
        }
    }
    fn damage_to(&self, other: &Group) -> usize {
        self.effective_power()
            * if other.immunities.contains(&self.attack_type) {
                0
            } else if other.weaknesses.contains(&self.attack_type) {
                2
            } else {
                1
            }
    }
}

trait EffectivePower {
    fn effective_power(&self) -> usize;
}

impl EffectivePower for Group {
    fn effective_power(&self) -> usize {
        self.count * self.attack_damage
    }
}

#[derive(Clone)]
struct Army {
    name: ArmyName,
    groups: Vec<Group>,
}
impl From<&[String]> for Army {
    fn from(input: &[String]) -> Self {
        let name = input[0].trim_end_matches(':').to_string();
        let groups = input.iter().skip(1).map(Group::from).collect();
        Army { name, groups }
    }
}
impl Army {
    fn unit_count(&self) -> usize {
        self.groups.iter().map(|g| g.count).sum()
    }
    fn print_summary(&self) {
        println!("{}:", self.name);
        self.groups
            .iter()
            .enumerate()
            .filter(|(_, g)| g.count > 0)
            .for_each(|(i, g)| println!("Group {} contains {} units", i + 1, g.count));
    }
    #[allow(unused)]
    fn apply_damage_boost(&mut self, damage_boost: usize) {
        self.groups.iter_mut().for_each(|g| {
            g.attack_damage += damage_boost;
        });
    }
}

#[derive(Clone, PartialEq, Debug)]
struct GroupId {
    name: ArmyName,
    idx: GroupIdx,
}
impl GroupId {
    fn new(idx: GroupIdx, name: &str) -> Self {
        GroupId {
            idx,
            name: name.to_string(),
        }
    }
    fn find_and_remove_target(
        &self,
        targets: &mut Vec<GroupId>,
        army1: &Army,
        army2: &Army,
    ) -> Option<GroupId> {
        let mut target_ids: Vec<&GroupId> = targets
            .iter()
            .filter(|group| !self.name.eq(&group.name))
            .collect();
        if target_ids.is_empty() {
            return None;
        }
        let own = group_of(self, army1, army2);
        target_ids.sort_unstable_by(|a, b| {
            let a = group_of(a, army1, army2);
            let b = group_of(b, army1, army2);
            match own.damage_to(a).partial_cmp(&own.damage_to(b)) {
                Some(Ordering::Equal) => {
                    match a.effective_power().partial_cmp(&b.effective_power()) {
                        Some(Ordering::Equal) => a.initiative.partial_cmp(&b.initiative),
                        power => power,
                    }
                }
                damage => damage,
            }
            .unwrap()
        });
        // println!("Self: {:?}", own);
        // target_ids.iter().for_each(|id| {
        //     let other = get_group(id, army1, army2);
        //     println!(
        //         "{}. target {:?} with EP {} in army {} would take damage {}",
        //         id.idx,
        //         other,
        //         other.effective_power(),
        //         id.name,
        //         own.damage_to(other),
        //     )
        // });
        let target = target_ids.pop().cloned().unwrap();
        if own.damage_to(group_of(&target, army1, army2)) == 0 {
            // If it cannot deal any defending groups damage, it does not choose a target.
            return None;
        }
        targets.remove(targets.iter().position(|t| t == &target).unwrap());
        Some(target)
    }
}

pub(crate) fn fight_until_one_army_left(lines: Vec<String>) -> usize {
    let (army1, army2) = parse_input(lines);
    fight_armies_until_only_one_left(army1, army2).unit_count()
}

fn parse_input(lines: Vec<String>) -> (Army, Army) {
    let (army1, army2) = lines.split_at(lines.iter().position(|s| s.is_empty()).unwrap());
    let (army1, army2) = (Army::from(army1), Army::from(&army2[1..]));
    (army1, army2)
}

fn fight_armies_until_only_one_left(mut army1: Army, mut army2: Army) -> Army {
    while army1.unit_count() > 0 && army2.unit_count() > 0 {
        println!("\n--------------------------\n");
        army1.print_summary();
        army2.print_summary();

        // Target selection phase
        let mut fights = arrange_fights(&mut army1, &mut army2);

        print_fight_plans(&mut fights, &mut army1, &mut army2);

        for (attacker, defender) in fights {
            let (attacking_group, defending_group) =
                mut_group_of(&attacker, &defender, &mut army1, &mut army2);
            if attacking_group.count == 0 {
                println!("Skipping attacking group {:?}", attacker);
            }
            let damage = attacking_group.damage_to(defending_group);
            let killed = usize::min(defending_group.count, damage / defending_group.hp);
            defending_group.count -= killed;
            println!(
                "{} group {} attacks defending group {}, killing {} units",
                attacker.name,
                attacker.idx + 1,
                defender.idx + 1,
                killed
            );
        }
    }
    if army1.unit_count() > 0 {
        army1.print_summary();
        army1
    } else {
        army2.print_summary();
        army2
    }
}

fn arrange_fights(army1: &mut Army, army2: &mut Army) -> Vec<(GroupId, GroupId)> {
    let mut attackers = order_attackers(army1, army2);
    let mut targets = attackers.clone();

    let mut fights = vec![];
    while let Some(attacker) = attackers.pop() {
        if let Some(target) = attacker.find_and_remove_target(&mut targets, army1, army2) {
            fights.push((attacker, target));
        } else {
            println!("did not find a target.")
        }
    }
    fights.sort_unstable_by(|a, b| {
        group_of(&a.0, army1, army2)
            .initiative
            .partial_cmp(&group_of(&b.0, army1, army2).initiative)
            .unwrap()
            .reverse()
    });
    fights
}

fn print_fight_plans(fights: &mut Vec<(GroupId, GroupId)>, army1: &mut Army, army2: &mut Army) {
    println!();
    fights.iter().for_each(|(a, d)| {
        let ag = group_of(a, army1, army2);
        let dg = group_of(d, army1, army2);
        let damage = ag.damage_to(dg);
        println!(
            "{} group {} would deal defending group {} {} damage",
            a.name,
            a.idx + 1,
            d.idx + 1,
            damage
        );
    });
    println!();
}

/// Return a Vec of (groupId, armyName), where the last should choose their target first
fn order_attackers<'a>(army1: &'a Army, army2: &'a Army) -> Vec<GroupId> {
    let mut attackers: Vec<GroupId> = army1
        .groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.count > 0)
        .map(|(idx, _)| GroupId::new(idx, &army1.name))
        .chain(
            army2
                .groups
                .iter()
                .enumerate()
                .filter(|(_, g)| g.count > 0)
                .map(|(idx, _)| GroupId::new(idx, &army2.name)),
        )
        .collect();
    attackers.sort_unstable_by(|a, b| {
        let a = group_of(a, army1, army2);
        let b = group_of(b, army1, army2);
        match a.effective_power().partial_cmp(&b.effective_power()) {
            Some(Ordering::Equal) => a.initiative.partial_cmp(&b.initiative),
            power_order => power_order,
        }
        .unwrap()
    });
    attackers
}
fn group_of<'a, 'b>(id: &'b GroupId, army1: &'a Army, army2: &'a Army) -> &'a Group {
    if id.name.eq(&army1.name) {
        &army1.groups[id.idx]
    } else {
        &army2.groups[id.idx]
    }
}
fn mut_group_of<'a, 'b>(
    id_a: &'b GroupId,
    id_b: &'b GroupId,
    army1: &'a mut Army,
    army2: &'a mut Army,
) -> (&'a mut Group, &'a mut Group) {
    if id_a.name.eq(&army1.name) {
        (
            army1.groups.get_mut(id_a.idx).unwrap(),
            army2.groups.get_mut(id_b.idx).unwrap(),
        )
    } else {
        (
            army2.groups.get_mut(id_a.idx).unwrap(),
            army1.groups.get_mut(id_b.idx).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn parse_group() {
        let group = Group::from("89 units each with 11269 hit points (weak to fire, radiation) with an attack that does 1018 slashing damage at initiative 7");
        assert_eq!(
            group,
            Group {
                count: 89,
                hp: 11269,
                immunities: vec![],
                weaknesses: vec![DamageType::Fire, DamageType::Radiation],
                attack_damage: 1018,
                attack_type: DamageType::Slashing,
                initiative: 7,
            }
        );
    }

    const EXAMPLE: &str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn part1_example() {
        let lines = read_str_to_lines(EXAMPLE);
        let unit_count_of_winning_army = fight_until_one_army_left(lines);
        assert_eq!(782 + 4434, unit_count_of_winning_army);
    }

    #[test]
    fn part1_input() {
        let lines = read_file_to_lines("input/day24.txt");
        let unit_count_of_winning_army = fight_until_one_army_left(lines);
        assert_eq!(
            3186 + 1252 + 2241 + 2590 + 1650 + 7766 + 1790 + 264 + 2257, // 22996
            unit_count_of_winning_army
        );
    }
}
