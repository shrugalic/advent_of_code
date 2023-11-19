use crate::parse;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display, Formatter};

const INPUT: &str = include_str!("../input/day18.txt");

pub(crate) fn day18_part1() -> usize {
    count_steps_to_collect_every_key(parse(INPUT))
}

pub(crate) fn day18_part2() -> usize {
    count_steps_to_collect_every_key_part2(parse(INPUT))
}

type Idx = u8;
type Key = char;
// Anything that implements KeySet can be used,
// such as BoolArrayKeys, U32Keys, Vec<Key> or String
// Run times for:   (example_4, part1)
// String:          (~1s,       ~25s)
// U32Keys          (<4s,       ~94s)
// BoolArrayKeys:   (>4s,       ~2 min)
// Vec<Key>         way too slow
// HashSet<char> can't be used, because it doesn't implement Hash
type Keys = String;
type Steps = usize;

trait KeySet {
    fn add(&mut self, key: Key);
    fn contains(&self, key: &Key) -> bool;
    fn count(&self) -> usize;
}

impl KeySet for String {
    fn add(&mut self, key: Key) {
        if !self.contains(&key) {
            let mut chars = self.chars().collect::<Vec<_>>();
            chars.push(key);
            chars.sort_unstable();
            *self = chars.iter().collect::<String>();
        }
    }
    fn contains(&self, key: &Key) -> bool {
        self.chars().any(|c| &c == key)
    }
    fn count(&self) -> usize {
        self.len()
    }
}

impl KeySet for Vec<Key> {
    fn add(&mut self, key: Key) {
        if !self.contains(&key) {
            self.push(key);
        }
    }
    fn contains(&self, key: &Key) -> bool {
        self.iter().any(|k| k == key)
    }
    fn count(&self) -> usize {
        self.len()
    }
}

trait IndexedKeys {
    fn add(&mut self, key: Key);
    fn contains(&self, key: &Key) -> bool {
        self.is_set(key.to_idx())
    }
    fn count(&self) -> usize {
        (0..26u8).into_iter().filter(|&i| self.is_set(i)).count()
    }
    fn is_set(&self, idx: u8) -> bool;
}

trait KeyToIdx {
    fn to_idx(&self) -> Idx;
}
impl KeyToIdx for Key {
    fn to_idx(&self) -> Idx {
        (*self as Idx) - 97
    }
}

trait IdxToKey {
    fn to_key(&self) -> char;
}
impl IdxToKey for Idx {
    fn to_key(&self) -> Key {
        (self + 97) as Key
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct U32Keys {
    keys: u32, // 1 bit for each of the max 26 keys
}

impl IndexedKeys for U32Keys {
    fn add(&mut self, key: Key) {
        self.keys |= U32Keys::mask_for(&key);
    }
    fn is_set(&self, idx: u8) -> bool {
        1 == 1 & self.keys >> idx
    }
}
impl U32Keys {
    fn mask_for(key: &Key) -> u32 {
        1 << key.to_idx()
    }
}
impl Default for U32Keys {
    fn default() -> Self {
        U32Keys { keys: 0 }
    }
}
impl Debug for U32Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_for_keys(self, f)
    }
}
impl Display for U32Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_for_keys(self, f)
    }
}
fn fmt_for_keys(keys: &dyn IndexedKeys, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
        f,
        "{{{}}}",
        (0..26u8)
            .into_iter()
            .filter(|&i| keys.is_set(i))
            .map(|i| i.to_key())
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct BoolArrayKeys {
    keys: [bool; 26], // 1 bit for each of the max 26 keys
}
impl IndexedKeys for BoolArrayKeys {
    fn add(&mut self, key: Key) {
        self.keys[key.to_idx() as usize] = true;
    }
    fn is_set(&self, idx: u8) -> bool {
        self.keys[idx as usize]
    }
}
impl Default for BoolArrayKeys {
    fn default() -> Self {
        BoolArrayKeys { keys: [false; 26] }
    }
}
impl Debug for BoolArrayKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_for_keys(self, f)
    }
}
impl Display for BoolArrayKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_for_keys(self, f)
    }
}

// This can be used to solve part 1 – it's faster than the alternative below
fn count_steps_to_collect_every_key(input: Vec<&str>) -> Steps {
    let vault = Vault::from(input);
    // println!("Vault:\n{}", vault.to_string());

    let (_keys, steps) = explore(&vault, vault.entrance());
    steps
}

