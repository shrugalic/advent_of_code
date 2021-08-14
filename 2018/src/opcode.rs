pub(crate) type Number = usize;
type Input = Number;
type Output = Number;
pub(crate) type Values = (Input, Input, Output);
pub(crate) type Register = Vec<Number>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Op {
    AddRegister,  // stores into register C the result of adding register A and register B.
    AddImmediate, // stores into register C the result of adding register A and value B.
    MultiplyRegister, // stores into register C the result of multiplying register A and register B.
    MultiplyImmediate, // stores into register C the result of multiplying register A and value B.
    BitwiseAndRegister, // stores into register C the result of the bitwise AND of register A and register B.
    BitwiseAndImmediate, // stores into register C the result of the bitwise AND of register A and value B.
    BitwiseOrRegister, // stores into register C the result of the bitwise OR of register A and register B.
    BitwiseOrImmediate, // stores into register C the result of the bitwise OR of register A and value B.
    SetRegister,        // copies the contents of register A into register C. (Input B is ignored.)
    SetImmediate,       // stores value A into register C. (Input B is ignored.)
    GreaterThanImmediateRegister, // sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    GreaterThanRegisterImmediate, // sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0
    GreaterThanRegisterRegister, // sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    EqualImmediateRegister, // sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    EqualRegisterImmediate, // sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    EqualRegisterRegister, // sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

pub(crate) const ALL_OPS: [Op; 16] = [
    Op::AddRegister,
    Op::AddImmediate,
    Op::MultiplyRegister,
    Op::MultiplyImmediate,
    Op::BitwiseAndRegister,
    Op::BitwiseAndImmediate,
    Op::BitwiseOrRegister,
    Op::BitwiseOrImmediate,
    Op::SetRegister,
    Op::SetImmediate,
    Op::GreaterThanImmediateRegister,
    Op::GreaterThanRegisterImmediate,
    Op::GreaterThanRegisterRegister,
    Op::EqualImmediateRegister,
    Op::EqualRegisterImmediate,
    Op::EqualRegisterRegister,
];

impl<T: AsRef<str>> From<T> for Op {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "addr" => Op::AddRegister,
            "addi" => Op::AddImmediate,
            "mulr" => Op::MultiplyRegister,
            "muli" => Op::MultiplyImmediate,
            "banr" => Op::BitwiseAndRegister,
            "bani" => Op::BitwiseAndImmediate,
            "borr" => Op::BitwiseOrRegister,
            "bori" => Op::BitwiseOrImmediate,
            "setr" => Op::SetRegister,
            "seti" => Op::SetImmediate,
            "gtir" => Op::GreaterThanImmediateRegister,
            "gtri" => Op::GreaterThanRegisterImmediate,
            "gtrr" => Op::GreaterThanRegisterRegister,
            "eqir" => Op::EqualImmediateRegister,
            "eqri" => Op::EqualRegisterImmediate,
            "eqrr" => Op::EqualRegisterRegister,
            op => panic!("Illegal op code {}", op),
        }
    }
}

impl Op {
    pub(crate) fn execute(&self, register: &mut Register, values: &Values) {
        let (a, b, c) = *values;
        let bool_to_number = |condition| if condition { 1 } else { 0 };
        register[c] = match self {
            Op::AddRegister => register[a] + register[b],
            Op::AddImmediate => register[a] + b,
            Op::MultiplyRegister => register[a] * register[b],
            Op::MultiplyImmediate => register[a] * b,
            Op::BitwiseAndRegister => register[a] & register[b],
            Op::BitwiseAndImmediate => register[a] & b,
            Op::BitwiseOrRegister => register[a] | register[b],
            Op::BitwiseOrImmediate => register[a] | b,
            Op::SetRegister => register[a],
            Op::SetImmediate => a,
            Op::GreaterThanImmediateRegister => bool_to_number(a > register[b]),
            Op::GreaterThanRegisterImmediate => bool_to_number(register[a] > b),
            Op::GreaterThanRegisterRegister => bool_to_number(register[a] > register[b]),
            Op::EqualImmediateRegister => bool_to_number(a == register[b]),
            Op::EqualRegisterImmediate => bool_to_number(register[a] == b),
            Op::EqualRegisterRegister => bool_to_number(register[a] == register[b]),
        };
    }
}
