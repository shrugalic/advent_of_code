use Mode::*;
use Op::*;

const TEST_MODE_INPUT: i64 = 1;
#[derive(Debug)]
struct IntCodeComputer {
    instr: Vec<i64>, // program
    ptr: usize,      // instruction pointer
    base: i64,       // relative base
}
impl IntCodeComputer {
    fn new(instr: &mut Vec<i64>) -> Self {
        IntCodeComputer {
            instr: instr.to_vec(),
            ptr: 0,
            base: 0,
        }
    }
    fn next_op_as_5_digit_string_padded_with_leading_zeroes(&self) -> String {
        let s = self.instr[self.ptr].to_string();
        "0".repeat(5 - s.len()) + s.as_ref()
    }
    fn get(&mut self, idx: usize) -> i64 {
        self.grow_if_needed(idx);
        self.instr[idx]
    }
    fn set(&mut self, idx: usize, val: i64) {
        self.grow_if_needed(idx);
        self.instr[idx] = val;
    }
    fn grow_if_needed(&mut self, idx: usize) {
        if idx >= self.instr.len() {
            let diff = 1 + idx - self.instr.len();
            if diff > 10000 {
                println!(
                    "old size = {}, idx = {}, huge diff = {}",
                    self.instr.len(),
                    idx,
                    diff
                );
            }
            self.instr.extend(vec![0; diff]);
        }
    }
    fn get_value(&mut self, offset: usize, mode: &Mode) -> i64 {
        let val = self.get(self.ptr + offset);
        match mode {
            Immediate => val,
            Position => self.get(val as usize),
            Relative => self.get((self.base + val) as usize),
        }
    }
    fn set_result(&mut self, offset: usize, mode: &Mode, res: i64) {
        let val = self.get(self.ptr + offset);
        println!(" [{}] = {}", val, res);
        match mode {
            Immediate => panic!("Output parameter in immediate mode!"),
            Position => self.set(val as usize, res),
            Relative => self.set((self.base + val) as usize, res),
        }
    }

    fn process_int_code_with_default_input(&mut self) -> Option<i64> {
        self.process_int_code_with_input(TEST_MODE_INPUT)
    }

    fn process_int_code_with_input(&mut self, input: i64) -> Option<i64> {
        let mut output = None;
        while self.ptr < self.instr.len() {
            let s = self.next_op_as_5_digit_string_padded_with_leading_zeroes();
            let code = to_num(&s[(s.len() - 2)..s.len()]);
            let op = Op::from_code(code);
            let modes = Mode::extract_modes(&s);
            let pre = format!("{:?}: {:?}", s, op);
            match op {
                Add | Multiply | LessThan | Equals => {
                    let p1 = self.get_value(1, &modes[0]);
                    let p2 = self.get_value(2, &modes[1]);
                    let res = match op {
                        Add => p1 + p2,
                        Multiply => p1 * p2,
                        LessThan => eval(p1 < p2),
                        Equals => eval(p1 == p2),
                        _ => unreachable!(),
                    };
                    print!("{}({}, {})", pre, p1, p2);
                    self.set_result(3, &modes[2], res);
                }
                Input => {
                    print!("{}", pre);
                    self.set_result(1, &modes[0], input);
                }
                Output => {
                    let value = self.get_value(1, &modes[0]);
                    println!("{} = {}", pre, value);
                    output = Some(value);
                }
                ShiftRelativeBase => {
                    let shift = self.get_value(1, &modes[0]);
                    let old_base = self.base;
                    self.base = self.base + shift;
                    println!("{} by {} from {} to {}", pre, shift, old_base, self.base);
                }
                JumpIfTrue | JumpIfFalse => {
                    let p1 = self.get_value(1, &modes[0]);
                    let p2 = self.get_value(2, &modes[1]);
                    if op == JumpIfTrue && p1 != 0 || op == JumpIfFalse && p1 == 0 {
                        self.ptr = p2 as usize;
                        println!("{} ({}) == true -> jump to {}", pre, p1, p2);
                        continue; // jump, rather than increasing idx below
                    }
                    println!("{} ({}) == false -> NO jump (to {})", pre, p1, p2);
                }
                Stop => {
                    println!("{}", pre);
                    break;
                }
            }
            self.ptr += op.value_count();
        }
        output
    }
}
fn to_num(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}
fn eval(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    ShiftRelativeBase,
    Stop,
}

