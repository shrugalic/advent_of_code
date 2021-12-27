use std::ops::RangeInclusive;

type Value = isize;

pub(crate) fn day24_part1() -> usize {
    find_max_model_number()
}

pub(crate) fn day24_part2() -> usize {
    find_min_model_number()
}

fn find_max_model_number() -> usize {
    let initial = |_min, max| max;
    let new_min = |mut min, diff, _len| -> usize {
        min /= 10_usize.pow(diff as u32);
        for _ in 0..diff {
            min = min * 10 + 1;
        }
        min
    };
    let new_max = |max, _diff, len| -> usize { dec(max, len) };
    find_model_number(initial, new_min, new_max, dec)
}

fn find_min_model_number() -> usize {
    let initial = |min, _max| min;
    let new_min = |min, _diff, len| -> usize { inc(min, len) };
    let new_max = |max, diff, _len| -> usize {
        let multiplier = 10_usize.pow(diff as u32);
        max / multiplier * multiplier + multiplier - 1
    };

    find_model_number(initial, new_min, new_max, inc)
}

fn find_model_number(
    initial: fn(usize, usize) -> usize,
    new_min: fn(usize, usize, usize) -> usize,
    new_max: fn(usize, usize, usize) -> usize,
    step_func: fn(usize, usize) -> usize,
) -> usize {
    // Meaningful total digit counts are 5, 7, 10, 11, 12, 13 and 14. We store only the difference:
    // 2 digits more than 5 digits are a total of 7 digits, 3 more than 7 are 10,
    // and the rest are 1 more digit than their respective previous count.
    const DIGIT_COUNT_DIFFS: [usize; 7] = [5, 2, 3, 1, 1, 1, 1];
    // For the base amount of 5 digits, these are the minimum and maximum numbers
    let mut min = 11111;
    let mut max = 99999;

    // This index controls how many extra (and total) digits we are testing
    let mut index = 0;

    let total_digit_count = |idx| DIGIT_COUNT_DIFFS.into_iter().take(idx + 1).sum::<usize>();

    // println!(
    //     "{}: {:2} total digits, min {:<14} max {:<14} initial",
    //     index,
    //     total_digit_count(index),
    //     min,
    //     max
    // );
    loop {
        let len: usize = total_digit_count(index);
        if let Some(best) = test_digits(len, initial(min, max), min..=max, step_func) {
            // If we found a result, we might be done
            if index == DIGIT_COUNT_DIFFS.len() - 1 {
                return best;
            }
            // Otherwise add more digits
            index += 1;

            let multiplier = 10_usize.pow(DIGIT_COUNT_DIFFS[index] as u32);
            max = best * multiplier + multiplier - 1;

            min = best;
            for _ in 0..DIGIT_COUNT_DIFFS[index] {
                min = min * 10 + 1;
            }

            // println!(
            //     "{}: {:2} total digits, min {:<14} max {:<14} increase number of digits",
            //     index,
            //     total_digit_count(index),
            //     min,
            //     max
            // );
        } else {
            // Else we need to backtrack: go back to fewer digits, and try again

            // Previous max and min
            let prev = max / 10_usize.pow(DIGIT_COUNT_DIFFS[index] as u32);
            if index > 0 {
                index -= 1;
            }

            min = new_min(prev, DIGIT_COUNT_DIFFS[index], total_digit_count(index));
            max = new_max(prev, DIGIT_COUNT_DIFFS[index], total_digit_count(index));

            // println!(
            //     "{}: {:2} total digits, min {:<14} max {:<14} decrease number of digits",
            //     index,
            //     total_digit_count(index),
            //     min,
            //     max
            // );
        }
    }
}

