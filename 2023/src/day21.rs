use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::iter;
use std::ops::Add;

use GridType::*;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT, 64)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT, 26_501_365)
}

fn solve_part1(input: &'static str, max_steps: usize) -> usize {
    let rock_map = RockMap::from(input);
    let mut grid = Grid::new_single_at_start(&rock_map);
    for step in 1..=max_steps {
        grid.step(step);
        // println!("t = {step}:\n{grid}\n");
    }
    grid.count_total_plots_visited(max_steps)
}

fn solve_part2(input: &'static str, max_steps: usize) -> usize {
    let rock_map = RockMap::from(input);

    // The original grid starts with step_count 0 at the rock map's start position
    let step_count = 0;
    let mut grid = Grid::new_empty_infinite(&rock_map, step_count);
    let initial_frontier: HashSet<Position> = HashSet::from_iter(iter::once(rock_map.start));
    grid.add_frontier(initial_frontier, step_count);

    // The original grid starts at offset (0, 0), and it's neighbors spread from there
    let mut grids_by_offset: HashMap<GridOffset, Grid> = HashMap::new();
    let origin: GridOffset = Position::default();
    grids_by_offset.insert(origin, grid);

    // Outer grids start having the exact same spread pattern ("history") as inner grids.
    // Remember offsets of repeated grids
    let mut reference_grid_offsets_by_history: HashMap<String, Vec<GridOffset>> = HashMap::new();

    // Track when we reach a point when all new grid histories repeat. Grids spread in a diamond shape,
    // and the smallest diamond is made up of 4 grids in a cross-wise pattern around the center grid.
    // The one further out is made up of 8 grids, the next of 12, then 16, and so forth.
    // This vec holds counts, its index is distance from center minus 1, so at index 0 we could have max 4 grids
    let mut repeating_grid_count_by_distance_from_center = vec![];

    let mut step_count = 0;
    while step_count < max_steps {
        step_count += 1;
        let mut all_transfers: HashMap<GridOffset, HashSet<Position>> = HashMap::new();

        // Step all grids
        for (offset, grid) in grids_by_offset
            .iter_mut()
            .filter(|(_, grid)| !grid.is_done())
        {
            let transfers: HashMap<GridOffset, HashSet<Position>> = grid.step(step_count);
            if grid.is_done() {
                reference_grid_offsets_by_history
                    .entry(grid.history_string())
                    .and_modify(|offsets| {
                        let distance = offset.manhattan_distance();
                        while repeating_grid_count_by_distance_from_center.len() < distance {
                            repeating_grid_count_by_distance_from_center.push(0);
                        }
                        repeating_grid_count_by_distance_from_center[distance - 1] += 1;
                        offsets.push(*offset);
                    })
                    .or_insert(vec![*offset]);
            }
            // Gather paths that escaped from the current grid to add transfer to neighboring grids
            for (transfer_offset, frontier) in transfers {
                let offset = *offset + transfer_offset;
                all_transfers.entry(offset).or_default().extend(frontier);
            }
        }

        // Transfer escaped paths to neighboring grids
        for (offset, frontier) in all_transfers {
            let neighbor_grid = grids_by_offset
                .entry(offset)
                .or_insert(Grid::new_empty_infinite(&rock_map, step_count));
            neighbor_grid.add_frontier(frontier, step_count);
        }

        // save_to_file(&rock_map, step_count, &grids_by_offset);
        // print_grids(&rock_map, step_count, &grids_by_pos);

        if repeating_grid_count_by_distance_from_center
            .iter()
            .enumerate()
            .map(|(i, count)| (i + 1, count))
            .any(|(distance, &count)| count == distance * 4)
        {
            // save_to_file(&rock_map, step_count, &grids_by_offset);

            // At this point we have a wave of grids spreading in a diamond pattern.
            // The ones at `distance` from center are full / in steady state
            // Grids further out will repeat this pattern, so let's calculate how many there will be after the step count
            let ref_grid_offsets: Vec<_> = reference_grid_offsets_by_history
                .values()
                .filter(|offsets| offsets.len() > 1)
                .map(|offsets| offsets[0])
                .collect();
            return visited_plot_count(max_steps, &mut grids_by_offset, ref_grid_offsets);
        }
    }
    // print_grids(&rock_map, step_count, &grids_by_offset);
    save_to_file(&rock_map, step_count, &grids_by_offset);
    // Sum of the totals
    grids_by_offset
        .into_values()
        .map(|grid| grid.count_total_plots_visited(max_steps))
        .sum()
}

