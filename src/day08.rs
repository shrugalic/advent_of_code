use line_reader::read_file_to_lines;
use std::collections::HashMap;

pub(crate) fn day8_part1() -> isize {
    greatest_value_in_any_register_after_running(read_file_to_lines("input/day08.txt")).0
}

pub(crate) fn day8_part2() -> isize {
    greatest_value_in_any_register_after_running(read_file_to_lines("input/day08.txt")).1
}

fn greatest_value_in_any_register_after_running(instructions: Vec<String>) -> (isize, isize) {
    let mut registers: HashMap<String, isize> = HashMap::new();
    let mut max_reg_val = 0;
    for instruction in instructions {
        let (instr, cond) = instruction.split_once(" if ").unwrap();

        let cond: Vec<_> = cond.split_ascii_whitespace().collect();
        let (reg_val, cmp_op, cmp_val) = parse_condition(cond, &registers);
        if is_condition_true(reg_val, cmp_op, cmp_val) {
            let (reg_idx, op, val) = parse_instruction(instr);
            let reg_val = registers.entry(reg_idx.to_string()).or_insert(0);
            apply_operation(op, val, reg_val);
            max_reg_val = max_reg_val.max(*reg_val);
        }
    }
    (*registers.values().max().unwrap(), max_reg_val)
}

fn parse_condition<'a>(
    cond: Vec<&'a str>,
    registers: &HashMap<String, isize>,
) -> (isize, &'a str, isize) {
    let reg_val = *registers.get(cond[0]).unwrap_or(&0);
    let cmp_op = cond[1];
    let cmp_val: isize = cond[2].parse().unwrap();
    (reg_val, cmp_op, cmp_val)
}

fn is_condition_true(reg_val: isize, cmp_op: &str, cmp_val: isize) -> bool {
    match cmp_op {
        ">" => reg_val > cmp_val,
        "<" => reg_val < cmp_val,
        ">=" => reg_val >= cmp_val,
        "<=" => reg_val <= cmp_val,
        "==" => reg_val == cmp_val,
        "!=" => reg_val != cmp_val,
        _ => panic!("Unsupported comparison operation {}", cmp_op),
    }
}

fn parse_instruction(instr: &str) -> (&str, &str, isize) {
    let instr: Vec<_> = instr.split_ascii_whitespace().collect();
    let reg = instr[0];
    let op = instr[1];
    let val: isize = instr[2].parse().unwrap();
    (reg, op, val)
}

fn apply_operation(op: &str, val: isize, reg: &mut isize) {
    match op {
        "inc" => *reg += val,
        "dec" => *reg -= val,
        _ => panic!("Unsupported op {}", op),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn example_part1() {
        assert_eq!(
            1,
            greatest_value_in_any_register_after_running(read_str_to_lines(EXAMPLE)).0
        );
    }

    #[test]
    fn part1() {
        assert_eq!(4902, day8_part1());
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            10,
            greatest_value_in_any_register_after_running(read_str_to_lines(EXAMPLE)).1
        );
    }

    #[test]
    fn part2() {
        assert_eq!(7037, day8_part2());
    }
}
