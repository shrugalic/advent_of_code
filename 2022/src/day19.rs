use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Deref, DerefMut, SubAssign};
use ResourceType::*;

const INPUT: &str = include_str!("../input/day19.txt");

pub(crate) fn day19_part1() -> usize {
    let input = parse_blueprints(INPUT);
    solve_part1(&input)
}

pub(crate) fn day19_part2() -> usize {
    let input = parse_blueprints(INPUT);
    solve_part2(&input)
}

fn solve_part1(blueprints: &[Blueprint]) -> usize {
    let max_time = 24;
    blueprints
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| (i + 1, blueprint))
        .map(|(quality, blueprint)| quality * max_geode_count_for(blueprint, max_time) as usize)
        .sum()
}

fn solve_part2(blueprints: &[Blueprint]) -> usize {
    let max_time = 32;
    blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| max_geode_count_for(blueprint, max_time) as usize)
        .product()
}

fn max_geode_count_for(blueprint: &Blueprint, max_time: u8) -> u8 {
    let initial_state = State {
        time_left: max_time,
        robots: RobotCounts::single_ore_robot(),
        resources: ResourceCounts::default(),
    };
    let mut cache: HashMap<State, u8> = HashMap::new();
    max_geode_count_starting_in_state(initial_state, blueprint, &mut cache)
}

fn max_geode_count_starting_in_state(
    state: State,
    blueprint: &Blueprint,
    cache: &mut HashMap<State, u8>,
) -> u8 {
    if state.time_left == 0 {
        return state.resources[Geode as usize];
    }

    if let Some(geode_count) = cache.get(&state) {
        return *geode_count;
    }

    let geode_count = state
        // decide on what types of robot we (still) need
        .robot_types_still_needed_for(blueprint)
        // fast forward to the state where one of those robots was built (if possible)
        .filter_map(|type_id| state.try_building_a_robot_of_type(type_id, blueprint))
        // calculate the maximum number of geodes cracked starting from that state
        .map(|next| max_geode_count_starting_in_state(next, blueprint, cache))
        .max()
        .unwrap_or(0);

    cache.insert(state, geode_count);
    geode_count
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ResourceType {
    Geode,
    Obsidian,
    Clay,
    Ore,
}

/// Counts of Geode, Obsidian, Clay and Ore resources (in this order)
#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq)]
struct ResourceCounts([u8; 4]);

/// RobotCounts are also equivalent to Resources: counts of Geode, Obsidian, Clay and Ore _robots_
type RobotCounts = ResourceCounts;

/// The type of robot is equivalent to the type of resource
type RobotType = ResourceType;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State {
    time_left: u8,
    robots: RobotCounts,
    resources: ResourceCounts,
}

/// RobotCost is equivalent to Resources: counts of Geode, Obsidian, Clay and Ore _resources_
type RobotCost = ResourceCounts;

/// Costs for Geode, Obsidian, Clay and Ore robots in this order
type RobotCosts = [RobotCost; 4];

#[derive(Debug)]
struct Blueprint {
    costs_by_robot: RobotCosts,
    max_cost_by_type: RobotCost,
}

impl AddAssign for ResourceCounts {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}
impl SubAssign for ResourceCounts {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

impl RobotCounts {
    fn single_ore_robot() -> Self {
        Self([0, 0, 0, 1])
    }
    fn resources_collected_in(&self, time: u8) -> ResourceCounts {
        ResourceCounts([
            self[0] * time,
            self[1] * time,
            self[2] * time,
            self[3] * time,
        ])
    }
}

const PRODUCTION_TIME: u8 = 1;
impl State {
    fn robot_types_still_needed_for<'a>(
        &'a self,
        blueprint: &'a Blueprint,
    ) -> impl Iterator<Item = RobotType> + '_ {
        // Heuristic: Geode robots are always needed, but from the others we won't need more than
        // the maximum cost of the resource it produces plus the resources we already have
        [Geode, Obsidian, Clay, Ore]
            .into_iter()
            .filter(move |&robot_type| {
                robot_type == Geode
                    || self.robots[robot_type as usize]
                        < blueprint.max_cost_by_type[robot_type as usize]
            })
    }
    fn try_building_a_robot_of_type(
        &self,
        robot_type: RobotType,
        blueprint: &Blueprint,
    ) -> Option<State> {
        let robot_cost = &blueprint.costs_by_robot[robot_type as usize];
        if !self.producing_materials_needed_to_cover(robot_cost) {
            return None;
        }

        let missing_resources = self.resources_missing_to_cover(robot_cost);
        let time_until_ready = self.time_to_collect(&missing_resources) + PRODUCTION_TIME;

        // Abort if too late
        let time_left = self.time_left.checked_sub(time_until_ready)?;

        // Update resources
        let mut resources = self.resources;
        resources += self.robots.resources_collected_in(time_until_ready);
        resources -= *robot_cost;

        // Add the robot we produced
        let mut robots = self.robots;
        robots[robot_type as usize] += 1;

        Some(State {
            time_left,
            robots,
            resources,
        })
    }
    fn producing_materials_needed_to_cover(&self, cost: &RobotCost) -> bool {
        cost.iter()
            .enumerate()
            .all(|(mat, cost)| cost == &0 || self.robots[mat] > 0)
    }
    fn resources_missing_to_cover(&self, cost: &RobotCost) -> ResourceCounts {
        // missing = costs - stock
        ResourceCounts([
            cost[0].saturating_sub(self.resources[0]),
            cost[1].saturating_sub(self.resources[1]),
            cost[2].saturating_sub(self.resources[2]),
            cost[3].saturating_sub(self.resources[3]),
        ])
    }
    fn time_to_collect(&self, needed: &ResourceCounts) -> u8 {
        needed
            .into_iter()
            .enumerate()
            .filter(|(_type_id, count)| count > &0)
            .map(|(type_id, quantity)| {
                // time = quantity / rate
                ((quantity as f64) / (self.robots[type_id] as f64)).ceil() as u8
            })
            .max()
            .unwrap_or(0)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:2} {}, {}",
            self.time_left, self.robots, self.resources
        )
    }
}

