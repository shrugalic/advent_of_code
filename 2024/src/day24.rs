use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

const INPUT: &str = include_str!("../../2024/input/day24.txt");
const EXPECTATION_FUNC: BinaryFunction = |x, y| x + y;
type BinaryFunction = fn(usize, usize) -> usize;

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> String {
    solve_part2(INPUT, EXPECTATION_FUNC)
}

fn solve_part1(input: &str) -> usize {
    Machine::from(input).calculate_output()
}

fn solve_part2(input: &str, expectation_func: BinaryFunction) -> String {
    let mut machine = Machine::from(input);
    machine.list_outputs_to_swap(expectation_func)
}

#[derive(Debug)]
struct Machine<'a> {
    x_input: usize,
    y_input: usize,
    input_len: usize,               // Number of bits in x and y
    z_output_gate_ids: Vec<GateId>, // Its index is the output number, its value an index into gates
    gates: Vec<Gate>,               // all gates, where their index is their id
    gate_names: Vec<WireName<'a>>,  // Mapping from gate id to output wire name
    gate_values: GateValueCache,
}

type GateId = usize;
type WireName<'a> = &'a str;
type WireId = usize;
type GateValueCache = Vec<Option<bool>>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Gate {
    input1: SourceId,
    op: Operation,
    input2: SourceId,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum SourceId {
    XWire(WireId),
    YWire(WireId),
    Gate(GateId),
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl SourceId {
    fn calculate_output(&self, machine: &Machine, cache: &mut GateValueCache) -> bool {
        match self {
            SourceId::XWire(id) => machine.x(id),
            SourceId::YWire(id) => machine.y(id),
            SourceId::Gate(id) => machine.gate_value(id, cache),
        }
    }
}

impl Debug for SourceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SourceId::XWire(x) => format!("x{x:02}"),
                SourceId::YWire(y) => format!("y{y:02}"),
                SourceId::Gate(gate_id) => format!("gate {gate_id}"),
            }
        )
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

impl<'a> From<&'a str> for Machine<'a> {
    fn from(input: &'a str) -> Self {
        let (wires, gates) = input.split_once("\n\n").unwrap();
        let input_bits = wires.lines().count() / 2;

        let mut gate_tuples = Vec::new();
        let mut gate_names = Vec::new();
        let mut z_gate_ids = Vec::new();
        let mut gate_id_by_output_name: HashMap<WireName, GateId> = HashMap::new();
        for (gate_id, line) in gates.lines().enumerate() {
            // Example: x00 AND y00 -> z00
            let parts: Vec<_> = line.split_whitespace().collect();
            let mut input1 = parts[0];
            let op = Operation::from(parts[1]);
            let mut input2 = parts[2];
            let output = parts[4];
            if input1 > input2 {
                (input1, input2) = (input2, input1);
            }
            gate_names.push(output);
            if output.starts_with("z") {
                z_gate_ids.push(gate_id);
            }
            gate_id_by_output_name.insert(output, gate_id);
            gate_tuples.push((input1, op, input2));
        }

        // Sort z-gates so their name corresponds with their index
        z_gate_ids.sort_unstable_by_key(|z| gate_names[*z]);

        let source_id_from = |name: &str| {
            if name.starts_with("x") {
                SourceId::XWire(name.strip_prefix("x").unwrap().parse::<WireId>().unwrap())
            } else if name.starts_with("y") {
                SourceId::YWire(name.strip_prefix("y").unwrap().parse::<WireId>().unwrap())
            } else {
                SourceId::Gate(gate_id_by_output_name[name])
            }
        };

        let gates: Vec<Gate> = gate_tuples
            .into_iter()
            .map(|(input1, op, input2)| Gate {
                input1: source_id_from(input1),
                op,
                input2: source_id_from(input2),
            })
            .collect();

        // stores the x and y bits as a single usize
        let accumulate = |prefix| {
            wires
                .lines()
                .rev()
                .filter(|line| line.starts_with(prefix))
                .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap())
                .fold(0usize, |acc, v| (acc << 1) + v)
        };

        let gate_values = vec![None; gates.len()];
        Machine {
            x_input: accumulate("x"),
            y_input: accumulate("y"),
            input_len: input_bits,
            z_output_gate_ids: z_gate_ids,
            gates,
            gate_names,
            gate_values,
        }
    }
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