fn to_input(num: usize, len: usize) -> Vec<usize> {
    format!("{:0digits$}", num, digits = len)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn inc(mut num: usize, len: usize) -> usize {
    num += 1;
    while to_input(num, len).iter().any(|n| *n == 0) {
        num += 1;
    }
    num
}

fn dec(mut num: usize, len: usize) -> usize {
    num -= 1;
    while to_input(num, len).iter().any(|n| *n == 0) {
        num -= 1;
    }
    num
}

fn test_digits(
    len: usize,
    initial: usize,
    range: RangeInclusive<usize>,
    step_func: fn(usize, usize) -> usize,
) -> Option<usize> {
    let mut num = initial;
    while range.contains(&num) {
        let inputs: Vec<usize> = to_input(num, len);

        let mut z = inputs[0] + 15;
        z *= 26;
        z += inputs[1] + 10;
        z *= 26;
        z += inputs[2] + 2;
        z *= 26;
        z += inputs[3] + 16;
        let mut w = inputs[4];
        let mut x = z % 26;
        z /= 26;
        if x != w + 12 {
            num = step_func(num, len);
            continue;
        } else if len == 5 {
            return Some(num);
        }

        z *= 26;
        z += inputs[5] + 11;

        w = inputs[6];
        x = z % 26;
        z /= 26;
        if x != w + 9 {
            num = step_func(num, len);
            continue;
        } else if len == 7 {
            return Some(num);
        }

        z *= 26;
        z += inputs[7] + 16;
        z *= 26;
        z += inputs[8] + 6;

        w = inputs[9];
        x = z % 26;
        z /= 26;
        if x != w + 14 {
            num = step_func(num, len);
            continue;
        } else if len == 10 {
            return Some(num);
        }

        w = inputs[10];
        x = z % 26;
        z /= 26;
        if x != w + 11 {
            num = step_func(num, len);
            continue;
        } else if len == 11 {
            return Some(num);
        }

        w = inputs[11];
        x = z % 26;
        z /= 26;
        if x != w + 2 {
            num = step_func(num, len);
            continue;
        } else if len == 12 {
            return Some(num);
        }

        w = inputs[12];
        x = z % 26;
        z /= 26;
        if x != w + 16 {
            num = step_func(num, len);
            continue;
        } else if len == 13 {
            return Some(num);
        }

        w = inputs[13];
        x = z % 26;
        // z /= 26;
        if x != w + 14 {
            // z *= 26;
            // z += w + 13;
            // println!("failed z {}", z);
            num = step_func(num, len);
            continue;
        } else if len == 14 {
            return Some(num);
        }
    }
    None
}

trait NumberToInput {
    fn to_input(&self) -> Vec<Value>;
}
impl NumberToInput for usize {
    fn to_input(&self) -> Vec<Value> {
        format!("{:014}", self)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as Value)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Display, Formatter};
    use Operation::*;
    use Placeholder::*;

    const INPUT: &str = include_str!("../input/day24.txt");

    impl Alu {
        fn translate_program(&self) -> Vec<String> {
            let mut translations = vec![];
            let instructions: Vec<_> = self
                .program
                .clone()
                .into_iter()
                .filter(|Instruction { op, a: _, b }| {
                    // Omit z /= 1
                    !(op == &Div && b == &Some(Num(1)))
                })
                .collect();

            let mut it = instructions.iter().enumerate().peekable();
            while let Some((i, instr)) = it.next() {
                let Instruction { op, a, b } = instr;
                let translated = match (op, a, b) {
                    (Mul, _, Some(Num(0))) => {
                        // Simplify x *= 0 to x = 0

                        // Consolidate multiple lines with the same variable
                        let mut next_i = i + 1;
                        let mut same_var_instructions = vec![];
                        while let Some(next_instr) = instructions.get(next_i) {
                            next_i += 1;
                            let Instruction {
                                op: _,
                                a: next_a,
                                b: _,
                            } = next_instr;
                            if next_a == a {
                                same_var_instructions.push(it.next().unwrap().1);
                            } else {
                                break;
                            }
                        }
                        if !same_var_instructions.is_empty() {
                            let mut it2 = same_var_instructions.iter().peekable();
                            let first = it2.next().unwrap();
                            let Instruction { op, a, b: _ } = first;
                            assert_eq!(op, &Add);
                            let mut rhs = first.b_string();
                            while let Some(instr) = it2.next() {
                                let Instruction { op, a: _, b: _ } = instr;
                                match op {
                                    Add => rhs = format!("({} + {})", rhs, instr.b_string()),
                                    Mul => rhs = format!("({} * {})", rhs, instr.b_string()),
                                    Mod => rhs = format!("({} % {})", rhs, instr.b_string()),
                                    Eql => {
                                        let next_instr = it2.peek().unwrap();
                                        let Instruction { op, a: _, b } = &next_instr;
                                        let next_checks_eq_0 = op == &Eql && b == &Some(Num(0));
                                        if next_checks_eq_0 {
                                            it2.next();
                                            rhs = format!("({} != {})", rhs, instr.b_string())
                                        } else {
                                            rhs = format!("({} == {})", rhs, instr.b_string())
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            if rhs.starts_with('(') && rhs.ends_with(')') {
                                rhs.remove(rhs.len() - 1);
                                rhs.remove(0);
                            }
                            format!("{} = {}", a, rhs)
                        } else {
                            format!("{} = 0", a)
                        }
                    }
                    (Eql, a, _b) => {
                        // Simplify equal check from x = (if x == w { 1 } else { 0 }) to x = (x == w)

                        // Or further simplify x = (x == w) followed by x = (if x == 0 { 1 } else { 0 })
                        // to x = (x != w)

                        let next_instr = it.peek().unwrap();
                        let Instruction { op, a: next_a, b } = &next_instr.1;
                        let next_is_eql_0 = op == &Eql && next_a == a && *b == Some(Num(0));
                        if next_is_eql_0 {
                            it.next();
                            format!("{} = ({} != {})", a, a, instr.b_string())
                        } else {
                            format!("{} = ({} == {})", a, a, instr.b_string())
                        }
                    }
                    _ => instr.translate(),
                };

                translations.push(translated);
            }
            translations
        }
        fn run_program_with(&mut self, inputs: &[Value]) -> Vec<Value> {
            let mut i = 0; // input index

            for instr in &self.program {
                // println!("{:?}", instr);
                let Instruction { op, a, b } = instr;
                if op == &Input {
                    let a = self.variables.get_mut(a.to_var_idx()).unwrap();
                    *a = inputs[i];
                    i += 1;
                } else {
                    let b = match b.unwrap() {
                        Var(v) => self.variables[v.to_var_idx()],
                        Num(n) => n,
                    };
                    let a = self.variables.get_mut(a.to_var_idx()).unwrap();
                    match op {
                        Input => unreachable!(),
                        Add => *a += b,
                        Mul => *a *= b,
                        Div => *a /= b,
                        Mod => *a %= b,
                        Eql => *a = if *a == b { 1 } else { 0 },
                    }
                }
            }
            self.variables.clone()
        }
        fn reset_variables(&mut self) {
            self.variables = vec![Value::default(); 4];
        }
    }
    #[derive(Debug)]
    struct Alu {
        variables: Vec<Value>,
        program: Vec<Instruction>,
    }
    impl From<&str> for Alu {
        fn from(input: &str) -> Self {
            let program = input.trim().lines().map(Instruction::from).collect();
            let variables = vec![Value::default(); 4];
            Alu { program, variables }
        }
    }

    type Variable = char;
    trait VariableToIndex {
        fn to_var_idx(&self) -> usize;
    }
    impl VariableToIndex for Variable {
        fn to_var_idx(&self) -> usize {
            (*self as u8 - b'w') as usize
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Placeholder {
        Var(char),
        Num(Value),
    }
    impl Display for Placeholder {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Var(v) => v.to_string(),
                    Num(n) => n.to_string(),
                }
            )
        }
    }

    #[derive(Debug, Clone)]
    struct Instruction {
        op: Operation,
        a: Variable,
        b: Option<Placeholder>,
    }
    impl Instruction {
        fn translate(&self) -> String {
            let a = self.a;
            let b = self.b_string();
            match self.op {
                Input => format!("{} = next_input", a),
                Add => format!("{} += {}", a, b),
                Mul => format!("{} *= {}", a, b),
                Div => format!("{} /= {}", a, b),
                Mod => format!("{} %= {}", a, b),
                Eql => format!("x = (if {} == {} {{ 1 }} else {{ 0 }})", a, b),
            }
        }
        fn b_string(&self) -> String {
            self.b
                .map(|b| b.to_string())
                .unwrap_or_else(|| "inp have no b".to_string())
        }
    }
    #[cfg(test)]
    impl From<&str> for Instruction {
        fn from(line: &str) -> Self {
            let char_from = |c: &str| c.chars().next().unwrap();
            let parts: Vec<_> = line.split_whitespace().collect();
            let op = Operation::from(parts[0]);
            let a = char_from(parts[1]);
            let b = if op == Input {
                None
            } else {
                let b = parts[2];
                let b = if let Ok(n) = b.parse::<isize>() {
                    Num(n)
                } else {
                    Var(char_from(b))
                };
                Some(b)
            };
            Instruction { op, a, b }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    enum Operation {
        Input,
        Add,
        Mul,
        Div,
        Mod,
        Eql,
    }
    impl From<&str> for Operation {
        fn from(op: &str) -> Self {
            match op {
                "inp" => Input,
                "add" => Add,
                "mul" => Mul,
                "div" => Div,
                "mod" => Mod,
                "eql" => Eql,
                _ => unreachable!("{}", op),
            }
        }
    }

    #[test]
    fn test_variable_to_index() {
        assert_eq!(0, 'w'.to_var_idx());
        assert_eq!(1, 'x'.to_var_idx());
        assert_eq!(2, 'y'.to_var_idx());
        assert_eq!(3, 'z'.to_var_idx());
    }

    #[test]
    fn test_number_to_input() {
        assert_eq!(
            13579246899999.to_input(),
            vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9],
        );
        assert_eq!(0.to_input(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],);
    }

    #[test]
    fn part1_example_negate() {
        let mut alu = Alu::from(
            "\
inp x
mul x -1",
        );
        let results = alu.run_program_with(&[123]);
        assert_eq!(-123, results['x'.to_var_idx()]);
    }

    #[test]
    fn part1_example_check_three_times_larger() {
        let mut alu = Alu::from(
            "\
inp z
inp x
mul z 3
eql z x",
        );

        let results = alu.run_program_with(&[1, 2]);
        assert_eq!(0, results['z'.to_var_idx()]);

        alu.reset_variables();
        let results = alu.run_program_with(&[1, 3]);
        assert_eq!(1, results['z'.to_var_idx()]);
    }

    #[test]
    fn part1_example_binary_conversion() {
        let mut alu = Alu::from(
            "\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2",
        );

        let results = alu.run_program_with(&[15]);
        assert_eq!(1, results['w'.to_var_idx()]);
        assert_eq!(1, results['x'.to_var_idx()]);
        assert_eq!(1, results['y'.to_var_idx()]);
        assert_eq!(1, results['z'.to_var_idx()]);

        alu.reset_variables();
        let results = alu.run_program_with(&[10]);
        assert_eq!(1, results['w'.to_var_idx()]);
        assert_eq!(0, results['x'.to_var_idx()]);
        assert_eq!(1, results['y'.to_var_idx()]);
        assert_eq!(0, results['z'.to_var_idx()]);

        alu.reset_variables();
        let results = alu.run_program_with(&[0]);
        assert_eq!(0, results['w'.to_var_idx()]);
        assert_eq!(0, results['x'.to_var_idx()]);
        assert_eq!(0, results['y'.to_var_idx()]);
        assert_eq!(0, results['z'.to_var_idx()]);
    }

    // Slow: 14s on i9-9900K
    #[test]
    fn part1() {
        assert_eq!(89_959_794_919_939, day24_part1());
    }

    #[test]
    fn print_translated_program() {
        let alu = Alu::from(INPUT);
        let instructions = alu.translate_program();
        println!("{}", instructions.join("\n"));
    }

    // Slow: 9s on i9-9900K
    #[test]
    fn part2() {
        assert_eq!(17_115_131_916_112, day24_part2());
    }
}
