use crate::device::Device;
use crate::opcode::Number;
use crate::parse;

const INPUT: &str = include_str!("../input/day19.txt");

pub(crate) fn day19_part2() -> Number {
    sum_of_divisors(10_551_430)
}

pub(crate) fn day19_part1() -> Number {
    let program = parse(INPUT);
    Device::default().run_program(&program)
}

fn sum_of_divisors(number: Number) -> Number {
    let mut sum = 0;
    let mut divisor = 1;
    while divisor <= number {
        if number % divisor == 0 {
            sum += divisor;
        }
        divisor += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::device::Device;
    use crate::parse;

    use super::*;

    const EXAMPLE_PROGRAM: &str = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn run_example_program() {
        let program = parse(EXAMPLE_PROGRAM);
        assert_eq!(6, Device::default().run_program(&program));
    }

    #[test]
    fn part1() {
        assert_eq!(1872, day19_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(
            1 + 2 + 5 + 10 + 1_055_143 + 2_110_286 + 5_275_715 + 10_551_430,
            day19_part2()
        );
    }
}
