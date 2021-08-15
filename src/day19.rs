use crate::opcode::Number;

pub(crate) fn sum_of_divisors(number: Number) -> Number {
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
    use super::*;
    use crate::device::Device;
    use line_reader::{read_file_to_lines, read_str_to_lines};

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
        let program = read_str_to_lines(EXAMPLE_PROGRAM);
        assert_eq!(6, Device::default().run_program(&program));
    }

    #[test]
    fn part1() {
        let program = read_file_to_lines("input/day19.txt");
        assert_eq!(1872, Device::default().run_program(&program));
    }

    #[test]
    fn part2() {
        assert_eq!(
            1 + 2 + 5 + 10 + 1_055_143 + 2_110_286 + 5_275_715 + 10_551_430,
            sum_of_divisors(10_551_430)
        );
    }
}
