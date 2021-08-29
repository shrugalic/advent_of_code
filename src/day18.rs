use line_reader::read_file_to_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};

pub(crate) fn day18_part1() -> usize {
    count_steps_to_collect_every_key(read_file_to_lines("input/day18.txt"))
}

pub(crate) fn day18_part2() -> usize {
    // TODO
    0
}

type Key = char;
type KeyDesc = String;
type Keys = String;
type Steps = usize;
fn count_steps_to_collect_every_key(input: Vec<String>) -> Steps {
    let vault = Vault::from(input);
    let key_count = vault.key_count();
    println!("Vault:\n{}", vault.to_string());

    // Keeps track of visited locations with steps-traveled by key-set.
    let mut visited: Vec<Vec<HashMap<KeyDesc, Steps>>> =
        vec![vec![HashMap::new(); vault.width()]; vault.height()];
    let mut finished = vec![];
    let mut explorers = BinaryHeap::new();
    explorers.push(Explorer::initial(vault.entrance()));
    while let Some(mut explorer) = explorers.pop() {
        if let Some(Tile::Key(key)) = vault.tile_at(&explorer.loc) {
            explorer.add_key(*key);
        }
        let local_steps = visited[explorer.loc.y][explorer.loc.x]
            .entry(explorer.key_desc())
            .or_insert(usize::MAX);
        if &explorer.steps >= local_steps {
            // println!("Removing {} >= local {}", explorer, local_steps);
            continue;
        } else {
            *local_steps = explorer.steps;
        }
        // println!("Best {}", explorer);

        if explorer.found_all_keys(key_count) {
            finished.push(explorer);
            continue;
        }
        explorers.extend(
            explorer
                .loc
                .neighbors()
                .iter()
                .filter(|loc| explorer.can_visit(vault.tile_at(loc)) && !explorer.just_visited(loc))
                .map(|loc| explorer.stepped_once_to(loc)),
        );
        // println!(" {} explorers", explorers.len());
        // explorers.iter().for_each(|e| println!(" - {}", e));

        // if explorers.len() > 4_000 {
        //     println!("aborting!");
        //     break;
        // }

        // if explorer.loc.x == 1 && explorer.loc.y == 3 {
        //     break;
        // }
    }
    println!("{} finished", finished.len());
    finished.iter().map(|e| e.steps).min().unwrap()
}

fn key_for(door: &char) -> char {
    (*door as u8 + 32) as char
}
#[derive(Debug, Eq, PartialEq)]
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
            Ordering::Equal => self.keys.len().cmp(&other.keys.len()),
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
        write!(f, "{} w {}: {:?}", self.loc, self.steps, self.keys)
    }
}

impl Explorer {
    fn initial(loc: Loc) -> Self {
        let prev = loc.clone();
        Explorer {
            loc,
            steps: 0,
            keys: String::new(),
            prev,
        }
    }
    fn can_visit(&self, tile: Option<&Tile>) -> bool {
        match tile {
            Some(Tile::Entrance) | Some(Tile::Empty) | Some(Tile::Key(_)) => true,
            Some(Tile::Door(door)) => self.has_key_for(door),
            Some(Tile::Wall) | None => false,
        }
    }
    fn just_visited(&self, loc: &Loc) -> bool {
        &self.prev == loc
    }
    fn stepped_once_to(&self, new: &Loc) -> Self {
        Explorer {
            loc: new.clone(),
            steps: self.steps + 1,
            keys: self.keys.clone(),
            prev: self.loc.clone(),
        }
    }
    fn has_key_for(&self, door: &char) -> bool {
        self.keys.chars().any(|key| key == key_for(door))
    }
    fn found_all_keys(&self, key_count: usize) -> bool {
        self.keys.len() == key_count
    }
    fn add_key(&mut self, key: Key) {
        let mut keys: Vec<_> = self.keys.chars().collect();
        if !keys.contains(&key) {
            keys.push(key);
            // prev loc may be visited again with more keys, only otherwise it would be pointless
            self.prev = self.loc.clone();
        }
        keys.sort_unstable();
        self.keys = keys.iter().collect();
    }
    fn key_desc(&self) -> KeyDesc {
        self.keys.clone()
    }
}

#[derive(Debug)]
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

impl From<Vec<String>> for Vault {
    fn from(input: Vec<String>) -> Self {
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
    fn key_count(&self) -> usize {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| matches!(tile, Tile::Key(_)))
                    .count()
            })
            .sum()
    }
    fn entrance(&self) -> Loc {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if let Tile::Entrance = tile {
                        Some(Loc::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .next()
            .unwrap()
    }
}

type Coord = usize;
type X = Coord;
type Y = Coord;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Loc {
    x: X,
    y: Y,
}
impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@({}, {})", self.x, self.y)
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
    use line_reader::read_str_to_lines;

    #[test]
    fn key_for_door() {
        assert_eq!('a', key_for(&'A'));
    }

    #[test]
    fn example_1_small() {
        assert_eq!(
            8,
            count_steps_to_collect_every_key(read_str_to_lines(
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
            count_steps_to_collect_every_key(read_str_to_lines(
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
            count_steps_to_collect_every_key(read_str_to_lines(
                "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
            ))
        );
    }

    // Slowish ~1s
    #[test]
    fn example_4() {
        assert_eq!(
            136,
            count_steps_to_collect_every_key(read_str_to_lines(
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
            count_steps_to_collect_every_key(read_str_to_lines(
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

    // Slow! ~19s
    #[test]
    fn part1() {
        assert_eq!(3270, day18_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1, day18_part2());
    }
}
