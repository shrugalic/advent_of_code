use intcode::IntCodeComputer;
use line_reader::read_file_to_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub(crate) fn day25_part1() -> usize {
    let explore = false;
    if explore {
        explore_ship();
        0
    } else {
        follow_path()
    }
}

const BLACKLIST: [&str; 5] = [
    "infinite loop",       // results in an infinite loop
    "escape pod",          // You're launched into space! Bye!
    "molten lava",         // The molten lava is way too hot! You melt!
    "photons",             // It is suddenly completely dark! You are eaten by a Grue!
    "giant electromagnet", // The giant electromagnet is stuck to you.  You can't move!!
];
const TOO_HEAVY: &str = "A loud, robotic voice says \"Alert! Droids on this ship are \
    lighter than the detected value!\" and you are ejected back to the checkpoint.";
const TOO_LIGHT: &str = "A loud, robotic voice says \"Alert! Droids on this ship are \
    heavier than the detected value!\" and you are ejected back to the checkpoint.";

fn follow_path() -> usize {
    let mut droid = Droid::with(path_to_security_checkpoint_taking_all_items());
    droid.start();

    // The droid is now at the Security Checkpoint:
    // "In the next room, a pressure-sensitive floor will verify your identity."

    // Drop all items
    let all_items: Vec<_> = droid.inventory.iter().cloned().collect();
    for item in &all_items {
        droid.drop(item);
    }

    // Try all possible item combinations
    for combo in 1..2u16.pow((all_items.len()) as u32) {
        let mask = format!("{:0width$b}", combo, width = all_items.len());

        let items: Vec<_> = mask
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'1')
            .map(|(i, _)| &all_items[i])
            .collect();

        for item in items.iter() {
            droid.take(item);
        }
        // enter room with pressure sensitive floor / weight check
        let (desc, _) = droid.go(Dir::North, false);
        if desc != TOO_LIGHT && desc != TOO_HEAVY {
            // A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
            // Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.
            // "Oh, hello! You should be able to get in by typing 35717128 on the keypad at the main airlock."
            return 35717128;
        }
        for item in items {
            droid.drop(item);
        }
    }
    unreachable!()
}

fn path_to_security_checkpoint_taking_all_items() -> Vec<Dir> {
    vec![
        Dir::West,  // go to passages to fetch mutex
        Dir::South, // go to engineering
        Dir::South, // go to science lab
        Dir::South, // go to gift wrapping center to fetch polygon
        Dir::North, // go back to science lab
        Dir::East,  // go to stables to fetch weather machine
        Dir::West,  // go back to science lab
        Dir::North, // go back…
        Dir::North, // to…
        Dir::East,  // start
        Dir::South, // go to hot chocolate fountain to fetch hologram
        // Dir::West, Dir::East // skip Arcade, there's nothing to pick up
        Dir::North, // go back to start
        Dir::North, // go to holo deck
        Dir::East,  // go to crew quarters…
        Dir::East,  // to fetch molten lava
        Dir::West,  // go back…
        Dir::West,  // go to holo deck
        Dir::North, // go to observatory
        Dir::North, // go to corridor to fetch semiconductor
        Dir::West,  // go to kitchen to fetch monolith
        Dir::South, // go to sick bay to fetch giant electromagnet
        Dir::North, // go back…
        Dir::East,  // to corridor
        Dir::East,  // go to navigation to fetch prime number
        Dir::West,  // go back…
        Dir::South, // go to observatory
        Dir::West,  // go to storage
        Dir::North, // go to hallway to fetch jam
        Dir::West,  // go to security checkpoint
    ]
}

fn explore_ship() {
    let droid = Droid::new();
    let mut droids = BinaryHeap::new();
    droids.push(droid);
    let mut visited = HashSet::new();
    while let Some(mut droid) = droids.pop() {
        let (name, doors) = droid.start();
        if name == TOO_HEAVY || name == TOO_LIGHT {
            continue;
        }
        if visited.contains(&name) {
            continue;
        }
        println!("{}", name);
        println!("Bot: {}", droid.desc());
        println!("Doors: {:?}\n", doors);
        visited.insert(name);
        for dir in doors {
            let mut next = Droid::with(droid.path.clone());
            next.path.push(dir);
            droids.push(next);
        }
    }
}

#[derive(Debug)]
struct Droid {
    icc: IntCodeComputer,
    inventory: HashSet<String>,
    path: Vec<Dir>,
}
impl Ord for Droid {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.path.len().cmp(&other.path.len()) {
            Ordering::Equal => self.inventory.len().cmp(&other.inventory.len()),
            u => u.reverse(),
        }
    }
}
impl PartialOrd for Droid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Droid {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path) && self.inventory.eq(&other.inventory)
    }
}
impl Eq for Droid {}

