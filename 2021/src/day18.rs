use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use Element::*;

const INPUT: &str = include_str!("../input/day18.txt");

pub(crate) fn day18_part1() -> usize {
    Homework::from(INPUT).add().magnitude()
}

pub(crate) fn day18_part2() -> usize {
    Homework::from(INPUT).largest_magnitude()
}

#[derive(Debug, PartialEq)]
struct Homework {
    lines: Vec<SnailfishNumber>,
}
impl From<&str> for Homework {
    fn from(input: &str) -> Self {
        let lines = input.trim().lines().map(SnailfishNumber::from).collect();
        Homework { lines }
    }
}
impl Homework {
    fn add(self) -> SnailfishNumber {
        self.lines
            .into_iter()
            .reduce(|lhs, rhs| (lhs + rhs).reduce())
            .unwrap()
    }
    fn largest_magnitude(&self) -> usize {
        let mut largest_magnitude = 0;
        for left in 0..self.lines.len() - 1 {
            for right in left + 1..self.lines.len() {
                let left_right = (self.lines[left].clone() + self.lines[right].clone()).magnitude();
                largest_magnitude = largest_magnitude.max(left_right);
                let right_left = (self.lines[right].clone() + self.lines[left].clone()).magnitude();
                largest_magnitude = largest_magnitude.max(right_left);
            }
        }
        largest_magnitude
    }
}

type Value = usize;

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Open,  // Open
    Close, // Close
    Num(Value),
}
impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '[' => Open,
            ']' => Close,
            n => Num(n.to_digit(10).unwrap() as Value),
        }
    }
}