fn explore(vault: &Vault, start: Loc) -> (Keys, Steps) {
    // Keeps track of visited locations with steps-traveled by key-set.
    let mut visited: Vec<Vec<HashMap<Keys, Steps>>> =
        vec![vec![HashMap::new(); vault.width()]; vault.height()];

    let mut fewest_steps_by_keys: HashMap<Keys, Steps> = HashMap::new();
    let mut explorers = BinaryHeap::new();
    explorers.push(Explorer::initial(start));
    while let Some(mut explorer) = explorers.pop() {
        if let Some(Tile::Key(key)) = vault.tile_at(&explorer.loc) {
            explorer.add_key(*key);
        }
        let local_steps = visited[explorer.loc.y][explorer.loc.x]
            .entry(explorer.keys())
            .or_insert(usize::MAX);
        if &explorer.steps >= local_steps {
            continue;
        } else {
            *local_steps = explorer.steps;
            // if it's a local best, it might be a global best
            let global_steps = fewest_steps_by_keys
                .entry(explorer.keys())
                .or_insert(usize::MAX);
            if &explorer.steps < global_steps {
                *global_steps = explorer.steps;
            }
        }
        // println!("Best {}", explorer);
        let next: Vec<Explorer> = explorer
            .loc
            .neighbors()
            .iter()
            .filter(|&loc| {
                if let Some(tile) = vault.tile_at(loc) {
                    !explorer.just_visited(loc)
                        && match tile {
                            Tile::Wall => false,
                            Tile::Door(door) => explorer.has(&key_for(door)),
                            Tile::Entrance | Tile::Empty | Tile::Key(_) => true,
                        }
                } else {
                    false // out-of-bounds
                }
            })
            .map(|loc| explorer.stepped_once_to(loc))
            .collect();
        explorers.extend(next);
        // println!(" {} explorers", explorers.len());
        // explorers.iter().for_each(|e| println!(" - {}", e));
    }
    fewest_steps_by_keys
        .into_iter()
        .max_by(
            |(a_keys, a_steps), (b_keys, b_steps)| match a_keys.count().cmp(&b_keys.count()) {
                Ordering::Equal => a_steps.cmp(b_steps).reverse(),
                more_keys => more_keys,
            },
        )
        .unwrap()
}

// This can also be used to solve part 1, but it's slower than the alternative above
#[allow(unused)]
fn count_steps_to_collect_every_key_part1(input: Vec<&str>) -> Steps {
    let vault = Vault::from(input);
    // println!("Vault:\n{}", vault.to_string());

    // The following is *not* an original solution
    min_steps_to_collect_all_keys(
        &vault.entrances(),
        &Keys::default(),
        &mut HashMap::new(),
        &vault,
    )
}

fn count_steps_to_collect_every_key_part2(input: Vec<&str>) -> Steps {
    let mut vault = Vault::from(input);
    vault.replace_single_entrance_with_four_entrances();
    // println!("Vault:\n{}", vault.to_string());

    // The following is *not* an original solution
    min_steps_to_collect_all_keys(
        &vault.entrances(),
        &Keys::default(),
        &mut HashMap::new(),
        &vault,
    )
}

fn min_steps_to_collect_all_keys(
    starts: &[Loc],
    keys: &Keys,
    cache: &mut HashMap<(Vec<Loc>, Keys), Steps>,
    vault: &Vault,
) -> Steps {
    // Return previous result, if any
    let state = (starts.to_vec(), keys.clone());
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }
    // Otherwise find all the next key locations that can be reached from the given
    // starting locations with the given keys,
    // and recursively try to reach more keys from the previously reached key locations
    let step_count = starts
        .iter()
        .flat_map(|start| find_reachable_keys_from(start, keys, vault))
        .map(|(reached_key, at_loc, steps, from_start_loc)| {
            // Remove the previous starting position
            let mut new_starts = starts.to_vec();
            if let Some(pos) = new_starts.iter().position(|loc| loc == &from_start_loc) {
                new_starts.remove(pos);
            }
            // Add the key location as new starting position
            new_starts.push(at_loc);

            // Add the new key
            let mut new_keys = keys.clone();
            new_keys.add(reached_key);

            // Add the steps to this key to the next steps that are found recursively
            steps + min_steps_to_collect_all_keys(&new_starts, &new_keys, cache, vault)
        })
        // Use the shortest total distance if there are several paths
        .min()
        // There might be no more keys to reach, if we found all keys
        .unwrap_or(0);
    // Cache this best result
    cache.insert(state, step_count);
    step_count
}

// HashMap is kinda slow, so let's try alternatives
// Times for example 4
// - Vec<Vec<Steps>>  ~2.6s
// - Vec<(Loc,Steps)> ~3.4s
// - HashMap          ~3.5s
trait StepCountAtLoc {
    fn set_steps_at(&mut self, loc: Loc, steps: Steps);
    fn contains(&self, loc: &Loc) -> bool;
    fn get_steps_at(&self, loc: &Loc) -> Steps;
}
impl StepCountAtLoc for HashMap<Loc, Steps> {
    fn set_steps_at(&mut self, loc: Loc, steps: Steps) {
        self.insert(loc, steps);
    }
    fn contains(&self, loc: &Loc) -> bool {
        self.contains_key(loc)
    }

