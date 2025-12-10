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

#[derive(PartialEq, Eq, Debug)]
struct JoltagesState {
    button_press_counts: Vec<usize>,
    available_button_presses: Vec<usize>,
    remaining_joltages: Vec<JoltageLevel>,
}
impl Ord for JoltagesState {
    fn cmp(&self, other: &Self) -> Ordering {
        match self
            .remaining_joltages
            .iter()
            .sum::<usize>()
            .cmp(&other.remaining_joltages.iter().sum()).reverse() // Lower is better
        {
            Ordering::Equal => self
                .button_press_counts
                .iter()
                .sum::<usize>()
                .cmp(&other.button_press_counts.iter().sum()), // Fewer is better
            use_joltage_ordering => use_joltage_ordering,
        }
    }
}
impl PartialOrd<Self> for JoltagesState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fewest_button_presses2(buttons: Buttons, target: JoltageLevels) -> usize {
    // Let's call this the remaining joltages, assume the target is to reach all zeroes,
    // and that each button press reduces its assigned joltages by 1
    let remaining_joltages = target;

    // The maximum number of times each button may be pressed
    // (because otherwise the joltage would be too high)
    let available_button_presses: Vec<_> = buttons
        .iter()
        .flat_map(|button| button.iter().map(|&ji| remaining_joltages[ji]).min())
        .collect();

    let initial = JoltagesState {
        button_press_counts: vec![0; buttons.len()],
        available_button_presses,
        remaining_joltages,
    };

    let mut queue = BinaryHeap::from([initial]);
    while let Some(JoltagesState {
        button_press_counts,
        available_button_presses,
        remaining_joltages,
    }) = queue.pop()
    {
        if remaining_joltages.iter().all(|&j| j == 0) {
            return button_press_counts.iter().sum();
        }

        let j_idx = remaining_joltages.iter().position(|j| j > &0).unwrap();
        let joltage_target = remaining_joltages[j_idx];
        let button_idxs: Vec<_> = buttons
            .iter()
            .enumerate()
            .filter_map(|(b_idx, buttons)| buttons.contains(&j_idx).then_some(b_idx))
            .collect();

        'distribution_loop: for distribution in list_all_valid_button_press_distributions(
            joltage_target,
            &button_idxs,
            &available_button_presses,
        ) {
            let mut next_remaining_joltages = remaining_joltages.clone();
            let mut next_button_press_counts = button_press_counts.clone();
            for (b_idx, press_count) in distribution.into_iter().enumerate() {
                next_button_press_counts[b_idx] += press_count;
                for j_idx in &buttons[b_idx] {
                    if press_count > next_remaining_joltages[*j_idx] {
                        continue 'distribution_loop;
                    }
                    next_remaining_joltages[*j_idx] -= press_count;
                }
            }

            let next_available_button_presses = buttons
                .iter()
                .flat_map(|button| {
                    button
                        .iter()
                        .map(|&j_idx| next_remaining_joltages[j_idx])
                        .min()
                })
                .collect();

            let next = JoltagesState {
                button_press_counts: next_button_press_counts,
                available_button_presses: next_available_button_presses,
                remaining_joltages: next_remaining_joltages,
            };
            queue.push(next);
        }
    }

    unreachable!()
}