#[derive(PartialEq, Clone)]
struct SnailfishNumber {
    elements: Vec<Element>,
}
impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for w in self.elements.windows(2) {
            match &w {
                [Open, ..] => s.push('['),
                [Close, Close, ..] => s.push(']'),
                [Close, ..] => s.push_str("],"),
                [Num(val), Close, ..] => s.push_str(&val.to_string()),
                [Num(val), ..] => s.push_str(&format!("{},", val)),
                _ => unreachable!(),
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}
impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl SnailfishNumber {
    #[cfg(test)]
    fn pair(left: Value, right: Value) -> Self {
        Self {
            elements: vec![Open, Num(left), Num(right), Close],
        }
    }
    fn reduce(mut self) -> Self {
        let mut prev = String::new();
        // println!("reduce  {}", self.to_string());
        while self.to_string() != prev {
            prev = self.to_string();
            self = self.explode();
            if self.to_string() != prev {
                // println!("explode {}", self.to_string());
                continue; // at most one action per cycle
            }
            self = self.split();
            // println!("split   {}", self.to_string());
        }
        self
    }
    fn remove(&mut self, i: usize) {
        let _removed = self.elements.remove(i);
        // println!("removed {:?} @ {}", _removed, i);
    }
    fn explode(mut self) -> Self {
        if let Some((i, left, right)) = self.find_pair() {
            self.replace_pair_with_0(i);
            self.propagate_left(i - 1, left);
            self.propagate_right(i + 1, right)
        }
        self
    }
    fn find_pair(&self) -> Option<(usize, Value, Value)> {
        let mut open = 0;
        for (i, w) in self.elements.windows(3).enumerate() {
            match (open, w) {
                // Open range 4.. works fine, but IntelliJ still warns about it
                (4..=usize::MAX, [Open, Num(left), Num(right), ..]) => {
                    return Some((i, *left, *right))
                }
                (_, [Open, ..]) => open += 1,
                (_, [Close, ..]) => open -= 1,
                (_, _) => (),
            }
        }
        None
    }
    fn replace_pair_with_0(&mut self, i: usize) {
        // [ left right ]
        // i i+1  i+2   i+3
        self.remove(i + 3); // ]
        self.remove(i + 2); // right
        self.elements[i + 1] = Num(0); // left
        self.remove(i); // [
    }
    fn propagate_left(&mut self, mut l: usize, left: Value) {
        while l > 0 {
            if let Some(Num(val)) = self.elements.get_mut(l) {
                *val += left;
                break;
            }
            l = l.saturating_sub(1);
        }
    }
    fn propagate_right(&mut self, mut r: usize, right: Value) {
        while r < self.elements.len() {
            if let Some(Num(val)) = self.elements.get_mut(r) {
                *val += right;
                break;
            }
            r += 1;
        }
    }
    fn split(mut self) -> Self {
        if let Some((i, val)) = self.elements.iter().enumerate().find_map(|(i, e)| match e {
            // The largest encountered value is 48. But IntelliJ warns
            // about open range 10.., even though it works fine
            Num(val @ 10..=usize::MAX) => Some((i, *val)),
            _ => None,
        }) {
            // println!("Found value {} to split @ {}", val, i);
            self.remove(i);
            self.elements.insert(i, Close);
            self.elements.insert(i, Num((val + 1) / 2));
            self.elements.insert(i, Num(val / 2));
            self.elements.insert(i, Open);
        }
        self
    }
    fn magnitude(mut self) -> usize {
        self = self.reduce();
        // let mut elements = self.elements.clone();
        while let Some((i, left, right)) =
            self.elements
                .windows(4)
                .enumerate()
                .find_map(|(i, w)| match w {
                    [Open, Num(left), Num(right), Close] => Some((i, *left, *right)),
                    _ => None,
                })
        {
            self.remove(i);
            self.remove(i);
            self.remove(i);
            self.remove(i);
            self.elements.insert(i, Num(3 * left + 2 * right));
        }
        if let Num(magnitude) = self.elements[0] {
            magnitude
        } else {
            unreachable!()
        }
    }
}
impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, mut rhs: Self) -> Self::Output {
        let mut elements = self.elements;

        elements.insert(0, Open);
        elements.append(&mut rhs.elements);
        elements.push(Close);

        SnailfishNumber { elements }
    }
}
impl From<&str> for SnailfishNumber {
    fn from(line: &str) -> Self {
        let elements = line
            .trim()
            .chars()
            .filter(|c| c != &',')
            .map(Element::from)
            .collect();
        SnailfishNumber { elements }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    const EXAMPLE2: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_pair_of_two_regular_numbers() {
        let line = "[1,2]";
        let number = SnailfishNumber::from(line);
        let elements = vec![Open, Num(1), Num(2), Close];
        assert_eq!(number, SnailfishNumber { elements });
        assert_eq!(number, SnailfishNumber::pair(1, 2));
        assert_eq!(number.to_string(), line);
    }

    #[test]
    fn test_complex_to_string() {
        let line = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let number = SnailfishNumber::from(line);
        assert_eq!(line, number.to_string())
    }

    #[test]
    fn test_pair_of_pair_and_number() {
        let line = "[[1,2],3]";
        assert_eq!(SnailfishNumber::from(line).to_string(), line);
    }

    #[test]
    fn test_pair_of_number_and_pair() {
        let line = "[9,[8,7]]";
        assert_eq!(SnailfishNumber::from(line).to_string(), line);
    }

    #[test]
    fn test_pair_of_pairs() {
        let line = "[[1,9],[8,5]]";
        assert_eq!(SnailfishNumber::from(line).to_string(), line);
    }

    #[test]
    fn test_pair_of_nested_1() {
        let line = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        assert_eq!(SnailfishNumber::from(line).to_string(), line);
    }

    #[test]
    fn test_pair_of_nested_2() {
        let line = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
        assert_eq!(SnailfishNumber::from(line).to_string(), line);
    }

    #[test]
    fn test_pair_of_nested_3() {
        let line = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        assert_eq!(SnailfishNumber::from(line).to_string(), line);
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            SnailfishNumber::from("[[[[[9,8],1],2],3],4]").explode(),
            SnailfishNumber::from("[[[[0,9],2],3],4]")
        );
        assert_eq!(
            SnailfishNumber::from("[7,[6,[5,[4,[3,2]]]]]").explode(),
            SnailfishNumber::from("[7,[6,[5,[7,0]]]]")
        );
        assert_eq!(
            SnailfishNumber::from("[[6,[5,[4,[3,2]]]],1]").explode(),
            SnailfishNumber::from("[[6,[5,[7,0]]],3]")
        );
        assert_eq!(
            SnailfishNumber::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").explode(),
            SnailfishNumber::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        assert_eq!(
            SnailfishNumber::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").explode(),
            SnailfishNumber::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            SnailfishNumber::pair(9, 1).split(),
            SnailfishNumber::from("[9,1]")
        );
        assert_eq!(
            SnailfishNumber::pair(10, 1).split(),
            SnailfishNumber::from("[[5,5],1]")
        );
        assert_eq!(
            SnailfishNumber::pair(11, 1).split(),
            SnailfishNumber::from("[[5,6],1]")
        );
    }

    #[test]
    fn test_add_assign() {
        let lhs = SnailfishNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = SnailfishNumber::from("[1,1]");
        assert_eq!(
            lhs + rhs,
            SnailfishNumber::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Homework::from("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]").add(),
            SnailfishNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_add_without_reduce() {
        assert_eq!(
            Homework::from("[1,1]\n[2,2]\n[3,3]\n[4,4]").add(),
            SnailfishNumber::from("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        )
    }

    #[test]
    fn test_add_with_single_reduce() {
        assert_eq!(
            Homework::from("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]")
                .add()
                .reduce(),
            SnailfishNumber::from("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        )
    }

    #[test]
    fn test_add_with_double_reduce() {
        assert_eq!(
            Homework::from("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]")
                .add()
                .reduce(),
            SnailfishNumber::from("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        )
    }

    #[test]
    fn part1_slightly_larger_example_add_step_by_step() {
        let lines: Vec<&str> = EXAMPLE.trim().lines().collect();
        let expecteds = vec![
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ];
        assert_eq!(
            (SnailfishNumber::from(lines[0]) + SnailfishNumber::from(lines[1])).reduce(),
            SnailfishNumber::from(expecteds[0])
        );
        for i in 1..expecteds.len() {
            assert_eq!(
                (SnailfishNumber::from(expecteds[i - 1]) + SnailfishNumber::from(lines[i + 1]))
                    .reduce(),
                SnailfishNumber::from(expecteds[i])
            );
        }
    }

    #[test]
    fn part1_slightly_larger_example_add() {
        assert_eq!(
            Homework::from(EXAMPLE).add(),
            SnailfishNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn part1_example_add() {
        assert_eq!(
            Homework::from(EXAMPLE2).add(),
            SnailfishNumber::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
    }

    #[test]
    fn test_magnitudes() {
        assert_eq!(SnailfishNumber::from("[9,1]").magnitude(), 3 * 9 + 2);
        assert_eq!(SnailfishNumber::from("[1,9]").magnitude(), 3 + 2 * 9);
        assert_eq!(
            SnailfishNumber::from("[[9,1],[1,9]]").magnitude(),
            3 * 29 + 2 * 21
        );
    }

    #[test]
    fn part1_example_magnitude() {
        assert_eq!(Homework::from(EXAMPLE2).add().magnitude(), 4140);
    }

    #[test]
    fn part1() {
        assert_eq!(4072, day18_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(3993, Homework::from(EXAMPLE2).largest_magnitude());
    }

    #[test]
    fn part2() {
        assert_eq!(4483, day18_part2());
    }
}
