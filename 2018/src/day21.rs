#[cfg(test)]
mod tests {
    use crate::device::Device;
    use line_reader::read_file_to_lines;

    #[test]
    fn part1() {
        let program = read_file_to_lines("input/day21.txt");
        let mut device = Device::default();
        let halting_value = device.halting_value(&program, 28, 4);
        assert_eq!(103548, halting_value);
    }
}