    fn get_steps_at(&self, loc: &Loc) -> Steps {
        *self.get(loc).unwrap()
    }
}
impl StepCountAtLoc for Vec<Vec<Steps>> {
    fn set_steps_at(&mut self, loc: Loc, steps: Steps) {
        self[loc.y][loc.x] = steps;
    }
    fn contains(&self, loc: &Loc) -> bool {
        self[loc.y][loc.x] < Steps::MAX
    }
    fn get_steps_at(&self, loc: &Loc) -> Steps {
        self[loc.y][loc.x]
    }
}
impl StepCountAtLoc for Vec<(Loc, Steps)> {
    fn set_steps_at(&mut self, loc: Loc, steps: Steps) {
        self.push((loc, steps));
    }
    fn contains(&self, loc: &Loc) -> bool {
        self.iter().any(|(l, _s)| l == loc)
    }
    fn get_steps_at(&self, loc: &Loc) -> Steps {
        self.iter().find(|(l, _s)| l == loc).unwrap().1
    }
}

fn find_reachable_keys_from(
    start: &Loc,
    keys: &Keys,
    vault: &Vault,
) -> Vec<(Key, Loc, Steps, Loc)> {
    let mut reachable_keys: Vec<(Key, Loc, Steps, Loc)> = vec![];
    let mut steps = vec![vec![Steps::MAX; vault.width()]; vault.height()];
    steps.set_steps_at(*start, 0);
    let mut queue = BinaryHeap::new();
    queue.push(*start);
    while let Some(next) = queue.pop() {
        next.neighbors()
            .iter()
            .filter(|&loc| match vault.tile_at(loc) {
                None | Some(Tile::Wall) => false,
                Some(Tile::Door(door)) => keys.contains(&key_for(door)),
                _ => true,
            })
            .for_each(|loc| {
                if !steps.contains(loc) {
                    steps.set_steps_at(*loc, steps.get_steps_at(&next) + 1);
                    if let Some(Tile::Key(key)) = vault.tile_at(loc) {
                        if !keys.contains(key) {
                            reachable_keys.push((*key, *loc, steps.get_steps_at(loc), *start));
                        } else {
                            queue.push(*loc);
                        }
                    } else {
                        queue.push(*loc);
                    }
                }
            });
    }
    reachable_keys
}

fn key_for(door: &char) -> char {
    (*door as u8 + 32) as char
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Explorer {
    loc: Loc,
    steps: Steps,
    keys: Keys,
    prev: Loc,
}

impl Ord for Explorer {
    fn cmp(&self, other: &Self) -> Ordering {
        // Fewer steps is better
        match self.steps.cmp(&other.steps) {
            // More keys is better
            Ordering::Equal => self.keys.count().cmp(&other.keys.count()),
            steps => steps.reverse(),
        }
    }
}

impl PartialOrd<Self> for Explorer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Explorer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} w {}: {}", self.loc, self.steps, self.keys)
    }
}

impl Explorer {
    fn initial(loc: Loc) -> Self {
        Explorer {
            loc,
            steps: 0,
            keys: Keys::default(),
            prev: loc,
        }
    }
    fn just_visited(&self, loc: &Loc) -> bool {
        &self.prev == loc
    }
    fn stepped_once_to(&self, new: &Loc) -> Self {
        Explorer {
            loc: *new,
            steps: self.steps + 1,
            keys: self.keys.clone(),
            prev: self.loc,
        }
    }
    fn has(&self, key: &Key) -> bool {
        self.keys.contains(key)
    }
    fn add_key(&mut self, key: Key) {
        if !self.keys.contains(&key) {
            self.keys.add(key);
            // prev loc may be visited again with more keys, only otherwise it would be pointless
            self.prev = self.loc;
        }
    }
    fn keys(&self) -> Keys {
        self.keys.clone()
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Entrance,
    Wall,
    Empty,
    Key(char),
    Door(char),
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '@' => Tile::Entrance,
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'a'..='z' => Tile::Key(ch),
            'A'..='Z' => Tile::Door(ch),
            _ => panic!("Invalid tile {}", ch),
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Entrance => '@',
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Key(k) => *k,
            Tile::Door(d) => *d,
        }
        .to_string()
    }
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Vault {
    grid: Grid,
}

impl From<Vec<&str>> for Vault {
    fn from(input: Vec<&str>) -> Self {
        let grid = input
            .iter()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Vault { grid }
    }
}

