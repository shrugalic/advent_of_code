use std::collections::{HashMap, HashSet};
use SmallCaveConstraint::*;

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn day12_part1() -> usize {
    CaveSystem::from(INPUT).number_of_paths(CanVisitOnlyOnce)
}

pub(crate) fn day12_part2() -> usize {
    CaveSystem::from(INPUT).number_of_paths(CanVisitOneAtMostTwiceAndOthersOnlyOnce)
}

struct CaveSystem<'a> {
    neighbors: HashMap<&'a str, Vec<&'a str>>,
}
impl<'a> CaveSystem<'a> {
    fn number_of_paths(&self, constraint: SmallCaveConstraint) -> usize {
        let mut paths: HashSet<String> = HashSet::new();
        let mut explorers = vec![Explorer::with(constraint)];

        while let Some(explorer) = explorers.pop() {
            if explorer.reached_the_end() {
                paths.insert(explorer.path());
                continue;
            }
            for cave in &self.neighbors[&explorer.curr_cave()] {
                if explorer.can_visit(cave) {
                    explorers.push(explorer.visit(cave));
                }
            }
        }
        paths.len()
    }
}
impl<'a> From<&'a str> for CaveSystem<'a> {
    fn from(input: &'a str) -> Self {
        let mut neighbors = HashMap::new();
        for connection in input.trim().lines() {
            let (from, to) = connection.split_once('-').unwrap();
            neighbors.entry(from).or_insert_with(Vec::new).push(to);
            neighbors.entry(to).or_insert_with(Vec::new).push(from);
        }
        CaveSystem { neighbors }
    }
}

#[derive(Clone)]
struct Explorer<'a> {
    path: Vec<&'a str>,
    constraint: SmallCaveConstraint,
}
impl<'a> Explorer<'a> {
    fn with(constraint: SmallCaveConstraint) -> Self {
        Explorer {
            path: vec!["start"],
            constraint,
        }
    }
    fn visit(&self, cave: &'a str) -> Self {
        let mut explorer = self.clone();
        explorer.path.push(cave);
        if cave.is_small() && self.visited(cave) {
            explorer.constraint = CanVisitOnlyOnce;
        }
        explorer
    }
    fn curr_cave(&self) -> &'a str {
        self.path.last().unwrap()
    }
    fn visited(&self, cave: &'a str) -> bool {
        self.path.contains(&cave)
    }
    fn can_visit(&self, cave: &'a str) -> bool {
        !self.visited(cave) || cave.is_large() || (self.can_visit_small(cave))
    }
    fn can_visit_small(&self, small_cave: &str) -> bool {
        self.constraint == CanVisitOneAtMostTwiceAndOthersOnlyOnce
            && !["start", "end"].contains(&small_cave)
    }
    fn reached_the_end(&self) -> bool {
        self.curr_cave() == "end"
    }
    fn path(&self) -> String {
        self.path.join(",")
    }
}

#[derive(PartialEq, Clone)]
enum SmallCaveConstraint {
    CanVisitOnlyOnce,
    CanVisitOneAtMostTwiceAndOthersOnlyOnce,
}

trait CaveSize {
    fn is_large(&self) -> bool;
    fn is_small(&self) -> bool {
        !self.is_large()
    }
}
impl CaveSize for &str {
    fn is_large(&self) -> bool {
        self.chars().next().unwrap().is_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE2: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EXAMPLE3: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part1_example1() {
        let cave = CaveSystem::from(EXAMPLE1);
        assert_eq!(
            10,
            cave.number_of_paths(SmallCaveConstraint::CanVisitOnlyOnce)
        );
    }

    #[test]
    fn part1_example2() {
        let cave = CaveSystem::from(EXAMPLE2);
        assert_eq!(
            19,
            cave.number_of_paths(SmallCaveConstraint::CanVisitOnlyOnce)
        );
    }

    #[test]
    fn part1_example3() {
        let cave = CaveSystem::from(EXAMPLE3);
        assert_eq!(
            226,
            cave.number_of_paths(SmallCaveConstraint::CanVisitOnlyOnce)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(3708, day12_part1());
    }

    #[test]
    fn part2_example1() {
        let cave = CaveSystem::from(EXAMPLE1);
        assert_eq!(
            36,
            cave.number_of_paths(SmallCaveConstraint::CanVisitOneAtMostTwiceAndOthersOnlyOnce)
        );
    }

    #[test]
    fn part2_example2() {
        let cave = CaveSystem::from(EXAMPLE2);
        assert_eq!(
            103,
            cave.number_of_paths(SmallCaveConstraint::CanVisitOneAtMostTwiceAndOthersOnlyOnce)
        );
    }

    #[test]
    fn part2_example3() {
        let cave = CaveSystem::from(EXAMPLE3);
        assert_eq!(
            3509,
            cave.number_of_paths(SmallCaveConstraint::CanVisitOneAtMostTwiceAndOthersOnlyOnce)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(93_858, day12_part2());
    }
}