// Lists all possible ways to distribute a given total number of button presses
// to different buttons, given a constraint of `available_button_presses`.
// This may return an empty list if impossible
fn list_all_valid_button_press_distributions(
    total: usize,
    available_buttons_ids: &[ButtonIndex],
    available_button_presses: &[usize],
) -> Vec<Vec<usize>> {
    debug_assert!(!available_buttons_ids.is_empty());

    let own_id = available_buttons_ids[0];
    let max_usable = usize::min(available_button_presses[own_id], total);

    if available_buttons_ids.len() == 1 {
        // Must use the full total for this button
        if available_button_presses[own_id] < total {
            return vec![]; // Cannot fulfill the request
        }
        let mut own_uses = vec![0; available_button_presses.len()];
        own_uses[own_id] = max_usable;
        vec![own_uses]
    } else {
        let mut all_uses: Vec<Vec<usize>> = vec![];

        // Can use max or less for this button
        let next_available_button_ids: Vec<_> = available_buttons_ids
            .iter()
            .filter(|&bid| bid != &own_id)
            .copied()
            .collect();
        for use_count in 0..=max_usable {
            // available_button_presses may no longer be accurate for this button,
            // but this button will not be used because it's no longer available
            let mut remaining_uses = list_all_valid_button_press_distributions(
                total - use_count,
                &next_available_button_ids,
                available_button_presses,
            );
            // Add own uses to the result
            for used in &mut remaining_uses {
                used[own_id] = use_count;
            }
            all_uses.append(&mut remaining_uses);
        }
        all_uses
    }
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
            32, // according to both first and second attempt
            solve_part2("[.#.#] (0,2,3) (1,3) (2,3) (0,1,2) (0) {31,4,31,29}")
        );
    }

    #[test]
    fn test_part2_line2() {
        assert_eq!(
            119, // according to second attempt (first was too slow)
            solve_part2(
                "[#..#..##.#] (1,2,3,4,5,6,7,8,9) (2,5,6,7) (0,1,3,5,7,8) (0,2,3,5,6,8,9) (0,1,3,5,6,7,8,9) (4,7) (3,5,7) (4,6) (1,2,4) (0,1,2,4,5,7,8,9) {34,50,61,55,68,80,58,88,50,48}"
            )
        );
    }

    #[test]
    fn test_part2_line50() {
        assert_eq!(
            1, // unknown, second attempt still too slow
            solve_part2(
                "[##.#.#.##.] (3,4,6) (0,1,4,6,7,8,9) (0,1,2,5,7,9) (1,3,7,8,9) (1,2,3,4,5,7,8,9) (0,1,2,3,4,7) (1,2,3,8) (0,1,2,4,5,8) (1,3,6,9) (1,3,7,9) (3,5) (0,1,2,3,4,6,7,8,9) (0,1,4,5,8,9) {167,207,67,89,174,32,150,167,166,169}"
            )
        );
    }

    #[test]
    fn test_distribute_1_item_with_equal_total_than_available_presses() {
        let total = 5;
        let available_ids = vec![0];
        let available_presses = vec![5];
        assert_eq!(
            vec![vec![5]],
            list_all_valid_button_press_distributions(total, &available_ids, &available_presses)
        );
    }

    #[test]
    fn test_distribute_1_item_with_less_total_than_available_presses() {
        let total = 3;
        let available_ids = vec![0];
        let available_presses = vec![5];
        assert_eq!(
            vec![vec![3]],
            list_all_valid_button_press_distributions(total, &available_ids, &available_presses)
        );
    }

    #[test]
    fn test_distribute_1_item_with_more_total_than_available_presses() {
        let total = 5;
        let available_ids = vec![0];
        let available_presses = vec![3];
        let expected: Vec<Vec<usize>> = vec![];
        assert_eq!(
            expected,
            list_all_valid_button_press_distributions(total, &available_ids, &available_presses)
        );
    }

    #[test]
    fn test_distribute_2_items_unlimited() {
        let total = 3;
        let available_ids = vec![0, 1];
        let available_presses = vec![3, 3];
        assert_eq!(
            vec![vec![0, 3], vec![1, 2], vec![2, 1], vec![3, 0]],
            list_all_valid_button_press_distributions(total, &available_ids, &available_presses)
        );
    }

    #[test]
    fn test_distribute_2_items_with_some_limits() {
        let total = 5;
        let available_ids = vec![0, 1];
        let available_presses = vec![5, 3];
        assert_eq!(
            vec![vec![2, 3], vec![3, 2], vec![4, 1], vec![5, 0]],
            list_all_valid_button_press_distributions(total, &available_ids, &available_presses)
        );
    }

    #[test]
    fn test_distribute_3_items() {
        let total = 4;
        let available_ids = vec![0, 1, 2];
        let available_presses = vec![4, 4, 3];
        assert_eq!(
            vec![
                vec![0, 1, 3],
                vec![0, 2, 2],
                vec![0, 3, 1],
                vec![0, 4, 0],
                vec![1, 0, 3],
                vec![1, 1, 2],
                vec![1, 2, 1],
                vec![1, 3, 0],
                vec![2, 0, 2],
                vec![2, 1, 1],
                vec![2, 2, 0],
                vec![3, 0, 1],
                vec![3, 1, 0],
                vec![4, 0, 0],
            ],
            list_all_valid_button_press_distributions(total, &available_ids, &available_presses)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(1 /* unknown */, solve_part2(INPUT));
    }
}
