use std::collections::HashSet;

type Index = u8;
type Step = char;
type Requirement = Step;
type FollowUp = Step;
type Instruction = (Requirement, FollowUp);
type RequiredIndices = Vec<Index>;
type WorkRemaining = u8;
type Worker = (Step, WorkRemaining);

trait ToInstruction {
    fn to_instruction(&self) -> Instruction;
}
impl ToInstruction for String {
    fn to_instruction(&self) -> Instruction {
        if let Some((req, follow_up)) = self.split_once(" must be finished before step ") {
            (
                req.trim_start_matches("Step ").to_step(),
                follow_up.trim_end_matches(" can begin.").to_step(),
            )
        } else {
            panic!("Illegal requirement {}", self);
        }
    }
}

trait ToIndex {
    fn to_index(&self) -> Index;
}
impl ToIndex for Step {
    fn to_index(&self) -> Index {
        *self as u8 - b'A'
    }
}
trait ToStep {
    fn to_step(&self) -> Step;
}

impl ToStep for Index {
    fn to_step(&self) -> Step {
        (self + b'A') as char
    }
}
impl ToStep for &str {
    fn to_step(&self) -> Step {
        self.chars().next().unwrap()
    }
}

pub fn order_of_steps(input: &[String]) -> String {
    order_and_duration(input, 1, 0).0
}
pub fn count_seconds(input: &[String], worker_count: u8, base_duration: u8) -> usize {
    order_and_duration(input, worker_count, base_duration).1
}

pub fn order_and_duration(
    input: &[String],
    worker_count: u8,
    base_duration: u8,
) -> (String, usize) {
    let instructions: Vec<Instruction> = input.iter().map(String::to_instruction).collect();
    // println!("{} instructions", instructions.len());
    let unique_steps: HashSet<Step> = instructions
        .iter()
        .flat_map(|(req, follow_up)| vec![*req, *follow_up])
        .collect();
    // println!("{} unique steps", unique_steps.len());

    // Requirements for a given step, where the index of the vec corresponds to a step,
    // which requires the other steps (indices) listed at this index to be completed first.
    // None means the step is being executed or finished, and an empty vec means it has no requirements (root step)
    let mut requirements: Vec<Option<RequiredIndices>> = vec![Some(vec![]); unique_steps.len()];
    instructions.iter().for_each(|(req, follow_up)| {
        if let Some(required_steps) = requirements.get_mut(follow_up.to_index() as usize).unwrap() {
            required_steps.push(req.to_index());
        }
    });
    // println!("{} required steps: {:?}", requirements.len(), requirements);

    let mut free_worker_count = worker_count;
    let mut busy_workers: Vec<Worker> = vec![];

    let mut executed_steps: Vec<char> = Vec::new();
    let mut i = 0;
    while executed_steps.len() < unique_steps.len() {
        while free_worker_count > 0 {
            if let Some(idx) = get_next_step(&mut requirements, &mut executed_steps) {
                // Remove step from requirements
                requirements[idx as usize] = None;
                // assign all available steps to free workers
                free_worker_count -= 1;
                // println!(
                //     "{}: Assigned {} @ {} to a worker, {} free",
                //     i,
                //     idx.to_step(),
                //     idx + 1 + base_duration,
                //     free_worker_count
                // );
                busy_workers.push((idx.to_step(), idx + 1 + base_duration));
            } else {
                // println!("No work available");
                break;
            }
        }
        // Work all busy workers one unit
        busy_workers.iter_mut().for_each(|(_step, remaining)| {
            if *remaining > 0 {
                // println!(
                //     "{}: Worked {} from {} to {}",
                //     i,
                //     _step,
                //     remaining,
                //     *remaining - 1
                // );
                *remaining -= 1
            }
        });
        // If a step is finished, mark it as such
        while let Some(pos) = busy_workers
            .iter()
            .position(|(_, remaining)| *remaining == 0)
        {
            let (step, _) = busy_workers.remove(pos);
            executed_steps.push(step);
            free_worker_count += 1;
            // println!(
            //     "{}: Worker finished {}, {} free",
            //     i, step, free_worker_count
            // );
        }
        i += 1;
    }

    (executed_steps.iter().collect(), i)
}

fn get_next_step(
    requirements: &mut Vec<Option<RequiredIndices>>,
    executed: &mut Vec<Step>,
) -> Option<Index> {
    requirements
        .iter_mut()
        .enumerate()
        .filter(|(_, required_steps)| required_steps.is_some())
        .find(|(_, required_steps)| {
            // has no prerequisites or they were already executed
            required_steps.as_ref().unwrap().is_empty()
                || required_steps
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|req| executed.contains(&req.to_step()))
        })
        .map(|(idx, _)| idx as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn test_to_index() {
        assert_eq!(0, 'A'.to_index());
    }

    #[test]
    fn test_to_step() {
        assert_eq!('A', 0u8.to_step());
    }

    const EXAMPLE1: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn example_1() {
        assert_eq!("CABDFE", order_of_steps(&read_str_to_lines(EXAMPLE1)));
    }

    #[test]
    fn part_1() {
        assert_eq!(
            "JNOIKSYABEQRUVWXGTZFDMHLPC",
            order_of_steps(&read_file_to_lines("input/day07.txt"))
        );
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(15, count_seconds(&read_str_to_lines(EXAMPLE1), 2, 0));
    }

    #[test]
    fn part_2() {
        assert_eq!(
            1099,
            count_seconds(&read_file_to_lines("input/day07.txt"), 5, 60)
        );
    }
}