impl Op {
    fn from_code(code: i64) -> Op {
        match code {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => ShiftRelativeBase,
            99 => Stop,
            _ => panic!("Unknown Op code {:?}", code),
        }
    }
    fn value_count(&self) -> usize {
        match self {
            Add | Multiply | LessThan | Equals => 4,
            JumpIfTrue | JumpIfFalse => 3,
            Input | Output | ShiftRelativeBase => 2,
            Stop => 1,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}
impl Mode {
    fn extract_modes(s: &str) -> Vec<Mode> {
        vec![
            Mode::from_code(to_num(&s[2..=2])),
            Mode::from_code(to_num(&s[1..=1])),
            Mode::from_code(to_num(&s[0..=0])),
        ]
    }
    fn from_code(code: i64) -> Mode {
        match code {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Unknown Mode code {:?}", code),
        }
    }
    fn to_code(&self) -> usize {
        match self {
            Position => 0,
            Immediate => 1,
            Relative => 2,
        }
    }
}

mod tests {
    use crate::{IntCodeComputer, Op, Op::*};

    // day 9 part 1

    #[test]
    fn test_autoextend_on_get() {
        let mut icc = IntCodeComputer::new(&mut vec![]);
        assert_eq!(icc.get(0), 0);
        assert_eq!(icc.instr.len(), 1);
    }

    #[test]
    fn test_autoextend_on_set() {
        let mut icc = IntCodeComputer::new(&mut vec![]);
        icc.set(0, 123);
        assert_eq!(icc.instr.len(), 1);
        assert_eq!(icc.get(0), 123);
    }

    #[test]
    fn day9_part1_example1() {
        let mut icc = IntCodeComputer::new(&mut vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(99));
    }

    #[test]
    fn day9_part1_example2() {
        let mut icc = IntCodeComputer::new(&mut vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!(
            icc.process_int_code_with_default_input(),
            Some(1219070632396864)
        );
    }

    #[test]
    fn day9_part1_example3() {
        let mut icc = IntCodeComputer::new(&mut vec![104, 1125899906842624, 99]);
        assert_eq!(
            icc.process_int_code_with_default_input(),
            Some(1125899906842624)
        );
    }
    #[test]
    fn day9_add_with_relative_input() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 2201, 1, 2, 9, 99, 11, 22, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![109, 6, 2201, 1, 2, 9, 99, 11, 22, 33]);
    }
    #[test]
    fn day9_add_with_relative_output() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 21101, 11, 22, 1, 99, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![109, 6, 21101, 11, 22, 1, 99, 33]);
    }
    #[test]
    fn day9_part1_output_with_relative_base_above_0() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 10, 204, -5, 99, 123]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(123));
    }
    #[test]
    fn day9_part1_output_with_relative_base_below_0() {
        let mut icc = IntCodeComputer::new(&mut vec![109, -5, 204, 10, 99, 123]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(123));
    }
    #[test]
    fn day9_part1_mirror_relative_input_to_output() {
        let mut icc = IntCodeComputer::new(&mut vec![203, 3, 104, 0, 99]);
        assert_eq!(icc.process_int_code_with_input(22), Some(22));
    }
    #[test]
    fn day9_part1_mirror_shifted_relative_input_to_output() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 203, -1, 104, 0, 99]);
        println!("{:?}", icc.instr);
        assert_eq!(icc.process_int_code_with_input(33), Some(33));
    }

    #[test]
    fn day9_part1() {
        let mut icc = IntCodeComputer::new(&mut day9_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(1), Some(3518157894));
    }
    #[test]
    fn day9_part2() {
        let mut icc = IntCodeComputer::new(&mut day9_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(2), Some(80379));
    }

    fn day9_puzzle_input() -> Vec<i64> {
        vec![
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 3, 0, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 25, 0, 1016, 1102, 760, 1, 1023, 1102,
            1, 20, 1003, 1102, 1, 22, 1015, 1102, 1, 34, 1000, 1101, 0, 32, 1006, 1101, 21, 0,
            1017, 1102, 39, 1, 1010, 1101, 30, 0, 1005, 1101, 0, 1, 1021, 1101, 0, 0, 1020, 1102,
            1, 35, 1007, 1102, 1, 23, 1014, 1102, 1, 29, 1019, 1101, 767, 0, 1022, 1102, 216, 1,
            1025, 1102, 38, 1, 1011, 1101, 778, 0, 1029, 1102, 1, 31, 1009, 1101, 0, 28, 1004,
            1101, 33, 0, 1008, 1102, 1, 444, 1027, 1102, 221, 1, 1024, 1102, 1, 451, 1026, 1101,
            787, 0, 1028, 1101, 27, 0, 1018, 1101, 0, 24, 1013, 1102, 26, 1, 1012, 1101, 0, 36,
            1002, 1102, 37, 1, 1001, 109, 28, 21101, 40, 0, -9, 1008, 1019, 41, 63, 1005, 63, 205,
            1001, 64, 1, 64, 1105, 1, 207, 4, 187, 1002, 64, 2, 64, 109, -9, 2105, 1, 5, 4, 213,
            1106, 0, 225, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -9, 1206, 10, 243, 4, 231, 1001,
            64, 1, 64, 1105, 1, 243, 1002, 64, 2, 64, 109, -3, 1208, 2, 31, 63, 1005, 63, 261, 4,
            249, 1106, 0, 265, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5, 21108, 41, 41, 0, 1005,
            1012, 287, 4, 271, 1001, 64, 1, 64, 1105, 1, 287, 1002, 64, 2, 64, 109, 6, 21102, 42,
            1, -5, 1008, 1013, 45, 63, 1005, 63, 307, 1105, 1, 313, 4, 293, 1001, 64, 1, 64, 1002,
            64, 2, 64, 109, -9, 1201, 0, 0, 63, 1008, 63, 29, 63, 1005, 63, 333, 1106, 0, 339, 4,
            319, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -13, 2102, 1, 4, 63, 1008, 63, 34, 63,
            1005, 63, 361, 4, 345, 1105, 1, 365, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5, 1201, 7,
            0, 63, 1008, 63, 33, 63, 1005, 63, 387, 4, 371, 1105, 1, 391, 1001, 64, 1, 64, 1002,
            64, 2, 64, 109, 7, 1202, 1, 1, 63, 1008, 63, 32, 63, 1005, 63, 411, 1105, 1, 417, 4,
            397, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 20, 1205, -7, 431, 4, 423, 1106, 0, 435,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, 2, 2106, 0, -3, 1001, 64, 1, 64, 1105, 1, 453,
            4, 441, 1002, 64, 2, 64, 109, -7, 21101, 43, 0, -9, 1008, 1014, 43, 63, 1005, 63, 479,
            4, 459, 1001, 64, 1, 64, 1105, 1, 479, 1002, 64, 2, 64, 109, -5, 21108, 44, 43, 0,
            1005, 1018, 495, 1105, 1, 501, 4, 485, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -7, 1205,
            9, 517, 1001, 64, 1, 64, 1105, 1, 519, 4, 507, 1002, 64, 2, 64, 109, 11, 1206, -1, 531,
            1106, 0, 537, 4, 525, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -15, 1208, 0, 36, 63,
            1005, 63, 557, 1001, 64, 1, 64, 1106, 0, 559, 4, 543, 1002, 64, 2, 64, 109, 7, 2101, 0,
            -7, 63, 1008, 63, 35, 63, 1005, 63, 581, 4, 565, 1106, 0, 585, 1001, 64, 1, 64, 1002,
            64, 2, 64, 109, -3, 21107, 45, 46, 4, 1005, 1015, 607, 4, 591, 1001, 64, 1, 64, 1105,
            1, 607, 1002, 64, 2, 64, 109, -16, 2102, 1, 10, 63, 1008, 63, 31, 63, 1005, 63, 631,
            1001, 64, 1, 64, 1106, 0, 633, 4, 613, 1002, 64, 2, 64, 109, 1, 2107, 33, 10, 63, 1005,
            63, 649, 1106, 0, 655, 4, 639, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 17, 2101, 0, -9,
            63, 1008, 63, 31, 63, 1005, 63, 679, 1001, 64, 1, 64, 1106, 0, 681, 4, 661, 1002, 64,
            2, 64, 109, -6, 2107, 34, 0, 63, 1005, 63, 703, 4, 687, 1001, 64, 1, 64, 1106, 0, 703,
            1002, 64, 2, 64, 109, 5, 1207, -5, 34, 63, 1005, 63, 719, 1105, 1, 725, 4, 709, 1001,
            64, 1, 64, 1002, 64, 2, 64, 109, -15, 1202, 6, 1, 63, 1008, 63, 20, 63, 1005, 63, 751,
            4, 731, 1001, 64, 1, 64, 1105, 1, 751, 1002, 64, 2, 64, 109, 21, 2105, 1, 5, 1001, 64,
            1, 64, 1106, 0, 769, 4, 757, 1002, 64, 2, 64, 109, 5, 2106, 0, 5, 4, 775, 1001, 64, 1,
            64, 1106, 0, 787, 1002, 64, 2, 64, 109, -27, 1207, 4, 35, 63, 1005, 63, 809, 4, 793,
            1001, 64, 1, 64, 1106, 0, 809, 1002, 64, 2, 64, 109, 13, 2108, 33, -1, 63, 1005, 63,
            831, 4, 815, 1001, 64, 1, 64, 1106, 0, 831, 1002, 64, 2, 64, 109, 4, 21107, 46, 45, 1,
            1005, 1014, 851, 1001, 64, 1, 64, 1105, 1, 853, 4, 837, 1002, 64, 2, 64, 109, 3, 21102,
            47, 1, -3, 1008, 1013, 47, 63, 1005, 63, 875, 4, 859, 1106, 0, 879, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, -9, 2108, 28, 2, 63, 1005, 63, 895, 1106, 0, 901, 4, 885, 1001,
            64, 1, 64, 4, 64, 99, 21101, 27, 0, 1, 21102, 1, 915, 0, 1106, 0, 922, 21201, 1, 59074,
            1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102, 942, 1,
            0, 1105, 1, 922, 21201, 1, 0, -1, 21201, -2, -3, 1, 21102, 1, 957, 0, 1105, 1, 922,
            22201, 1, -1, -2, 1106, 0, 968, 22102, 1, -2, -2, 109, -3, 2105, 1, 0,
        ]
    }

    // day 5 below

    #[test]
    fn op_from_int_code() {
        assert_eq!(Add, Op::from_code(1));
    }

    #[test]
    fn explanation_example() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(
            icc.instr,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn add_example_1() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 0, 0, 0, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn mult_example_1() {
        let mut icc = IntCodeComputer::new(&mut vec![2, 3, 0, 3, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn mult_example_2() {
        let mut icc = IntCodeComputer::new(&mut vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn add_example_2() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    // day 5 part 1

    #[test]
    fn multiply_example() {
        let mut icc = IntCodeComputer::new(&mut vec![1002, 4, 3, 4, 33]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![1002, 4, 3, 4, 99]);
    }

    fn day5_puzzle_input() -> Vec<i64> {
        vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1, 192, 154, 224, 101, -161, 224, 224,
            4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1001, 157, 48, 224, 1001,
            224, -61, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1102, 15,
            28, 225, 1002, 162, 75, 224, 1001, 224, -600, 224, 4, 224, 1002, 223, 8, 223, 1001,
            224, 1, 224, 1, 224, 223, 223, 102, 32, 57, 224, 1001, 224, -480, 224, 4, 224, 102, 8,
            223, 223, 101, 1, 224, 224, 1, 224, 223, 223, 1101, 6, 23, 225, 1102, 15, 70, 224,
            1001, 224, -1050, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223, 223,
            101, 53, 196, 224, 1001, 224, -63, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1,
            224, 223, 223, 1101, 64, 94, 225, 1102, 13, 23, 225, 1101, 41, 8, 225, 2, 105, 187,
            224, 1001, 224, -60, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 224, 223,
            223, 1101, 10, 23, 225, 1101, 16, 67, 225, 1101, 58, 10, 225, 1101, 25, 34, 224, 1001,
            224, -59, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999,
            1108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 107, 226, 226,
            224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 107, 677, 226, 224, 102, 2,
            223, 223, 1005, 224, 359, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1005,
            224, 374, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 389, 101,
            1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 101, 1, 223, 223,
            7, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 1107, 226, 677,
            224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2,
            223, 223, 1005, 224, 449, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005,
            224, 464, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 479, 1001,
            223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223,
            1008, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 1107, 677,
            226, 224, 1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 108, 677, 677, 224,
            1002, 223, 2, 223, 1005, 224, 539, 1001, 223, 1, 223, 1107, 226, 226, 224, 1002, 223,
            2, 223, 1006, 224, 554, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006,
            224, 569, 1001, 223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 584, 101,
            1, 223, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223,
            1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 614, 1001, 223, 1, 223, 8, 677, 677,
            224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2,
            223, 223, 1005, 224, 644, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223,
            1005, 224, 659, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224,
            674, 1001, 223, 1, 223, 4, 223, 99, 226,
        ]
    }

    #[test]
    fn part_1() {
        let mut icc = IntCodeComputer::new(&mut day5_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(1), Some(11049715));
    }

    // day 5 part 2

    #[test]
    fn input_equal_to_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(9), Some(0));
    }
    #[test]
    fn input_less_than_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(7), Some(1));
    }
    #[test]
    fn input_not_less_than_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(0));
    }
    #[test]
    fn input_equal_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(9), Some(0));
    }
    #[test]
    fn input_less_than_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(icc.process_int_code_with_input(7), Some(1));
    }
    #[test]
    fn input_not_less_than_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(icc.process_int_code_with_input(8), Some(0));
    }
    #[test]
    fn jump_test_position_mode_1() {
        let mut icc = IntCodeComputer::new(&mut vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        assert_eq!(icc.process_int_code_with_input(1), Some(1));
    }
    #[test]
    fn jump_test_position_mode_0() {
        let mut icc = IntCodeComputer::new(&mut vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        assert_eq!(icc.process_int_code_with_input(0), Some(0));
    }
    #[test]
    fn jump_test_immediate_mode_1() {
        let mut icc =
            IntCodeComputer::new(&mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(icc.process_int_code_with_input(1), Some(1));
    }
    #[test]
    fn jump_test_immediate_mode_0() {
        let mut icc =
            IntCodeComputer::new(&mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(icc.process_int_code_with_input(0), Some(0));
    }

    fn larger_example_input() -> Vec<i64> {
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
    }
    #[test]
    fn larger_example_less_than_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(7), Some(999));
    }
    #[test]
    fn larger_example_exactly_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(8), Some(1000));
    }
    #[test]
    fn larger_example_greater_than_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(9), Some(1001));
    }

    #[test]
    fn part_2() {
        let mut icc = IntCodeComputer::new(&mut day5_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(5), Some(2140710));
    }
}
