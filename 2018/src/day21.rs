use crate::opcode::Number;

pub(crate) fn reversed_day21program(limit: usize) -> Vec<Number> {
    let mut halting_values = vec![];

    let mut r4 = 0;
    let mut r3;
    loop {
        r3 = r4 | 65536;
        r4 = 10552971;
        loop {
            r4 += r3 & 255;
            r4 &= 16777215;
            r4 *= 65899;
            r4 &= 16777215;
            if 256 > r3 {
                break;
            }
            r3 /= 256;
        }
        // if r4 == r0 { return; }
        // It would halt here if r4 == r0, so remember this r4
        if halting_values.len() >= limit || halting_values.contains(&r4) {
            return halting_values;
        }
        halting_values.push(r4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Device;
    use line_reader::read_file_to_lines;

    #[test]
    fn part1() {
        let halting_value = *reversed_day21program(1).first().unwrap();
        assert_eq!(103548, halting_value);
    }

    #[test]
    fn compare_instructions_with_reversed_program() {
        let limit = 10;

        let program = read_file_to_lines("input/day21.txt");
        let mut device = Device::default();
        let halting_values_from_instructions = device.halting_values(&program, 28, 4, limit);

        let halting_values_from_program = reversed_day21program(limit);
        assert_eq!(
            halting_values_from_program,
            halting_values_from_instructions
        );
    }

    // #[test] // slow at ~5 minutes
    fn part2_with_instructions() {
        let program = read_file_to_lines("input/day21.txt");
        let mut device = Device::default();
        let halting_values = device.halting_values(&program, 28, 4, usize::MAX - 1);

        println!("{} values", halting_values.len());
        assert_eq!(14256686, *halting_values.last().unwrap());
    }

    #[test]
    fn part2_with_reversed_program() {
        let halting_values = reversed_day21program(usize::MAX);
        assert_eq!(14256686, *halting_values.last().unwrap());
    }
}