impl Display for ResourceCounts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// Quality of life ;)
impl Deref for ResourceCounts {
    type Target = [u8; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ResourceCounts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for Blueprint {
    fn from(line: &str) -> Self {
        let mut costs_by_robot = [RobotCost::default(); 4];
        for (robot_type, costs_by_mat) in line.split(" Each ").skip(1).map(|part| {
            // ore robot costs 2 ore.
            // clay robot costs 2 ore.
            // obsidian robot costs 2 ore and 17 clay.
            // geode robot costs 2 ore and 10 obsidian.
            let (robot_type, costs) = part.split_once(" robot costs ").unwrap();
            let mut costs_by_mat = RobotCost::default();
            for cost in costs.strip_suffix('.').unwrap().split(" and ") {
                let (count, material) = cost.split_once(' ').unwrap();
                costs_by_mat[ResourceType::from(material) as usize] = count.parse().unwrap();
            }
            (RobotType::from(robot_type), costs_by_mat)
        }) {
            costs_by_robot[robot_type as usize] = costs_by_mat;
        }

        let mut max_cost_by_type = RobotCost::default();
        for resource_type in 0..4 {
            max_cost_by_type[resource_type] = costs_by_robot
                .iter()
                .map(|costs| costs[resource_type])
                .max()
                .unwrap();
        }

        Blueprint {
            costs_by_robot,
            max_cost_by_type,
        }
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from).collect()
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Geode => "Geode",
                Obsidian => "Obsidian",
                Clay => "Clay",
                Ore => "Ore",
            }
        )
    }
}
impl From<&str> for ResourceType {
    fn from(s: &str) -> Self {
        match s {
            "ore" => Ore,
            "clay" => Clay,
            "obsidian" => Obsidian,
            "geode" => Geode,
            _ => unreachable!("Unknown materials {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_blueprints(EXAMPLE);
        assert_eq!(9 + 2 * 12, solve_part1(&input));
    }

    #[ignore]
    #[test]
    fn solve_part1_example_blueprint_1() {
        let input = parse_blueprints(EXAMPLE);
        assert_eq!(9, solve_part1(&input[0..=0]));
    }

    #[ignore]
    #[test]
    fn solve_part1_example_blueprint_2() {
        let input = parse_blueprints(EXAMPLE);
        assert_eq!(12, solve_part1(&input[1..=1]));
    }

    #[test]
    fn part1() {
        assert_eq!(1_550, day19_part1());
    }

    #[ignore]
    #[test]
    fn solve_part2_example_blueprint_1() {
        let input = parse_blueprints(EXAMPLE);
        assert_eq!(56, solve_part2(&input[0..=0]));
    }

    #[ignore]
    #[test]
    fn solve_part2_example_blueprint_2() {
        let input = parse_blueprints(EXAMPLE);
        assert_eq!(62, solve_part2(&input[1..=1]));
    }

    #[ignore] // Very slow at 105s
    #[test]
    fn part2_example() {
        let input = parse_blueprints(EXAMPLE);
        assert_eq!(56 * 62, solve_part2(&input));
    }

    #[test] // Somewhat slow at 5s
    fn part2() {
        assert_eq!(18_630, day19_part2());
    }

    const EXAMPLE: &str = "\
 Blueprint 1: \
  Each ore robot costs 4 ore. \
  Each clay robot costs 2 ore. \
  Each obsidian robot costs 3 ore and 14 clay. \
  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: \
  Each ore robot costs 2 ore. \
  Each clay robot costs 3 ore. \
  Each obsidian robot costs 3 ore and 8 clay. \
  Each geode robot costs 3 ore and 12 obsidian.
";
}
