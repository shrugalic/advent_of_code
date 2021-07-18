use std::collections::HashSet;

#[cfg(test)]
mod tests;

type Index = u8;
type Step = char;
type Requirement = Step;
type FollowUp = Step;
type Instruction = (Requirement, FollowUp);
type RequiredIndices = Vec<Index>;

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
    let instructions: Vec<Instruction> = input.iter().map(String::to_instruction).collect();
    println!("{} instructions", instructions.len());
    let unique_steps: HashSet<Step> = instructions
        .iter()
        .flat_map(|(req, follow_up)| vec![*req, *follow_up])
        .collect();
    println!("{} unique steps", unique_steps.len());

    // Requirements for a given step, where the index of the vec
    // corresponds to the step that requires the follow-ups listed at this index
    let mut requirements: Vec<Option<RequiredIndices>> = vec![Some(vec![]); unique_steps.len()];
    instructions.iter().for_each(|(req, follow_up)| {
        if let Some(required_steps) = requirements.get_mut(follow_up.to_index() as usize).unwrap() {
            required_steps.push(req.to_index());
        }
    });
    // println!("{} required steps: {:?}", requirements.len(), requirements);

    let mut executed_steps: Vec<char> = Vec::new();
    while executed_steps.len() < unique_steps.len() {
        'inner: for (idx, required_steps) in requirements.iter_mut().enumerate() {
            if let Some(requireds) = required_steps {
                if requireds.is_empty()
                    || requireds
                        .iter()
                        .all(|req| executed_steps.contains(&req.to_step()))
                {
                    executed_steps.push((idx as Index).to_step());
                    *required_steps = None;
                    break 'inner;
                }
            }
        }
    }

    executed_steps.iter().collect()
}
