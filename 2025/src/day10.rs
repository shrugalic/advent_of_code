use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../../2025/input/day10.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

type IndicatorLights = Vec<bool>;
type LightOrJoltageIndex = usize;
type Button = Vec<LightOrJoltageIndex>;
type Buttons = Vec<Button>;
type ButtonIndex = usize;
type JoltageLevel = usize;
type JoltageLevels = Vec<JoltageLevel>;
type Line = (IndicatorLights, Buttons, JoltageLevels);
type ButtonPressCount = usize;

fn solve_part1(input: &str) -> usize {
    parse(input)
        .map(|(diagram, buttons, _)| fewest_button_presses1(diagram, buttons))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .map(|(_, buttons, requirements)| fewest_button_presses2(buttons, requirements))
        .sum()
}

#[derive(PartialEq, Eq)]
struct LightsState {
    buttons_pressed: Vec<ButtonIndex>,
    diagram: IndicatorLights,
}
impl Ord for LightsState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.buttons_pressed
            .len()
            .cmp(&other.buttons_pressed.len())
            .reverse() // fewer button presses is better
    }
}
impl PartialOrd<Self> for LightsState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fewest_button_presses1(target: IndicatorLights, buttons: Buttons) -> usize {
    let mut visited: HashSet<IndicatorLights> = HashSet::new();

    let initial_state = LightsState {
        buttons_pressed: vec![],
        diagram: vec![false; target.len()],
    };
    let mut queue = BinaryHeap::from([initial_state]);

    while let Some(LightsState {
        buttons_pressed,
        diagram,
    }) = queue.pop()
    {
        if visited.contains(&diagram) {
            continue;
        } else {
            visited.insert(diagram.clone());
        }

        // Press each button…
        for (button_idx, button) in buttons.iter().enumerate() {
            // …except the previously pressed one
            if let Some(last_button) = buttons_pressed.last()
                && button_idx == *last_button
            {
                continue;
            }

            // Calculate the next diagram when pressing this button
            let mut next_diagram = diagram.clone();
            for idx in button {
                next_diagram[*idx] = !next_diagram[*idx];
            }

            let mut next_buttons_pressed = buttons_pressed.clone();
            next_buttons_pressed.push(button_idx);

            // Check if we've reached the goal
            if next_diagram == target {
                println!(
                    "Reached goal {target:?} by pressing the {} buttons {next_buttons_pressed:?}",
                    buttons_pressed.len() + 1
                );
                return buttons_pressed.len() + 1;
            }

            // Queue the next state if it is new
            if !visited.contains(&next_diagram) {
                queue.push(LightsState {
                    buttons_pressed: next_buttons_pressed,
                    diagram: next_diagram,
                })
            }
        }
    }
    unreachable!()
}

#[derive(PartialEq, Eq)]
struct JoltagesState {
    pressed_buttons: Vec<ButtonIndex>,
    available_buttons: Vec<ButtonIndex>,
    joltages: Vec<JoltageLevel>,
}
impl Ord for JoltagesState {
    fn cmp(&self, other: &Self) -> Ordering {
        let prioritize_button_presses = true;
        if prioritize_button_presses {
            // Prefer fewer button presses, then higher joltage totals
            match self.pressed_buttons.len().cmp(&other.pressed_buttons.len()) {
                Ordering::Equal => self
                    .joltages
                    .iter()
                    .sum::<usize>()
                    .cmp(&other.joltages.iter().sum()),
                fewer_button_presses_is_better => fewer_button_presses_is_better.reverse(),
            }
        } else {
            // Prefer higher joltages, then fewer button presses
            match self
                .joltages
                .iter()
                .sum::<usize>()
                .cmp(&other.joltages.iter().sum())
            {
                Ordering::Equal => self
                    .pressed_buttons
                    .len()
                    .cmp(&other.pressed_buttons.len())
                    .reverse(),
                higher_joltage_is_better => higher_joltage_is_better,
            }
        }
    }
}
impl PartialOrd<Self> for JoltagesState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fewest_button_presses2(buttons: Buttons, goal: JoltageLevels) -> usize {
    let mut queued: HashSet<JoltageLevels> = HashSet::new();
    println!("===== goal {goal:?} =====");
    let initial_state = JoltagesState {
        pressed_buttons: vec![],
        available_buttons: (0..buttons.len()).collect(),
        joltages: vec![0; goal.len()],
    };
    let mut queue = BinaryHeap::from([initial_state]);

