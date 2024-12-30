use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

const INPUT: &str = include_str!("../../2024/input/day24.txt");
const EXPECTATION_FUNC: BinaryPredicate = |x, y| x + y;
type BinaryPredicate = fn(usize, usize) -> usize;

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> String {
    solve_part2(INPUT, EXPECTATION_FUNC)
}

fn solve_part1(input: &str) -> usize {
    let (gate_by_name, inputs_by_name) = parse(input);
    gate_by_name
        .calculate_outputs(&inputs_by_name)
        .sum_up_z_bits()
}

fn solve_part2(input: &str, expectation_func: BinaryPredicate) -> String {
    let (gate_by_name, inputs_by_name) = parse(input);
    gate_by_name.list_outputs_to_swap(inputs_by_name, expectation_func)
}

type WireName<'n> = &'n str;
struct GateByName<'n>(HashMap<WireName<'n>, Gate<'n>>);
type ValueByName<'n> = HashMap<WireName<'n>, bool>;
struct OutputByName<'n>(ValueByName<'n>);

#[derive(Debug)]
struct Gate<'n> {
    input1: WireName<'n>,
    op: Operation,
    input2: WireName<'n>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

fn parse(input: &str) -> (GateByName, ValueByName) {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let inputs_by_name: HashMap<WireName, bool> = wires
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(name, value)| (name, value == "1"))
        .collect();

    let gate_by_name = gates
        .lines()
        .map(|line| {
            // Example: x00 AND y00 -> z00
            let parts: Vec<_> = line.split_whitespace().collect();
            let input1 = parts[0];
            let op = Operation::from(parts[1]);
            let input2 = parts[2];
            let output = parts[4];
            (output, Gate { input1, op, input2 })
        })
        .collect();
    (GateByName(gate_by_name), inputs_by_name)
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        match op {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!("Illegal op {op}"),
        }
    }
}
impl Operation {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Operation::And => lhs & rhs,
            Operation::Or => lhs | rhs,
            Operation::Xor => lhs ^ rhs,
        }
    }
}
impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::And => "AND",
                Operation::Or => "OR ",
                Operation::Xor => "XOR",
            }
        )
    }
}

