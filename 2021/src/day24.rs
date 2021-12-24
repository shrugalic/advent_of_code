type Value = isize;

pub(crate) fn day24_part1() -> usize {
    let mut model_number = 99_999_999_999_999;

    // Still slow, 80s for 10M!

    let tries = 0; // 10_000_000_usize;
    let mut i = 0;

    let mut z = -1;
    while z != 0 {
        model_number -= 1;
        while model_number.to_input().iter().any(|n| *n == 0) {
            model_number -= 1;
        }

        i += 1;
        if i >= tries {
            break;
        }
        let inputs = &model_number.to_input();
        z = run_program_natively(inputs);
        println!("{}", z);
    }
    model_number
}

pub(crate) fn day24_part2() -> usize {
    0 // TODO
}

fn run_program_natively(inputs: &[Value]) -> Value {
    let (mut w, mut x, mut y, mut z); // = (0, 0, 0, 0);

    w = inputs[0];
    z = w + 15;

    w = inputs[1];
    z *= 26;
    y = w + 10;
    z += y;

    w = inputs[2];
    if (z % 26) + 12 != w {
        z *= 26;
        y = w + 2;
        z += y;
    }

    w = inputs[3];
    if (z % 26) + 13 != w {
        z *= 26;
        y = w + 16;
        z += y;
    }

    w = inputs[4];
    x = z % 26;
    z /= 26;
    x -= 12;
    if x != w {
        z *= 26;
        y = w + 12;
        z += y;
    }

    w = inputs[5];
    if (z % 26) + 10 != w {
        z *= 26;
        y = w + 11;
        z += y;
    }

    w = inputs[6];
    x = z % 26;
    z /= 26;
    x -= 9;
    if x != w {
        z *= 26;
        y = w + 5;
        z += y;
    }

    w = inputs[7];
    if (z % 26) + 14 != w {
        z *= 26;
        y = w + 16;
        z += y;
    }

    w = inputs[8];
    if (z % 26) + 13 != w {
        z *= 26;
        y = w + 6;
        z += y;
    }

    w = inputs[9];
    x = z % 26;
    z /= 26;
    x -= 14;
    if x != w {
        z *= 26;
        y = w + 15;
        z += y;
    }

    w = inputs[10];
    x = z % 26;
    z /= 26;
    x -= 11;
    if x != w {
        z *= 26;
        y = w + 3;
        z += y;
    }

    w = inputs[11];
    x = z % 26;
    z /= 26;
    x -= 2;
    if x != w {
        z *= 26;
        y = w + 12;
        z += y;
    }

    w = inputs[12];
    x = z % 26;
    z /= 26;
    x -= 16;
    if x != w {
        z *= 26;
        y = w + 10;
        z += y;
    }

    w = inputs[13];
    x = z % 26;
    z /= 26;
    x -= 14;
    if x != w {
        z *= 26;
        y = w + 13;
        z += y;
    }

    z
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

    impl ALU {
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
    struct ALU {
        variables: Vec<Value>,
        program: Vec<Instruction>,
    }
    impl From<&str> for ALU {
        fn from(input: &str) -> Self {
            let program = input.trim().lines().map(Instruction::from).collect();
            let variables = vec![Value::default(); 4];
            ALU { program, variables }
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
        let mut alu = ALU::from(
            "\
inp x
mul x -1",
        );
        let results = alu.run_program_with(&[123]);
        assert_eq!(-123, results['x'.to_var_idx()]);
    }

    #[test]
    fn part1_example_check_three_times_larger() {
        let mut alu = ALU::from(
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
        let mut alu = ALU::from(
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

    #[test]
    fn part1() {
        assert_eq!(1, day24_part1());
    }

    #[test]
    fn print_translated_program() {
        let alu = ALU::from(INPUT);
        let instructions = alu.translate_program();
        // println!("{}", instructions.join("\n"));
    }

    #[test]
    fn verify_native_program_has_same_results_as_interpreted() {
        let mut alu = ALU::from(INPUT);

        let mut model_number = 99_999_999_999_999;

        // Slow! 200k 5.5s, 300k 8.4s, 400k 11s, 500k 13s, 1m in 27s
        let tries = 0; //1_000_000_usize;
        let mut i = 0;

        let mut is_valid = false;
        while !is_valid {
            model_number -= 1;
            while model_number.to_input().iter().any(|n| *n == 0) {
                model_number -= 1;
            }
            alu.reset_variables();

            i += 1;
            if i >= tries {
                break;
            }
            let inputs = &model_number.to_input();
            let results = alu.run_program_with(inputs);
            let z = results['z'.to_var_idx()];
            assert_eq!(z, run_program_natively(inputs));
            is_valid = 0 == z;
        }
    }

    #[test]
    fn part2() {
        assert_eq!(1, day24_part2());
    }
}