    let mut counter = 0usize;
    while let Some(JoltagesState {
        pressed_buttons,
        available_buttons,
        joltages,
    }) = queue.pop()
    {
        // Check if we've reached the goal
        if joltages == goal {
            println!(
                "Reached goal {goal:?} by pressing {} buttons {pressed_buttons:?}",
                pressed_buttons.len()
            );
            return pressed_buttons.len();
        }

        // println!(
        //     "{counter}: Visiting joltages {joltages:?} with available {available_buttons:?} by having pressed {} buttons {pressed_buttons:?}",
        //     pressed_buttons.len()
        // );

        // Press each available button…
        let next: Vec<_> = available_buttons
            .iter()
            .map(|&button_idx| {
                // Calculate the next joltages when pressing this button
                let mut next_joltages = joltages.clone();
                for idx in &buttons[button_idx] {
                    next_joltages[*idx] += 1;
                }

                let mut next_buttons_pressed = pressed_buttons.clone();
                next_buttons_pressed.push(button_idx);

                let unavailable_button = next_joltages
                    .iter()
                    .zip(goal.iter())
                    .any(|(current_joltage, goal_joltage)| current_joltage > goal_joltage)
                    .then_some(button_idx);

                (next_joltages, next_buttons_pressed, unavailable_button)
            })
            .collect();

        // Remove buttons that resulted in the next joltage being too high from the available ones
        let mut next_available_buttons = available_buttons;
        for unavailable_button in next
            .iter()
            .filter_map(|(_, _, unavailable_button)| *unavailable_button)
        {
            let removal_idx = next_available_buttons
                .iter()
                .position(|i| i == &unavailable_button)
                .unwrap();
            // println!("Removing unavailable button {unavailable_button} at {removal_idx}");
            next_available_buttons.remove(removal_idx);
        }
        for (next_joltages, next_buttons_pressed, unavailable_button) in next {
            // Skip if the next joltage is too high
            if unavailable_button.is_some() {
                // println!("Dropping {next_joltages:?} > {goal:?}");
                continue;
            }

            // Queue the next state if it is new
            if !queued.contains(&next_joltages) {
                queued.insert(next_joltages.clone());

                // println!("Putting {next_joltages:?} in the queue");
                queue.push(JoltagesState {
                    pressed_buttons: next_buttons_pressed,
                    available_buttons: next_available_buttons.clone(),
                    joltages: next_joltages,
                })
            }
        }
    }
    unreachable!()
}

fn parse(input: &str) -> impl Iterator<Item = Line> {
    input.trim().lines().map(|line| {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        let diagram = parts.remove(0);
        let requirements = parts.remove(parts.len() - 1);

        let diagram: Vec<_> = diagram
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect();

        let buttons: Vec<_> = parts
            .into_iter()
            .map(|button| {
                button
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let requirements = requirements
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (diagram, buttons, requirements)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_part1_example() {
        assert_eq!(7, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(542, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(33, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2_line1() {
        assert_eq!(
            32,
            solve_part2("[.#.#] (0,2,3) (1,3) (2,3) (0,1,2) (0) {31,4,31,29}")
        );
    }

    #[test]
    fn test_part2_line2() {
        assert_eq!(
            1, // unknown
            solve_part2(
                "[#..#..##.#] (1,2,3,4,5,6,7,8,9) (2,5,6,7) (0,1,3,5,7,8) (0,2,3,5,6,8,9) (0,1,3,5,6,7,8,9) (4,7) (3,5,7) (4,6) (1,2,4) (0,1,2,4,5,7,8,9) {34,50,61,55,68,80,58,88,50,48}"
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(1 /* unknown */, solve_part2(INPUT));
    }
}
