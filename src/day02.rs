use crate::parse;

const INPUT: &str = include_str!("../input/day02.txt");

pub(crate) fn day02_part1() -> usize {
    total_wrapping_paper_needed(parse(INPUT))
}
pub(crate) fn day02_part2() -> usize {
    total_ribbon_needed(parse(INPUT))
}

struct Box {
    h: usize,
    l: usize,
    w: usize,
}
impl<T> From<T> for Box
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let mut dims: Vec<_> = s.as_ref().split('x').map(|d| d.parse().unwrap()).collect();
        dims.sort_unstable(); // smallest sides first
        Box {
            h: dims[0],
            l: dims[1],
            w: dims[2],
        }
    }
}
impl Box {
    fn wrapping_paper_needed(&self) -> usize {
        2 * self.h * self.l + 2 * self.h * self.w + 2 * self.l * self.w + self.h * self.l
    }
    fn ribbon_needed(&self) -> usize {
        2 * self.h + 2 * self.l + self.h * self.l * self.w
    }
}

fn total_wrapping_paper_needed(input: Vec<&str>) -> usize {
    input
        .iter()
        .map(|s| Box::from(s).wrapping_paper_needed())
        .sum()
}

fn total_ribbon_needed(input: Vec<&str>) -> usize {
    input.iter().map(|s| Box::from(s).ribbon_needed()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(58, Box::from("2x3x4").wrapping_paper_needed());
        assert_eq!(43, Box::from("1x1x10").wrapping_paper_needed());
    }

    #[test]
    fn part1() {
        assert_eq!(1588178, day02_part1());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(34, Box::from("2x3x4").ribbon_needed());
        assert_eq!(14, Box::from("1x1x10").ribbon_needed());
    }

    #[test]
    fn part2() {
        assert_eq!(3783758, day02_part2());
    }
}
