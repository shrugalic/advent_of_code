use std::fmt::{Debug, Display, Formatter};

type TileRows = Vec<Vec<Tile>>;
type RoundCount = usize;
type HitPoints = u8;
type Coord = usize;
type StepCount = usize;
type StepCountRows = Vec<Vec<StepCount>>;
type SumOfRemainingHitPoints = usize;

const INITIAL_HP: HitPoints = 200;
const ATTACK_POWER: HitPoints = 3;

#[derive(Debug, PartialEq)]
pub(crate) struct Grid {
    rows: TileRows,
    rounds: RoundCount,
}

impl Grid {
    pub(crate) fn play_until_no_enemies_remain(&mut self) -> usize {
        loop {
            if let Some(sum_of_remaining_hit_points) = self.play_round() {
                return sum_of_remaining_hit_points;
            }
        }
    }
    fn play_round(&mut self) -> Option<SumOfRemainingHitPoints> {
        let mut units = self.unit_locations();
        while !units.is_empty() {
            let mut unit = units.remove(0);
            let enemies = self.enemies_of(&unit);
            if enemies.is_empty() {
                // If no more enemies remain, combat ends
                return Some(self.rounds * self.sum_of_remaining_hit_points());
            }
            // 1. Optionally move 1 step toward the closest enemy
            if !unit.is_in_range_of_any(&enemies) {
                let did_move = self.try_to_move(&mut unit, &enemies);
                if !did_move {
                    // If the unit cannot reach (find an open path to) any of the squares
                    // that are in range, it ends its turn.
                    continue;
                }
            }
            // 2. Attack if there's an enemy in range
            if unit.is_in_range_of_any(&enemies) {
                let enemy = self.lowest_hp_adjacent_enemy_of(&mut unit);
                let enemy_killed = self.attack(&enemy, ATTACK_POWER);
                if enemy_killed {
                    // Remove the killed unit from the units to be handled if it's still there
                    if let Some(pos) = units.iter().position(|loc| loc == &enemy) {
                        units.remove(pos);
                    }
                }
            }
        }
        self.rounds += 1;
        None
    }
    pub(crate) fn play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss(
        &mut self,
    ) -> usize {
        let rows_backup = self.rows.clone();
        let mut elf_attack_power: HitPoints = 3;
        loop {
            elf_attack_power += 1;
            if let Some(sum_of_remaining_hit_points) =
                self.play_until_all_an_elf_dies_or_all_goblins_are_dead(elf_attack_power)
            {
                println!(
                    "{} elves won after {} rounds with {} attack power",
                    self.elves().len(),
                    self.rounds,
                    elf_attack_power,
                );
                return sum_of_remaining_hit_points;
            }
            // reset
            self.rows = rows_backup.clone();
            self.rounds = 0;
        }
    }
    fn play_until_all_an_elf_dies_or_all_goblins_are_dead(
        &mut self,
        elf_attack_power: HitPoints,
    ) -> Option<SumOfRemainingHitPoints> {
        loop {
            let mut units = self.unit_locations();
            while !units.is_empty() {
                let mut unit = units.remove(0);
                let enemies = self.enemies_of(&unit);
                if enemies.is_empty() {
                    // If no more enemies remain, combat ends
                    return Some(self.rounds * self.sum_of_remaining_hit_points());
                }
                // 1. Optionally move 1 step toward the closest enemy
                if !unit.is_in_range_of_any(&enemies) {
                    let did_move = self.try_to_move(&mut unit, &enemies);
                    if !did_move {
                        // If the unit cannot reach (find an open path to) any of the squares
                        // that are in range, it ends its turn.
                        continue;
                    }
                }
                // 2. Attack if there's an enemy in range
                if unit.is_in_range_of_any(&enemies) {
                    let enemy = self.lowest_hp_adjacent_enemy_of(&mut unit);
                    let enemy_is_elf = self.is_elf_at(&enemy);
                    let attack_power = if enemy_is_elf {
                        ATTACK_POWER
                    } else {
                        elf_attack_power
                    };
                    let enemy_killed = self.attack(&enemy, attack_power);
                    if enemy_killed {
                        if enemy_is_elf {
                            // If an elf died, start over
                            return None;
                        }
                        // Remove the killed unit from the units to be handled if it's still there
                        if let Some(pos) = units.iter().position(|loc| loc == &enemy) {
                            units.remove(pos);
                        }
                    }
                }
            }
            self.rounds += 1;
        }
    }
    fn unit_locations(&self) -> Vec<Loc> {
        self.locations_of(&Tile::is_unit)
    }
    fn enemies_of(&self, loc: &Loc) -> Vec<Loc> {
        if self.is_elf_at(loc) {
            self.goblins()
        } else {
            self.elves()
        }
    }
    fn sum_of_remaining_hit_points(&self) -> usize {
        self.unit_locations()
            .iter()
            .map(|loc| self.hp_of_unit_at(loc) as usize)
            .sum::<usize>()
    }
    fn try_to_move(&mut self, unit: &mut Loc, enemies: &[Loc]) -> bool {
        // To move, the unit first considers the squares that are in range
        let targets: Vec<Loc> = self.open_squares_adjacent_to_any(enemies);
        // and determines which of those squares it could reach in the fewest steps.
        if let Some(closest) = self.choose_closest_location(unit, targets) {
            let new_pos = self.pick_first_step_to_target(unit, &closest);
            self.move_unit(unit, &new_pos);
            *unit = new_pos;
            true
        } else {
            // Unable to move
            false
        }
    }
    fn lowest_hp_adjacent_enemy_of(&mut self, curr_pos: &mut Loc) -> Loc {
        let enemies = self.enemies_of(&curr_pos);
        let mut adjacent_enemies: Vec<Loc> = curr_pos
            .neighbors()
            .into_iter()
            .filter(|adjacent_pos| enemies.contains(adjacent_pos))
            .collect();
        adjacent_enemies.sort_by_key(|loc| self.hp_of_unit_at(loc));
        adjacent_enemies.remove(0) // Lowest HP first
    }
    fn attack(&mut self, enemy: &Loc, attack_power: HitPoints) -> bool {
        if let Tile::Elf(hp) | Tile::Goblin(hp) = &mut self.rows[enemy.y][enemy.x] {
            if *hp > attack_power {
                *hp -= attack_power;
            } else {
                self.rows[enemy.y][enemy.x] = Tile::Open;
                return true; // Enemy unit was killed
            }
        }
        false
    }
    fn goblins(&self) -> Vec<Loc> {
        self.locations_of(&Tile::is_goblin)
    }
    fn elves(&self) -> Vec<Loc> {
        self.locations_of(&Tile::is_elf)
    }
    fn open_squares(&self) -> Vec<Loc> {
        self.locations_of(&Tile::is_open)
    }
    fn locations_of(&self, filter: &dyn Fn(&Tile) -> bool) -> Vec<Loc> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_x, tile)| filter(tile))
                    .map(move |(x, _tile)| Loc::new(x, y))
            })
            .collect()
    }
    fn open_squares_adjacent_to_any(&self, locs: &[Loc]) -> Vec<Loc> {
        self.open_squares()
            .into_iter()
            .filter(|open_square| locs.iter().any(|loc| loc.is_adjacent_to(open_square)))
            .collect()
    }
    fn closest_locations(&self, start: &Loc, target_locs: Vec<Loc>) -> Vec<Loc> {
        let reachable_locations = self.reachable_locations(start, target_locs);
        Grid::nearest(reachable_locations)
    }
    fn reachable_locations(&self, start: &Loc, target_locs: Vec<Loc>) -> Vec<(StepCount, Loc)> {
        const MAX: StepCount = StepCount::MAX;
        // Keeps track of the minimum steps needed to reach a given location
        let mut step_counts: StepCountRows = vec![vec![MAX; self.rows[0].len()]; self.rows.len()];
        step_counts[start.y][start.x] = 0;

        let mut current: Vec<(Loc, StepCount)> = vec![(start.clone(), 0)];
        while !current.is_empty() {
            let (curr_loc, curr_count) = current.pop().unwrap();
            for neighbor in self.open_neighbors_of(&curr_loc) {
                if curr_count + 1 < step_counts[neighbor.y][neighbor.x] {
                    step_counts[neighbor.y][neighbor.x] = curr_count + 1;
                    current.push((neighbor, curr_count + 1));
                } // Else there's a faster path already
            }
        }
        target_locs
            .into_iter()
            .filter(|loc| step_counts[loc.y][loc.x] < MAX)
            .map(|loc| (step_counts[loc.y][loc.x], loc))
            .collect()
    }
    fn nearest(locations: Vec<(StepCount, Loc)>) -> Vec<Loc> {
        if let Some((min_count, _loc)) = locations.iter().min_by_key(|(count, _)| *count).cloned() {
            locations
                .into_iter()
                .filter(|(count, _loc)| count == &min_count)
                .map(|(_count, loc)| loc)
                .collect()
        } else {
            vec![]
        }
    }
    fn choose_closest_location(&self, start: &Loc, target_locs: Vec<Loc>) -> Option<Loc> {
        self.closest_locations(start, target_locs).first().cloned()
    }
    fn open_neighbors_of(&self, loc: &Loc) -> Vec<Loc> {
        loc.neighbors()
            .into_iter()
            .filter(|loc| self.contains(loc) && self.is_open_at(loc))
            .collect()
    }
    fn contains(&self, loc: &Loc) -> bool {
        loc.x < self.width() && loc.y < self.height()
    }
    fn width(&self) -> usize {
        self.rows[0].len()
    }
    fn height(&self) -> usize {
        self.rows.len()
    }
    #[allow(dead_code)]
    fn print_unit_health(&self) {
        println!(
            "Round {}: {:?}",
            self.rounds,
            self.unit_locations()
                .iter()
                .map(|loc| format!("{:?}: {}", loc, self.hp_of_unit_at(loc)))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    fn hp_of_unit_at(&self, loc: &Loc) -> HitPoints {
        match &self.rows[loc.y][loc.x] {
            Tile::Elf(hp) | Tile::Goblin(hp) => *hp,
            Tile::Wall | Tile::Open => unreachable!(),
        }
    }
    fn pick_first_step_to_target(&self, unit: &Loc, closest_target: &Loc) -> Loc {
        // All open neighbors of the unit's current location are first step candidates:
        let first_step_candidates = self.open_neighbors_of(unit);
        // Reverse search from the target back to these candidates
        let distances_to_candidates =
            self.reachable_locations(&closest_target, first_step_candidates);
        // Pick the first of the closest ones (they are ordered properly by reading order)
        Grid::nearest(distances_to_candidates)
            .first()
            .cloned()
            .unwrap()
    }
    fn move_unit(&mut self, old_pos: &Loc, new_pos: &Loc) {
        self.rows[new_pos.y][new_pos.x] = self.rows[old_pos.y][old_pos.x].clone();
        self.rows[old_pos.y][old_pos.x] = Tile::Open;
    }

    fn is_elf_at(&self, loc: &Loc) -> bool {
        self.location_matches_tile_filter(loc, &Tile::is_elf)
    }

    fn is_open_at(&self, loc: &Loc) -> bool {
        self.location_matches_tile_filter(loc, &Tile::is_open)
    }

    fn location_matches_tile_filter(&self, loc: &Loc, filter: &dyn Fn(&Tile) -> bool) -> bool {
        filter(&self.rows[loc.y][loc.x])
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rows
                .iter()
                .map(|row| row
                    .iter()
                    .map(|tile| tile.to_string())
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl From<&Vec<String>> for Grid {
    fn from(input: &Vec<String>) -> Self {
        let grid = input
            .iter()
            .map(|row| row.chars().map(Tile::from).collect())
            .collect();
        Grid {
            rows: grid,
            rounds: 0,
        }
    }
}
#[derive(PartialEq, Clone)]
struct Loc {
    x: Coord,
    y: Coord,
}
impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Loc {
    fn new(x: Coord, y: Coord) -> Self {
        Loc { x, y }
    }
    fn is_adjacent_to(&self, other: &Loc) -> bool {
        self.x == other.x && (self.y + 1 == other.y || self.y == other.y + 1)
            || self.y == other.y && (self.x + 1 == other.x || self.x == other.x + 1)
    }
    fn is_in_range_of_any(&self, others: &[Loc]) -> bool {
        others
            .iter()
            .any(|other_loc| self.is_adjacent_to(other_loc))
    }
    fn neighbors(&self) -> Vec<Loc> {
        let mut neighbors = vec![];
        if self.y > 0 {
            neighbors.push(Loc::new(self.x, self.y - 1));
        }
        if self.x > 0 {
            neighbors.push(Loc::new(self.x - 1, self.y));
        }
        neighbors.push(Loc::new(self.x + 1, self.y));
        neighbors.push(Loc::new(self.x, self.y + 1));
        neighbors
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    Open,
    Goblin(HitPoints),
    Elf(HitPoints),
}
impl ToString for Tile {
    fn to_string(&self) -> String {
        String::from(match self {
            Tile::Wall => "#",
            Tile::Open => ".",
            Tile::Elf(_) => "E",
            Tile::Goblin(_) => "G",
        })
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            'E' => Tile::Elf(INITIAL_HP),
            'G' => Tile::Goblin(INITIAL_HP),
            _ => panic!("Illegal tile '{}'", ch),
        }
    }
}

impl Tile {
    fn is_unit(&self) -> bool {
        self.is_elf() || self.is_goblin()
    }
    fn is_elf(&self) -> bool {
        matches!(self, Tile::Elf(_))
    }
    fn is_goblin(&self) -> bool {
        matches!(self, Tile::Goblin(_))
    }
    fn is_open(&self) -> bool {
        matches!(self, Tile::Open)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const SIMPLE_GRID: &str = "\
####
#.G#
#E.#
####";

    #[test]
    fn tile_from_char() {
        assert_eq!(Tile::from('.'), Tile::Open);
    }

    #[test]
    fn simple_grid_from_input() {
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Open, Tile::Goblin(INITIAL_HP), Tile::Wall],
            vec![Tile::Wall, Tile::Elf(INITIAL_HP), Tile::Open, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        assert_eq!(
            Grid::from(&read_str_to_lines(SIMPLE_GRID)),
            Grid {
                rows: grid,
                rounds: 0
            }
        );
    }

    #[test]
    fn loc_is_adjacent_to_other() {
        let loc = Loc::new(1, 1);
        assert!(loc.is_adjacent_to(&Loc::new(1, 0)));
        assert!(loc.is_adjacent_to(&Loc::new(1, 2)));
        assert!(loc.is_adjacent_to(&Loc::new(0, 1)));
        assert!(loc.is_adjacent_to(&Loc::new(2, 1)));
        assert!(!loc.is_adjacent_to(&Loc::new(1, 1)));
        assert!(!loc.is_adjacent_to(&Loc::new(0, 0)));
        assert!(!loc.is_adjacent_to(&Loc::new(0, 2)));
        assert!(!loc.is_adjacent_to(&Loc::new(2, 0)));
        assert!(!loc.is_adjacent_to(&Loc::new(2, 2)));
    }

    // SIMPLE_GRID:
    //   0123
    // 0 ####
    // 1 #.G#
    // 2 #E.#
    // 3 ####

    #[test]
    fn unit_positions() {
        // See SIMPLE_GRID picture above
        let grid = Grid::from(&read_str_to_lines(SIMPLE_GRID));
        assert_eq!(grid.unit_locations(), vec![Loc::new(2, 1), Loc::new(1, 2),]);
    }

    #[test]
    fn goblins() {
        let grid = Grid::from(&read_str_to_lines(SIMPLE_GRID));
        assert_eq!(grid.goblins(), vec![Loc::new(2, 1)]);
    }

    #[test]
    fn elves() {
        let grid = Grid::from(&read_str_to_lines(SIMPLE_GRID));
        assert_eq!(grid.elves(), vec![Loc::new(1, 2),]);
    }

    #[test]
    fn enemies() {
        let grid = Grid::from(&read_str_to_lines(SIMPLE_GRID));
        let elf_loc = Loc::new(1, 2);
        let goblin_loc = Loc::new(2, 1);
        assert_eq!(grid.enemies_of(&elf_loc), vec![goblin_loc.clone()]);
        assert_eq!(grid.enemies_of(&goblin_loc), vec![elf_loc]);
    }

    #[test]
    fn loc_is_in_range_of_any_units() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        // Locations in range are marked as ? in the picture below.
        // All locations are checked: even though walls and units aren't suitable
        // open spots, their location might still be in range of a unit.
        let goblins = grid.goblins();
        // row 1
        assert!(!Loc::new(1, 1).is_in_range_of_any(&goblins));
        assert!(!Loc::new(2, 1).is_in_range_of_any(&goblins));
        assert!(Loc::new(3, 1).is_in_range_of_any(&goblins));
        assert!(!Loc::new(4, 1).is_in_range_of_any(&goblins)); // Unit
        assert!(Loc::new(5, 1).is_in_range_of_any(&goblins));
        // row 2
        assert!(!Loc::new(1, 2).is_in_range_of_any(&goblins));
        assert!(Loc::new(2, 2).is_in_range_of_any(&goblins));
        assert!(!Loc::new(3, 2).is_in_range_of_any(&goblins));
        assert!(Loc::new(4, 2).is_in_range_of_any(&goblins)); // Wall
        assert!(Loc::new(5, 2).is_in_range_of_any(&goblins));
        // row 3
        assert!(Loc::new(1, 3).is_in_range_of_any(&goblins));
        assert!(!Loc::new(2, 3).is_in_range_of_any(&goblins)); // Unit
        assert!(Loc::new(3, 3).is_in_range_of_any(&goblins));
        assert!(Loc::new(4, 3).is_in_range_of_any(&goblins)); // Wall
        assert!(!Loc::new(5, 3).is_in_range_of_any(&goblins)); // Unit
    }
    //   0123456
    // 0 #######
    // 1 #E.?G?#
    // 2 #.?.#?#
    // 3 #?G?#G#
    // 4 #######
    const PATH_FINDING_GRID: &str = "\
#######
#E..G.#
#...#.#
#.G.#G#
#######";

    #[test]
    fn open_squares_adjacent_to_any_goblins() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        let goblins = grid.goblins();
        let expected_open_squares = vec![
            Loc::new(3, 1),
            Loc::new(5, 1),
            Loc::new(2, 2),
            Loc::new(5, 2),
            Loc::new(1, 3),
            Loc::new(3, 3),
        ];
        assert_eq!(
            grid.open_squares_adjacent_to_any(&goblins),
            expected_open_squares
        );
    }

    #[test]
    fn open_neighbors_top_left() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        assert_eq!(
            grid.open_neighbors_of(&Loc::new(1, 1)),
            vec![Loc::new(2, 1), Loc::new(1, 2)]
        );
    }
    #[test]
    fn open_neighbors_top_right() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        assert_eq!(
            grid.open_neighbors_of(&Loc::new(5, 1)),
            vec![Loc::new(5, 2)]
        );
    }
    #[test]
    fn open_neighbors_bottom_left() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        assert_eq!(
            grid.open_neighbors_of(&Loc::new(1, 3)),
            vec![Loc::new(1, 2)]
        );
    }
    #[test]
    fn open_neighbors_bottom_right() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        assert_eq!(
            grid.open_neighbors_of(&Loc::new(5, 3)),
            vec![Loc::new(5, 2)]
        );
    }
    #[test]
    fn open_neighbors_somewhat_center() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        assert_eq!(
            grid.open_neighbors_of(&Loc::new(2, 2)),
            vec![Loc::new(2, 1), Loc::new(1, 2), Loc::new(3, 2)]
        );
    }

    #[test]
    fn reachable_locations() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        let elf = Loc::new(1, 1);
        let open_squares = grid.open_squares_adjacent_to_any(&grid.goblins());
        // In range:     Reachable:
        // #######       #######
        // #E.?G?#       #E.@G.#
        // #.?.#?#  -->  #.@.#.#
        // #?G?#G#       #@G@#G#
        // #######       #######
        let expected = vec![
            (2, Loc::new(3, 1)),
            (2, Loc::new(2, 2)),
            (2, Loc::new(1, 3)),
            (4, Loc::new(3, 3)),
        ];
        assert_eq!(expected, grid.reachable_locations(&elf, open_squares));
    }
    #[test]
    fn nearest_reachable_locations() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        let elf = Loc::new(1, 1);
        let open_squares = grid.open_squares_adjacent_to_any(&grid.goblins());
        // Reachable:    Nearest:
        // #######       #######
        // #E.@G.#       #E.!G.#
        // #.@.#.#  -->  #.!.#.#
        // #@G@#G#       #!G.#G#
        // #######       #######
        let expected = vec![Loc::new(3, 1), Loc::new(2, 2), Loc::new(1, 3)];
        assert_eq!(expected, grid.closest_locations(&elf, open_squares));
    }
    #[test]
    fn chosen_nearest_location() {
        let grid = Grid::from(&read_str_to_lines(PATH_FINDING_GRID));
        let elf = Loc::new(1, 1);
        let open_squares = grid.open_squares_adjacent_to_any(&grid.goblins());
        // Reachable:    Nearest:      Chosen:
        // #######       #######       #######
        // #E.@G.#       #E.!G.#       #E.+G.#
        // #.@.#.#  -->  #.!.#.#  -->  #...#.#
        // #@G@#G#       #!G.#G#       #.G.#G#
        // #######       #######       #######
        assert_eq!(
            grid.choose_closest_location(&elf, open_squares),
            Some(Loc::new(3, 1))
        );
    }

    const CHOSEN_PATH_GRID: &str = "\
#######
#.E...#
#.....#
#...G.#
#######";
    #[test]
    fn first_steps() {
        let grid = Grid::from(&read_str_to_lines(CHOSEN_PATH_GRID));
        // In range:       Nearest:      Chosen:       Distance:     Step:
        //   0123456
        // 0 #######       #######       #######       #######       #######
        // 1 #.E...#       #.E...#       #.E...#       #4E212#       #..E..#
        // 2 #...?.#  -->  #...!.#  -->  #...+.#  -->  #32101#  -->  #.....#
        // 3 #..?G?#       #..!G.#       #...G.#       #432G2#       #...G.#
        // 4 #######       #######       #######       #######       #######

        let elf = Loc::new(2, 1);
        let targets: Vec<Loc> = grid.open_squares_adjacent_to_any(&grid.goblins());
        let closest = grid.choose_closest_location(&elf, targets).unwrap();
        let first_steps = grid.pick_first_step_to_target(&elf, &closest);
        assert_eq!(first_steps, Loc::new(3, 1));
    }

    const MOVEMENT_EXAMPLE_INITIAL: &str = "\
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
    const MOVEMENT_EXAMPLE_AFTER_1_ROUND: &str = "\
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########";
    const MOVEMENT_EXAMPLE_AFTER_2_ROUNDS: &str = "\
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########";
    const MOVEMENT_EXAMPLE_AFTER_3_ROUNDS: &str = "\
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########";
    #[test]
    fn movement_example() {
        let mut grid = Grid::from(&read_str_to_lines(MOVEMENT_EXAMPLE_INITIAL));

        grid.play_round();
        assert_eq!(
            format!("{}", grid),
            String::from(MOVEMENT_EXAMPLE_AFTER_1_ROUND)
        );

        grid.play_round();
        assert_eq!(
            format!("{}", grid),
            String::from(MOVEMENT_EXAMPLE_AFTER_2_ROUNDS)
        );

        grid.play_round();
        assert_eq!(
            format!("{}", grid),
            String::from(MOVEMENT_EXAMPLE_AFTER_3_ROUNDS)
        );

        // Units can no longer move, so it stays like this until a unit dies
        grid.play_round();
        assert_eq!(
            format!("{}", grid),
            String::from(MOVEMENT_EXAMPLE_AFTER_3_ROUNDS)
        );
    }

    const COMBAT_EXAMPLE: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    #[test]
    fn combat_example() {
        let mut grid = Grid::from(&read_str_to_lines(COMBAT_EXAMPLE));
        let sum_of_remaining_hit_points = grid.play_until_no_enemies_remain();
        assert_eq!(27_730, sum_of_remaining_hit_points);
    }

    #[test]
    fn combat_example_2() {
        assert_eq!(
            36_334,
            Grid::from(&read_str_to_lines(
                "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
            ))
            .play_until_no_enemies_remain()
        );
    }

    #[test]
    fn combat_example_3() {
        assert_eq!(
            39_514,
            Grid::from(&read_str_to_lines(
                "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
            ))
            .play_until_no_enemies_remain()
        );
    }

    #[test]
    fn combat_example_4() {
        assert_eq!(
            27_755,
            Grid::from(&read_str_to_lines(
                "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
            ))
            .play_until_no_enemies_remain()
        );
    }

    #[test]
    fn combat_example_5() {
        assert_eq!(
            28_944,
            Grid::from(&read_str_to_lines(
                "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
            ))
            .play_until_no_enemies_remain()
        );
    }

    #[test]
    fn combat_example_6() {
        assert_eq!(
            18_740,
            Grid::from(&read_str_to_lines(
                "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
            ))
            .play_until_no_enemies_remain()
        );
    }

    #[test] // pretty slow at 16s
    fn part1() {
        assert_eq!(
            207_059,
            Grid::from(&read_file_to_lines("input/day15.txt")).play_until_no_enemies_remain()
        );
    }

    #[test]
    fn part2_summarized_combat_example1() {
        assert_eq!(
            4_988,
            Grid::from(&read_str_to_lines(
                "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
            ))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
        );
    }

    #[test]
    fn part2_summarized_combat_example2() {
        assert_eq!(
            31_284,
            Grid::from(&read_str_to_lines(
                "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
            ))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
        );
    }

    #[test]
    fn part2_summarized_combat_example3() {
        assert_eq!(
            3_478,
            Grid::from(&read_str_to_lines(
                "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
            ))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
        );
    }

    #[test]
    fn part2_summarized_combat_example4() {
        assert_eq!(
            6_474,
            Grid::from(&read_str_to_lines(
                "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
            ))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
        );
    }

    #[test]
    fn part2_summarized_combat_example5() {
        assert_eq!(
            1_140,
            Grid::from(&read_str_to_lines(
                "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
            ))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
        );
    }

    #[test] // pretty slow at 2min 44s
    fn part2() {
        assert_eq!(
            49_120,
            Grid::from(&read_file_to_lines("input/day15.txt"))
                .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
        );
    }
}