impl Droid {
    fn new() -> Droid {
        Droid::with(vec![])
    }
    fn with(path: Vec<Dir>) -> Droid {
        Droid {
            icc: IntCodeComputer::new(parse_software_from_puzzle_input()),
            inventory: HashSet::new(),
            path,
        }
    }
    fn desc(&self) -> String {
        format!("{:?}, {:?}", self.path, self.inventory)
    }
    fn start(&mut self) -> (String, Vec<Dir>) {
        let mut output = self.proceed();
        let path = self.path.clone();
        path.into_iter().for_each(|dir| {
            output = self.go(dir, false);
        });
        output
    }

    fn proceed(&mut self) -> (String, Vec<Dir>) {
        let output = self.icc.run_until_waiting_for_input();
        self.process_output(output)
    }
    fn go(&mut self, dir: Dir, store_path: bool) -> (String, Vec<Dir>) {
        if store_path {
            self.path.push(dir);
        }
        self.execute(&*dir.to_string())
    }
    fn take(&mut self, item: &str) -> (String, Vec<Dir>) {
        self.execute(format!("take {}", item).as_str())
    }
    fn drop(&mut self, item: &str) -> (String, Vec<Dir>) {
        self.execute(format!("drop {}", item).as_str())
    }
    fn execute(&mut self, cmd: &str) -> (String, Vec<Dir>) {
        let input = cmd.chars().map(|c| c as u8 as isize).collect::<Vec<_>>();
        self.icc.add_inputs(&input);
        self.icc.add_input(b'\n' as isize);
        self.proceed()
    }
    fn process_output(&mut self, output: String) -> (String, Vec<Dir>) {
        // println!("{}", output);
        let parts: Vec<_> = output.trim().split("\n\n").collect();
        // println!("Parts: {:?}", parts);
        const TAKE: &str = "You take the ";
        const DROP: &str = "You drop the ";
        if parts[0].starts_with(TAKE) {
            let item = parts[0].trim_start_matches(TAKE).trim_end_matches('.');
            self.inventory.insert(item.to_string());
            if parts[1] != "Command?" {
                println!("Result of taking '{}': {}", item, parts[1]);
            }
            return (format!("Took '{}': {}", item, parts[1]), vec![]);
        } else if parts[0].starts_with(DROP) {
            let item = parts[0].trim_start_matches(DROP).trim_end_matches('.');
            self.inventory.remove(item);
            return (format!("Dropped '{}' from inventory", item), vec![]);
        } else if parts[0].starts_with("Items in your inventory:\n- ") {
            parts[0].split("\n- ").skip(1).for_each(|item| {
                self.inventory.insert(item.to_string());
            });
            return ("Updated inventory".to_string(), vec![]);
        } else if parts[0].starts_with("== Pressure-Sensitive Floor ==") {
            return (parts[2].to_string(), vec![]);
        } else if parts[0].starts_with("You can't go that way.") {
            println!("{}", self.desc());
            return (parts[0].to_string(), vec![]);
        } else if parts[0].is_empty() || parts.len() == 1 {
            return ("NO MORE OUTPUT!".to_string(), vec![]);
        }
        let mut name = parts[0].to_string();
        if name.contains('\n') {
            name = name.replace('\n', ": ");
            name = name.replace("== ", "");
            name = name.replace(" ==", "");
        }
        let doors = if parts[1].starts_with("Doors here lead:") {
            parts[1].split("\n- ").skip(1).map(Dir::from).collect()
        } else {
            println!("Unexpected outcome: {:?}", parts);
            vec![]
        };
        if parts[2].starts_with("Items here:") {
            parts[2]
                .split("\n- ")
                .skip(1)
                .filter(|item| !BLACKLIST.contains(item))
                .for_each(|item| {
                    // println!("Taking '{}'", item);
                    self.take(item);
                });
        }

        (name, doors)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}
impl From<&str> for Dir {
    fn from(dir: &str) -> Self {
        match dir {
            "north" => Dir::North,
            "east" => Dir::East,
            "south" => Dir::South,
            "west" => Dir::West,
            _ => panic!("Illegal direction {}", dir),
        }
    }
}
impl ToString for Dir {
    fn to_string(&self) -> String {
        match self {
            Dir::North => "north",
            Dir::East => "east",
            Dir::South => "south",
            Dir::West => "west",
        }
        .to_string()
    }
}

fn parse_software_from_puzzle_input() -> Vec<isize> {
    let input = read_file_to_lines("input/day25.txt");
    input[0].split(',').map(|n| n.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(35717128, day25_part1());
    }
}
