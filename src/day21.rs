use crate::device::{InstrPointer, RegisterIndex};
use crate::opcode::Number;

pub(crate) fn reversed_day21program() -> Vec<Number> {
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
                // It would halt here if r4 == r0, so remember this r4
                halting_values.push(r4);
                if halting_values.len() > 10 {
                    return halting_values;
                }
                // break; // break inner loop, re-init r3 and r4
            }
            r3 /= 256;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Device;
    use line_reader::read_file_to_lines;

    #[test]
    fn part1_with_instructions() {
        let program = read_file_to_lines("input/day21.txt");
        let mut device = Device::default();
        let halting_value = *device.halting_values(&program, 28, 4).first().unwrap();
        assert_eq!(103548, halting_value);
    }

    #[test]
    fn part1_with_reversed_program() {
        let halting_value = *reversed_day21program().first().unwrap();
        assert_eq!(103548, halting_value);
    }
}