impl ToString for Vault {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(Tile::to_string).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Vault {
    fn tile_at(&self, loc: &Loc) -> Option<&Tile> {
        self.grid.get(loc.y).and_then(|row| row.get(loc.x))
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    fn entrance(&self) -> Loc {
        self.entrances()[0]
    }
    fn entrances(&self) -> Vec<Loc> {
        self.locations_matching(&|tile| tile == &Tile::Entrance)
    }
    fn locations_matching(&self, filter: &dyn Fn(&Tile) -> bool) -> Vec<Loc> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if filter(tile) {
                        Some(Loc::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    fn replace_single_entrance_with_four_entrances(&mut self) {
        let loc = &self.entrance();
        self.grid[loc.y - 1][loc.x - 1] = Tile::Entrance;
        self.grid[loc.y - 1][loc.x + 1] = Tile::Entrance;
        self.grid[loc.y + 1][loc.x - 1] = Tile::Entrance;
        self.grid[loc.y + 1][loc.x + 1] = Tile::Entrance;
        self.grid[loc.y][loc.x] = Tile::Wall;
        self.grid[loc.y][loc.x - 1] = Tile::Wall;
        self.grid[loc.y][loc.x + 1] = Tile::Wall;
        self.grid[loc.y - 1][loc.x] = Tile::Wall;
        self.grid[loc.y + 1][loc.x] = Tile::Wall;
    }
}

type Coord = usize;
type X = Coord;
type Y = Coord;

#[derive(Clone, Eq, PartialEq, Hash, Copy, Ord, PartialOrd)]
struct Loc {
    x: X,
    y: Y,
}
impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@({:2}, {:2})", self.x, self.y)
    }
}
impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Loc {
    fn new(x: X, y: Y) -> Self {
        Loc { x, y }
    }
    fn neighbors(&self) -> Vec<Loc> {
        let mut neighbors = vec![self.offset(0, 1), self.offset(1, 0)];
        if self.x > 0 {
            neighbors.push(self.offset(-1, 0));
        }
        if self.y > 0 {
            neighbors.push(self.offset(0, -1));
        };
        neighbors
    }
    fn offset(&self, x: isize, y: isize) -> Loc {
        Loc {
            x: (self.x as isize + x) as usize,
            y: (self.y as isize + y) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn key_for_door() {
        assert_eq!('a', key_for(&'A'));
    }

    #[test]
    fn example_1_small() {
        assert_eq!(
            8,
            count_steps_to_collect_every_key(parse(
                "\
#########
#b.A.@.a#
#########"
            ))
        );
    }

    #[test]
    fn example_2_larger() {
        assert_eq!(
            86,
            count_steps_to_collect_every_key(parse(
                "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
            ))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            132,
            count_steps_to_collect_every_key(parse(
                "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
            ))
        );
    }

    // ~1s slow when using explore(…)
    // ~4s slow when using minimum_steps(…)
    #[test]
    fn example_4() {
        assert_eq!(
            136,
            count_steps_to_collect_every_key(parse(
                "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
            ))
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            81,
            count_steps_to_collect_every_key(parse(
                "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
            ))
        );
    }
    // ~25s slow when using explore(…)
    // ~32s slow when using minimum_steps(…)
    #[test]
    fn part1() {
        assert_eq!(3270, day18_part1());
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(
            8,
            count_steps_to_collect_every_key_part2(parse(
                "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######"
            ))
        );
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(
            24,
            count_steps_to_collect_every_key_part2(parse(
                "\
###############
#d.ABC.#.....a#
######.#.######
######.@.######
######.#.######
#b.....#.....c#
###############"
            ))
        );
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(
            32,
            count_steps_to_collect_every_key_part2(parse(
                "\
#############
#DcBa.#.GhKl#
#.###.#.#I###
#e#d##@##j#k#
###C#.#.###J#
#fEbA.#.FgHi#
#############"
            ))
        );
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(
            72,
            count_steps_to_collect_every_key_part2(parse(
                "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"
            ))
        );
    }

    // #[test]
    #[allow(unused)]
    fn part2() {
        assert_eq!(1628, day18_part2());
    }

    #[test]
    fn key_to_idx() {
        assert_eq!(0u8, 'a'.to_idx());
        assert_eq!(25u8, 'z'.to_idx());
    }

    #[test]
    fn idx_to_key() {
        assert_eq!('a', 0u8.to_key());
        assert_eq!('z', 25u8.to_key());
    }

    #[test]
    fn adding_and_testing_for_keys() {
        let mut keys = Keys::default();

        let key = 'a';
        assert!(!keys.contains(&key));
        keys.add(key);
        assert!(keys.contains(&key));
        assert_eq!(1, keys.count());

        let key = 'z';
        assert!(!keys.contains(&key));
        keys.add(key);
        assert!(keys.contains(&key));
        assert_eq!(2, keys.count());
    }
}