impl<'n> GateByName<'n> {
    fn calculate_outputs(&'n self, inputs: &'n ValueByName<'n>) -> OutputByName<'n> {
        let mut outputs = ValueByName::new();
        for &name in self.0.keys().filter(|name| name.starts_with('z')) {
            self.value_of_wire(name, inputs, &mut outputs);
        }
        OutputByName(outputs)
    }

    fn value_of_wire(
        &'n self,
        name: WireName<'n>,
        inputs: &ValueByName<'n>,
        outputs: &mut ValueByName<'n>,
    ) -> bool {
        if name.starts_with(['x', 'y']) {
            inputs[&name]
        } else if let Some(value) = outputs.get(name) {
            *value
        } else {
            let Gate { input1, op, input2 } = &self.0[name];
            let value1 = self.value_of_wire(input1, inputs, outputs);
            let value2 = self.value_of_wire(input2, inputs, outputs);
            let value = op.apply(value1, value2);
            outputs.insert(name, value);
            value
        }
    }

    // Includes all ancestors of the given gate (not including the gate itself)
    fn ancestors_of(&self, name: WireName<'n>) -> HashSet<WireName<'n>> {
        let mut ancestors = HashSet::new();
        if let Some(Gate { input1, input2, .. }) = self.0.get(name) {
            if !input1.starts_with(['x', 'y']) && !ancestors.contains(input1) {
                ancestors.insert(*input1);
                ancestors.extend(self.ancestors_of(input1));
            }
            if !input2.starts_with(['x', 'y']) && !ancestors.contains(input2) {
                ancestors.insert(*input2);
                ancestors.extend(self.ancestors_of(input2));
            }
        }
        ancestors
    }

    fn list_outputs_to_swap(
        mut self,
        mut inputs: ValueByName<'n>,
        expectation_func: BinaryPredicate,
    ) -> String {
        let mut swaps: Vec<String> = vec![];

        while let Some(idx) = self.index_of_first_error(&mut inputs, expectation_func) {
            // There's an error at index `idx`, there must be some misconfigured gates nearby

            let z_at = |idx| {
                self.0
                    .keys()
                    .find(|&name| *name == format!("z{idx:02}").as_str())
                    .unwrap()
            };

            // The misconfiguration must be after the previous and before the next index
            let curr_z = z_at(idx);
            let next_z = z_at(idx + 1);
            let prev_z = z_at(idx - 1);
            let mut next_ancestors = self.ancestors_of(next_z);
            // Add the current output gate, because it could be missing
            // from the next output gate's ancestors due to a misconfiguration
            next_ancestors.insert(curr_z);
            let prev_ancestors = self.ancestors_of(prev_z);
            let candidates = next_ancestors
                .difference(&prev_ancestors)
                .cloned()
                .collect();

            // Find the correct pair to swap by trial and error
            swaps.extend(self.try_swapping_to_find_correct_pair(
                &mut inputs,
                candidates,
                expectation_func,
                idx,
            ));
        }
        swaps.sort_unstable();
        swaps.join(",")
    }

    fn index_of_first_error(
        &self,
        inputs: &mut ValueByName,
        expectation_func: BinaryPredicate,
    ) -> Option<usize> {
        let input_len = inputs.len() / 2; // inputs contains both x and y
        let set_input_bit = |prefix: char, val: usize, inputs: &mut ValueByName| {
            for (name, bit) in inputs
                .iter_mut()
                .filter(|(name, _)| name.starts_with(prefix))
            {
                let idx: usize = name.strip_prefix(['x', 'y']).unwrap().parse().unwrap();
                *bit = (val & (1 << idx)) != 0;
            }
        };

        for idx in 0..input_len - 1 {
            for (x, y) in [(0, 1), (1, 0), (1, 1)] {
                let x = x << idx;
                let y = y << idx;
                set_input_bit('x', x, inputs);
                set_input_bit('y', y, inputs);
                let actual = self.calculate_outputs(inputs).sum_up_z_bits();
                let expected = expectation_func(x, y);
                let diff = actual ^ expected;
                if actual != expected {
                    return (0usize..).find(|idx| diff & (1 << *idx) != 0);
                }
            }
        }
        None
    }

    fn try_swapping_to_find_correct_pair<'a>(
        &mut self,
        inputs_by_name: &'a mut ValueByName<'n>,
        candidates: Vec<WireName<'n>>,
        expectation_func: BinaryPredicate,
        prev_idx: usize,
    ) -> Vec<String> {
        let mut swaps: Vec<_> = vec![];
        for (i, gate1) in candidates.iter().enumerate().take(candidates.len() - 1) {
            for gate2 in candidates.iter().skip(i + 1) {
                self.swap(gate1, gate2);

                if !self.contains_cycle(gate1)
                    && !self.contains_cycle(gate2)
                    && self
                        .index_of_first_error(inputs_by_name, expectation_func)
                        .is_none_or(|curr_idx| curr_idx > prev_idx)
                {
                    // Fails later or not at all
                    swaps.push(gate1.to_string());
                    swaps.push(gate2.to_string());
                    return swaps;
                }
                // Revert
                self.swap(gate1, gate2);
            }
        }
        unreachable!();
    }

    fn swap(&mut self, name1: WireName<'n>, name2: WireName<'n>) {
        let gate1 = self.0.remove(name1).unwrap();
        let gate2 = self.0.remove(name2).unwrap();
        self.0.insert(name2, gate1);
        self.0.insert(name1, gate2);
    }

    fn contains_cycle(&self, name: WireName) -> bool {
        self.is_node_its_own_child(name, &mut HashSet::new())
    }

    fn is_node_its_own_child(
        &self,
        name: WireName<'n>,
        children: &mut HashSet<WireName<'n>>,
    ) -> bool {
        if name.starts_with(['x', 'y']) {
            return false;
        }
        if !children.insert(name) {
            return true;
        }
        if let Some(Gate { input1, input2, .. }) = &self.0.get(name) {
            self.is_node_its_own_child(input1, children)
                || self.is_node_its_own_child(input2, children)
        } else {
            false
        }
    }
}

impl OutputByName<'_> {
    fn sum_up_z_bits(self) -> usize {
        self.0
            .into_iter()
            .filter(|(name, bit)| name.starts_with('z') && *bit)
            .map(|(name, _)| 2usize.pow(name.strip_prefix("z").unwrap().parse().unwrap()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE_1: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const EXAMPLE_1: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn test_part1_small_example() {
        assert_eq!(4, solve_part1(SMALL_EXAMPLE_1));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(2024, solve_part1(EXAMPLE_1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(58367545758258, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("bpf,fdw,hcc,hqc,qcw,z05,z11,z35", part2());
    }

    #[test]
    fn test_part1_example_gate_values() {
        let expected_gate_values = "\
bfw: 1
bqk: 1
djm: 1
ffh: 0
fgs: 1
frj: 1
fst: 1
gnj: 1
hwm: 1
kjc: 0
kpj: 1
kwq: 0
mjb: 1
nrd: 1
ntg: 0
pbm: 1
psh: 1
qhw: 1
rvg: 0
tgd: 0
tnw: 1
vdt: 1
wpb: 0
z00: 0
z01: 0
z02: 0
z03: 1
z04: 0
z05: 1
z06: 1
z07: 1
z08: 1
z09: 1
z10: 1
z11: 0
z12: 0";

        let (gate_by_name, inputs_by_name) = parse(EXAMPLE_1);

        let expected_values: HashMap<WireName, bool> = expected_gate_values
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, value)| (name, value == "1"))
            .collect();

        let actual_values = gate_by_name.calculate_outputs(&inputs_by_name);
        for name in gate_by_name.0.keys() {
            let expected = &expected_values[name];
            let actual = actual_values.0[name];
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_ancestors_of() {
        let (gate_by_name, _) = parse(INPUT);
        let z01_deps = gate_by_name.ancestors_of("z01");
        assert_eq!(z01_deps, HashSet::from(["mkf", "msh"]));
    }

    #[test]
    fn test_cycle_detection() {
        let (mut gate_by_name, _) = parse(INPUT);
        gate_by_name.swap("z11", "cgn"); // this produces a cycle
        assert!(!gate_by_name.contains_cycle("z11"));
        assert!(gate_by_name.contains_cycle("z12"));
    }
}