fn visited_plot_count(
    max_steps: usize,
    grids_by_offset: &mut HashMap<GridOffset, Grid>,
    ref_grid_offsets: Vec<GridOffset>,
) -> usize {
    let ref_grid_where = |filter: fn(&GridOffset) -> bool| {
        &grids_by_offset[ref_grid_offsets.iter().find(|o| filter(o)).unwrap()]
    };
    let left_ref_grid = ref_grid_where(|o| o.y == 0 && o.x < 0);
    let right_ref_grid = ref_grid_where(|o| o.y == 0 && o.x > 0);
    let top_ref_grid = ref_grid_where(|o| o.y < 0 && o.x == 0);
    let bottom_ref_grid = ref_grid_where(|o| o.y > 0 && o.x == 0);

    let top_left_grid = ref_grid_where(|o| o.y < 0 && o.x < 0);
    let top_right_grid = ref_grid_where(|o| o.y < 0 && o.x > 0);
    let bottom_left_grid = ref_grid_where(|o| o.y > 0 && o.x < 0);
    let bottom_right_grid = ref_grid_where(|o| o.y > 0 && o.x > 0);

    let right_ref_grid_offset = ref_grid_offsets
        .iter()
        .find(|o| o.y == 0 && o.x > 0)
        .unwrap();
    let after_right_ref_grid = &grids_by_offset[&Position::new(right_ref_grid_offset.x + 1, 0)];
    let velocity = after_right_ref_grid.first_step - right_ref_grid.first_step;

    let top_left_start = top_left_grid.first_step;
    let top_right_start = top_right_grid.first_step;
    let bottom_left_start = bottom_left_grid.first_step;
    let bottom_right_start = bottom_right_grid.first_step;

    let top_left_outer_partial_steps = (max_steps - top_left_start) % velocity;
    let top_right_outer_partial_steps = (max_steps - top_right_start) % velocity;
    let bottom_left_outer_partial_steps = (max_steps - bottom_left_start) % velocity;
    let bottom_right_outer_partial_steps = (max_steps - bottom_right_start) % velocity;

    let top_left_inner_partial_steps = top_left_outer_partial_steps + velocity;
    let top_right_inner_partial_steps = top_right_outer_partial_steps + velocity;
    let bottom_left_inner_partial_steps = bottom_left_outer_partial_steps + velocity;
    let bottom_right_inner_partial_steps = bottom_right_outer_partial_steps + velocity;

    let num_top_left_grids = 1 + (max_steps - top_left_start) / velocity;
    let num_top_right_grids = 1 + (max_steps - top_right_start) / velocity;
    let num_bottom_left_grids = 1 + (max_steps - bottom_left_start) / velocity;
    let num_bottom_right_grids = 1 + (max_steps - bottom_right_start) / velocity;

    let num_top_left_full_grids = num_top_left_grids - 2;
    let num_top_right_full_grids = num_top_right_grids - 2;
    let num_bottom_left_full_grids = num_bottom_left_grids - 2;
    let num_bottom_right_full_grids = num_bottom_right_grids - 2;

    let num_top_left_inner_partial_grids = num_top_left_grids - 1;
    let num_top_right_inner_partial_grids = num_top_right_grids - 1;
    let num_bottom_left_inner_partial_grids = num_bottom_left_grids - 1;
    let num_bottom_right_inner_partial_grids = num_bottom_right_grids - 1;

    let num_top_left_outer_partial_grids = num_top_left_grids;
    let num_top_right_outer_partial_grids = num_top_right_grids;
    let num_bottom_left_outer_partial_grids = num_bottom_left_grids;
    let num_bottom_right_outer_partial_grids = num_bottom_right_grids;

    let grids_on_axis =
        right_ref_grid_offset.x as StepCount + (max_steps - left_ref_grid.first_step) / velocity;
    let full_grids_on_axis = grids_on_axis - 2; // The outer 1-2 are partial
    let axis_outer_partial_steps = (max_steps - left_ref_grid.first_step) % velocity;
    let axis_inner_partial_steps = axis_outer_partial_steps + velocity;
    // println!("grids_on_axis {grids_on_axis}, full_grids_on_axis {full_grids_on_axis}, axis_inner_partial_step {axis_inner_partial_steps}, axis_outer_partial_step {axis_outer_partial_steps}");
    // println!("top_left_inner_partial_steps {top_left_inner_partial_steps}, top_right_inner_partial_steps {top_right_inner_partial_steps}, bottom_left_inner_partial_steps {bottom_left_inner_partial_steps}, bottom_right_inner_partial_steps {bottom_right_inner_partial_steps}");
    // println!("top_left_outer_partial_steps {top_left_outer_partial_steps}, top_right_outer_partial_steps {top_right_outer_partial_steps}, bottom_left_outer_partial_steps {bottom_left_outer_partial_steps}, bottom_right_outer_partial_steps {bottom_right_outer_partial_steps}");
    // println!("top_left_grids {num_top_left_grids}, top_right_grids {num_top_right_grids}, bottom_left_grids {num_bottom_left_grids}, bottom_right_grids {num_bottom_right_grids}");
    // println!("top_left_full_grids {num_top_left_full_grids}, top_right_full_grids {num_top_right_full_grids}, bottom_left_full_grids {num_bottom_left_full_grids}, bottom_right_full_grids {num_bottom_right_full_grids}");
    // println!("num_top_left_inner_partial_grids {num_top_left_inner_partial_grids}, num_top_right_inner_partial_grids {num_top_right_inner_partial_grids}, num_bottom_left_inner_partial_grids {num_bottom_left_inner_partial_grids}, num_bottom_right_inner_partial_grids {num_bottom_right_inner_partial_grids}");
    // println!("num_top_left_outer_partial_grids {num_top_left_outer_partial_grids}, num_top_right_outer_partial_grids {num_top_right_outer_partial_grids}, num_bottom_left_outer_partial_grids {num_bottom_left_outer_partial_grids}, num_bottom_right_outer_partial_grids {num_bottom_right_outer_partial_grids}");

    let only_evens = |full| full / 2; // 4 -> 2, 5 -> 2
    let only_odds = |full| (full + 1) / 2; // 4 -> 2, 5 -> 3

    // O
    // EO
    // OEO
    // EOEO
    // OEOEO
    // This counts the O tiles, as seen diagonally: 1 + 3 + 5
    let count_stair_tiles_like_corner = |base_width| only_odds(base_width) * only_odds(base_width);
    // This counts the E tiles, as seen diagonally: 2 + 4
    let count_non_corner_stair_tiles =
        |base_width| only_evens(base_width) * (only_evens(base_width) + 1);

    // count grids
    let num_center_grids = 1;
    let num_full_even_grids = num_center_grids
        + 4 * only_evens(full_grids_on_axis)
        + count_stair_tiles_like_corner(num_top_left_full_grids)
        + count_stair_tiles_like_corner(num_top_right_full_grids)
        + count_stair_tiles_like_corner(num_bottom_left_full_grids)
        + count_stair_tiles_like_corner(num_bottom_right_full_grids);
    let num_full_odd_grids = 4 * only_odds(full_grids_on_axis)
        + count_non_corner_stair_tiles(num_top_left_full_grids)
        + count_non_corner_stair_tiles(num_top_right_full_grids)
        + count_non_corner_stair_tiles(num_bottom_left_full_grids)
        + count_non_corner_stair_tiles(num_bottom_right_full_grids);
    // let num_axis_inner_partial_grids = 4;
    // let num_axis_outer_partial_grids = 4;
    // let num_total_inner_partial_grids = num_top_left_inner_partial_grids
    //     + num_top_right_inner_partial_grids
    //     + num_bottom_left_inner_partial_grids
    //     + num_bottom_right_inner_partial_grids;
    // let num_total_outer_partial_grids = num_top_left_outer_partial_grids
    //     + num_top_right_outer_partial_grids
    //     + num_bottom_left_outer_partial_grids
    //     + num_bottom_right_outer_partial_grids;
    // let total_grid_count = num_full_even_grids
    //     + num_full_odd_grids
    //     + num_axis_inner_partial_grids
    //     + num_axis_outer_partial_grids
    //     + num_total_inner_partial_grids
    //     + num_total_outer_partial_grids;
    // total_grid_count might be useful for debugging

    // count plots
    let origin: GridOffset = Position::default();
    let even_grid = &grids_by_offset[&origin];
    let odd_grid = &grids_by_offset[&Position::new(1, 0)];

    num_full_even_grids * even_grid.count_total_plots_visited(max_steps)
        + num_full_odd_grids * odd_grid.count_total_plots_visited(max_steps)
        // quadrant inner partial
        + num_top_left_inner_partial_grids * top_left_grid.count_partial_plots_visited(top_left_inner_partial_steps)
        + num_top_right_inner_partial_grids * top_right_grid.count_partial_plots_visited(top_right_inner_partial_steps)
        + num_bottom_left_inner_partial_grids * bottom_left_grid.count_partial_plots_visited(bottom_left_inner_partial_steps)
        + num_bottom_right_inner_partial_grids * bottom_right_grid.count_partial_plots_visited(bottom_right_inner_partial_steps)
        // quadrant outer partial
        + num_top_left_outer_partial_grids * top_left_grid.count_partial_plots_visited(top_left_outer_partial_steps)
        + num_top_right_outer_partial_grids * top_right_grid.count_partial_plots_visited(top_right_outer_partial_steps)
        + num_bottom_left_outer_partial_grids * bottom_left_grid.count_partial_plots_visited(bottom_left_outer_partial_steps)
        + num_bottom_right_outer_partial_grids * bottom_right_grid.count_partial_plots_visited(bottom_right_outer_partial_steps)
        // axes inner partial
        + left_ref_grid.count_partial_plots_visited(axis_inner_partial_steps)
        + right_ref_grid.count_partial_plots_visited(axis_inner_partial_steps)
        + top_ref_grid.count_partial_plots_visited(axis_inner_partial_steps)
        + bottom_ref_grid.count_partial_plots_visited(axis_inner_partial_steps)
        // axes outer partial
        + left_ref_grid.count_partial_plots_visited(axis_outer_partial_steps)
        + right_ref_grid.count_partial_plots_visited(axis_outer_partial_steps)
        + top_ref_grid.count_partial_plots_visited(axis_outer_partial_steps)
        + bottom_ref_grid.count_partial_plots_visited(axis_outer_partial_steps)
}