impl Machine<'_> {
    fn x(&self, id: &WireId) -> bool {
        (self.x_input & (1 << *id)) != 0
    }
    fn y(&self, id: &WireId) -> bool {
        (self.y_input & (1 << *id)) != 0
    }
    fn gate(&self, id: &GateId) -> &Gate {
        &self.gates[*id]
    }
    fn reset(&mut self) {
        self.x_input = 0;
        self.y_input = 0;
        self.gate_values = vec![None; self.gates.len()];
    }
    fn calculate_output(&mut self) -> usize {
        let mut cache: GateValueCache = vec![None; self.gates.len()];
        let sum = self
            .z_output_gate_ids
            .iter()
            .enumerate()
            .map(|(exponent, gate_id)| {
                if self.gate_value(gate_id, &mut cache) {
                    2usize.pow(exponent as u32)
                } else {
                    0
                }
            })
            .sum();

        self.gate_values = cache;
        sum
    }
    fn gate_value(&self, gate_id: &GateId, cache: &mut GateValueCache) -> bool {
        if let Some(value) = cache[*gate_id] {
            value
        } else {
            let Gate {
                input1, op, input2, ..
            } = &self.gate(gate_id);

            let input1 = input1.calculate_output(self, cache);
            let input2 = input2.calculate_output(self, cache);
            let value = match op {
                Operation::And => input1 & input2,
                Operation::Or => input1 | input2,
                Operation::Xor => input1 ^ input2,
            };

            cache[*gate_id] = Some(value);
            value
        }
    }

    pub(crate) fn list_outputs_to_swap(
        &mut self,
        expectation_func: fn(usize, usize) -> usize,
    ) -> String {
        let mut swaps: Vec<(&str, &str)> = vec![];
        'test: loop {
            // increase inputs until output is incorrect
            for bit_idx in 0..self.input_len {
                self.reset();
                let mut expected_bits: Vec<bool> = Vec::new();
                let mut actual_bits: Vec<bool> = Vec::new();
                let mut actual_gate_bits: Vec<Vec<bool>> = vec![vec![]; self.gates.len()];

                for (x_bit, y_bit) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                    self.x_input = x_bit << bit_idx;
                    self.y_input = y_bit << bit_idx;

                    let expected_val = expectation_func(self.x_input, self.y_input);
                    let expected_bit = (expected_val & (1 << bit_idx)) != 0;
                    expected_bits.push(expected_bit);

                    let actual_val = self.calculate_output();
                    let actual_bit = (actual_val & (1 << bit_idx)) != 0;
                    actual_bits.push(actual_bit);

                    actual_gate_bits
                        .iter_mut()
                        .zip(&self.gate_values)
                        .for_each(|(bits, value)| bits.push(value.unwrap()));
                }

                if expected_bits != actual_bits {
                    println!("Unexpected outputs for input permutations at index {bit_idx}");
                    let matching_gate_idxs: Vec<_> = actual_gate_bits
                        .iter()
                        .enumerate()
                        .filter(|(_, actual)| actual == &&expected_bits)
                        .map(|(i, _)| i)
                        .collect();
                    if matching_gate_idxs.len() == 1 {
                        // Another gate's output exactly matches the expectation, swap the two.
                        // This works for the part 2 example, but not the real input.
                        let matching_gate_idx = *matching_gate_idxs.first().unwrap();
                        let z_gate_idx = self.z_output_gate_ids[bit_idx];
                        self.gates.swap(matching_gate_idx, z_gate_idx);
                        swaps.push((
                            self.gate_names[matching_gate_idx],
                            self.gate_names[z_gate_idx],
                        ));

                        continue 'test;
                    } else if matching_gate_idxs.is_empty() {
                        println!("- no match");
                    } else {
                        // The part 2 input lands here, at 4 bits, with 2, 3 or 4 matches each.
                        // These were inspected manually
                        println!("- {} matches", matching_gate_idxs.len());
                    }
                }
            }
            let mut swaps: Vec<_> = swaps.into_iter().flat_map(|(a, b)| [a, b]).collect();
            swaps.sort_unstable();
            return swaps.join(",");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTATION_FUNC: BinaryFunction = |x, y| x & y;
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

    const EXAMPLE_2: &str = "\
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
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
    fn test_part1_example_gate_values() {
        let mut machine = Machine::from(EXAMPLE_1);
        machine.calculate_output();
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
        let expected_value_by_name: HashMap<WireName, bool> = expected_gate_values
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, value)| (name, value == "1"))
            .collect();
        for (id, output_name) in machine.gate_names.iter().enumerate() {
            let expected = &expected_value_by_name[output_name];
            let actual = machine.gate_values[id].unwrap();
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(58367545758258, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!("z00,z01,z02,z05", solve_part2(EXAMPLE_2, EXPECTATION_FUNC));
    }

    #[test]
    fn test_part2() {
        assert_eq!("bpf,fdw,hcc,hqc,qcw,z05,z11,z35", part2());
    }
}