#[allow(dead_code)]
fn print_grids(rock_map: &RockMap, step_count: StepCount, grids_by_pos: &HashMap<Position, Grid>) {
    println!("{}", grids_to_string(rock_map, step_count, grids_by_pos));
}

#[allow(dead_code)]
fn save_to_file(rock_map: &RockMap, step_count: StepCount, grids_by_pos: &HashMap<Position, Grid>) {
    let mut file = File::create(format!(
        "grid_{}_time_{step_count}.txt",
        rock_map.boundary.x
    ))
    .unwrap();
    file.write_all(grids_to_string(rock_map, step_count, grids_by_pos).as_bytes())
        .unwrap();
}

fn grids_to_string(
    rock_map: &RockMap,
    step_count: StepCount,
    grids_by_pos: &HashMap<Position, Grid>,
) -> String {
    let empty_infinite = Grid::new_empty_infinite(rock_map, step_count);
    let mut result = format!("t = {step_count}\n");
    let min_x_offset = grids_by_pos.keys().map(|pos| pos.x).min().unwrap();
    let max_x_offset = grids_by_pos.keys().map(|pos| pos.x).max().unwrap();
    let min_y_offset = grids_by_pos.keys().map(|pos| pos.y).min().unwrap();
    let max_y_offset = grids_by_pos.keys().map(|pos| pos.y).max().unwrap();
    for o_y in min_y_offset..=max_y_offset {
        for y in 0..=rock_map.boundary.y {
            for o_x in min_x_offset..=max_x_offset {
                let grid = grids_by_pos
                    .get(&Position::new(o_x, o_y))
                    .unwrap_or(&empty_infinite);
                result.push_str(
                    &(0..=rock_map.boundary.x)
                        .map(|x| Position::new(x as Coord, y as Coord))
                        .map(|pos| {
                            if grid.rock_map.rock_at_pos(&pos) {
                                '#'
                            } else if let Some(time) = grid.history.get(&pos) {
                                if (grid.first_step + time) % 2 == 0 {
                                    'x'
                                } else {
                                    'o'
                                }
                            } else if pos.x == grid.rock_map.boundary.x
                                && pos.y == grid.rock_map.boundary.y
                            {
                                '+'
                            } else if pos.x == grid.rock_map.boundary.x {
                                '|'
                            } else if pos.y == grid.rock_map.boundary.y {
                                '-'
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>(),
                );
            }
            result.push('\n');
        }
    }
    result
}

#[derive(PartialEq, Debug, Clone)]
enum GridType {
    Single,
    Infinite,
}

type Coord = i32;
type StepCount = usize;
type GridOffset = Position;
#[derive(Debug)]
struct RockMap {
    rocks: Vec<Vec<bool>>, // `true` if there's a rock at index [y][x]
    start: Position,
    boundary: Position,
}

#[derive(Debug, Clone)]
struct Grid<'a> {
    rock_map: &'a RockMap,
    grid_type: GridType,
    first_step: StepCount, // When the first plot was occupied, in global "time"
    last_step: Option<StepCount>, // When the last plot was occupied, in global "time"
    // Position by the step count when it was first reached (it will be re-visited every other step)
    history: HashMap<Position, StepCount>,
}

impl<'a> Grid<'a> {
    fn new_single_at_start(rock_map: &'a RockMap) -> Self {
        let step_count = 0;
        Grid {
            rock_map,
            grid_type: Single,
            history: HashMap::from_iter(iter::once((rock_map.start, step_count))),
            first_step: step_count,
            last_step: None,
        }
    }
    fn new_empty_infinite(rock_map: &'a RockMap, step_count: StepCount) -> Self {
        Grid {
            rock_map,
            grid_type: Infinite,
            history: HashMap::new(),
            first_step: step_count,
            last_step: None,
        }
    }
    fn add_frontier(&mut self, frontier: HashSet<Position>, global_step: StepCount) {
        for pos in frontier {
            self.history
                .entry(pos)
                .or_insert(global_step - self.first_step);
        }
    }
    fn step(&mut self, global_step: usize) -> HashMap<Position, HashSet<Position>> {
        let step = global_step - self.first_step;
        let prev_step = step - 1;
        let prev_frontier: HashSet<_> = self
            .history
            .iter()
            .filter_map(|(pos, time)| (time == &prev_step).then_some(pos))
            .collect();

        let mut transfers: HashMap<Position, HashSet<Position>> = HashMap::new();
        let new_frontier: HashSet<Position> = prev_frontier
            .iter()
            .flat_map(|current| current.neighbors(self.rock_map))
            .filter(|next| !self.history.contains_key(next))
            .filter(|next| {
                if next.is_within(&self.rock_map.boundary) {
                    true
                } else if self.grid_type == Single {
                    false
                } else {
                    // Transfer to neighboring grid
                    let offset = next.grid_offset(&self.rock_map.boundary);
                    transfers
                        .entry(offset)
                        .or_default()
                        .insert(next.normalized_to(&self.rock_map.boundary));
                    false
                }
            })
            .collect();

        if new_frontier
            .iter()
            .flat_map(|next| next.neighbors(self.rock_map))
            .filter(|after_next| after_next.is_within(&self.rock_map.boundary))
            .all(|after_next| self.history.contains_key(&after_next))
        {
            // The next frontier only contains already visited plots, so we're done
            self.last_step = Some(global_step);
        }
        self.history
            .extend(new_frontier.iter().map(|pos| (*pos, step)));

        transfers
    }
    fn is_done(&self) -> bool {
        self.last_step.is_some()
    }
    fn count_total_plots_visited(&self, global_step_count: StepCount) -> usize {
        self.history
            .iter()
            .filter(|(_, &local_step_count)| {
                (self.first_step + local_step_count) % 2 == global_step_count % 2
            })
            .count()
    }
    fn count_partial_plots_visited(&self, partial_step_count: StepCount) -> usize {
        self.history
            .iter()
            .filter(|(_, &local_step_count)| local_step_count <= partial_step_count)
            .filter(|(_, &local_step_count)| local_step_count % 2 == partial_step_count % 2)
            .count()
    }
    #[allow(dead_code)]
    fn format_positions(set: &HashSet<Position>) -> String {
        let mut vec: Vec<_> = set.iter().collect();
        vec.sort_unstable();
        format!(
            "{{{}}}",
            vec.iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
    #[allow(dead_code)]
    fn step_count(&self) -> StepCount {
        *self.history.values().max().unwrap()
    }
    fn history_string(&self) -> String {
        let mut sorted: Vec<_> = self.history.iter().collect();
        sorted.sort_unstable_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        let mut lines = vec![];
        let mut prev_step_count = -1;
        let mut line = String::new();
        for (pos, step_count) in sorted {
            if *step_count as i32 > prev_step_count {
                if !line.is_empty() {
                    lines.push(line);
                }
                line = format!("t = {step_count}: {pos}");
            } else {
                line = format!("{line}, {pos}");
            }
            prev_step_count = *step_count as i32;
        }
        lines.join("\n")
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: Coord,
    y: Coord,
}

impl RockMap {
    fn rock_at_pos(&self, pos: &Position) -> bool {
        let normalized = pos.normalized_to(&self.boundary);
        self.rocks[normalized.y as usize][normalized.x as usize]
    }
}

impl Position {
    fn new(x: Coord, y: Coord) -> Self {
        Position { x, y }
    }
    fn offset(value: Coord, limit: Coord) -> Coord {
        if value < 0 {
            (value - limit) / limit
        } else {
            value / limit
        }
    }
    fn grid_offset(self, boundary: &Position) -> GridOffset {
        Position::new(
            Position::offset(self.x, boundary.x),
            Position::offset(self.y, boundary.y),
        )
    }
    fn is_within(&self, boundary: &Position) -> bool {
        (0..boundary.x).contains(&self.x) && (0..boundary.y).contains(&self.y)
    }
    fn neighbors(self, grid: &RockMap) -> impl Iterator<Item = Position> + '_ {
        [
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
        ]
        .into_iter()
        .filter(|pos| !grid.rock_at_pos(pos))
    }
    fn normalized_to(&self, boundary: &Position) -> Self {
        Position::new(self.x.rem_euclid(boundary.x), self.y.rem_euclid(boundary.y))
    }
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl From<&str> for RockMap {
    fn from(input: &str) -> Self {
        let mut start = Position::new(0, 0);
        let grid: Vec<_> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|(x, c)| {
                        if c == &'S' {
                            start = Position::new(*x as Coord, y as Coord);
                        }
                    })
                    .map(|(_x, c)| c == '#')
                    .collect::<Vec<_>>()
            })
            .collect();
        let boundary = Position::new(grid[0].len() as Coord, grid.len() as Coord);
        RockMap {
            rocks: grid,
            start,
            boundary,
        }
    }
}

impl Display for Grid<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..=self.rock_map.boundary.y)
                .map(|y| (0..=self.rock_map.boundary.x)
                    .map(|x| Position::new(x as Coord, y as Coord))
                    .map(|pos| if self.rock_map.rock_at_pos(&pos) {
                        '#'
                    } else if let Some(time) = self.history.get(&pos) {
                        if (self.first_step + time) % 2 == 0 {
                            'x'
                        } else {
                            'o'
                        }
                    } else if pos.x == self.rock_map.boundary.x && pos.y == self.rock_map.boundary.y
                    {
                        '+'
                    } else if pos.x == self.rock_map.boundary.x {
                        '|'
                    } else if pos.y == self.rock_map.boundary.y {
                        '-'
                    } else {
                        '.'
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for RockMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rocks
                .iter()
                .enumerate()
                .map(|(y, row)| row
                    .iter()
                    .enumerate()
                    .map(
                        |(x, b)| if self.start == Position::new(x as Coord, y as Coord) {
                            'S'
                        } else if *b {
                            '#'
                        } else {
                            '.'
                        }
                    )
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_part1_example() {
        assert_eq!(2, solve_part1(EXAMPLE, 1));
        assert_eq!(4, solve_part1(EXAMPLE, 2));
        assert_eq!(6, solve_part1(EXAMPLE, 3));
        assert_eq!(9, solve_part1(EXAMPLE, 4));
        assert_eq!(13, solve_part1(EXAMPLE, 5));
        assert_eq!(16, solve_part1(EXAMPLE, 6));
    }

    #[test]
    fn test_part1_example_parse_and_to_string() {
        let rock_map = RockMap::from(EXAMPLE);
        assert_eq!(EXAMPLE.trim(), format!("{rock_map}"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(3_671, solve_part1(INPUT, 64));
    }

    #[test]
    fn test_offset() {
        let max = 10;
        assert_eq!(0, Position::offset(0, max));
        assert_eq!(0, Position::offset(5, max));
        assert_eq!(0, Position::offset(9, max));

        assert_eq!(1, Position::offset(10, max));
        assert_eq!(1, Position::offset(15, max));
        assert_eq!(1, Position::offset(19, max));

        assert_eq!(2, Position::offset(20, max));
        assert_eq!(2, Position::offset(25, max));

        assert_eq!(-1, Position::offset(-1, max));
        assert_eq!(-1, Position::offset(-5, max));
        assert_eq!(-1, Position::offset(-9, max));

        assert_eq!(-2, Position::offset(-10, max));
        assert_eq!(-2, Position::offset(-15, max));
    }

    #[test]
    fn test_normalize() {
        let boundary = Position::new(10, 10);
        let normalized = |pos: Position| pos.normalized_to(&boundary);

        assert_eq!(Position::new(0, 0), normalized(Position::new(0, 0)));
        assert_eq!(Position::new(9, 9), normalized(Position::new(9, 9)));

        assert_eq!(Position::new(0, 0), normalized(Position::new(10, 10)));
        assert_eq!(Position::new(9, 9), normalized(Position::new(19, 19)));

        assert_eq!(Position::new(0, 0), normalized(Position::new(-10, -10)));
        assert_eq!(Position::new(9, 9), normalized(Position::new(-1, -1)));
    }

    #[test]
    fn test_part2_example() {
        // my own
        assert_eq!(22, solve_part2(EXAMPLE, 7)); // 8A 13C 1T
        assert_eq!(30, solve_part2(EXAMPLE, 8)); // 10F 7P 9O 4T
        assert_eq!(41, solve_part2(EXAMPLE, 9)); // 18F 9P 13C 1T
        assert_eq!(63, solve_part2(EXAMPLE, 11)); // 21F, 19P 22C
        assert_eq!(1653, solve_part2(EXAMPLE, 51));
        assert_eq!(1914, solve_part2(EXAMPLE, 55));
        assert_eq!(3467, solve_part2(EXAMPLE, 73));

        // AOC
        assert_eq!(16, solve_part2(EXAMPLE, 6));
        assert_eq!(50, solve_part2(EXAMPLE, 10));
        assert_eq!(1594, solve_part2(EXAMPLE, 50));
        assert_eq!(6536, solve_part2(EXAMPLE, 100));
        assert_eq!(167_004, solve_part2(EXAMPLE, 500));
        assert_eq!(668_697, solve_part2(EXAMPLE, 1000));
        assert_eq!(16_733_044, solve_part2(EXAMPLE, 5000)); // 17.22s with history
    }

    #[test]
    fn test_part2() {
        // my own; x-count according to rip grep (o-count 371261)
        assert_eq!(372_380, solve_part2(INPUT, 654));
        // AOC
        assert_eq!(609_708_004_316_870, solve_part2(INPUT, 26_501_365));
    }
}
