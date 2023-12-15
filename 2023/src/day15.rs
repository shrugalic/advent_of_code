use Operation::*;

const INPUT: &str = include_str!("../input/day15.txt");

pub(crate) fn part1() -> usize {
    sum_of_hash_values(INPUT)
}

pub(crate) fn part2() -> usize {
    sum_of_focussing_powers(INPUT)
}

fn sum_of_hash_values(input: &str) -> usize {
    parse(input).map(calculate_hash).map(|v| v as usize).sum()
}

fn sum_of_focussing_powers(input: &'static str) -> usize {
    apply_operations_to_empty_boxes(input)
        .into_iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            let box_num = box_idx + 1;
            box_num * focussing_power(lenses)
        })
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split(',')
}

fn calculate_hash(input: &str) -> u8 {
    input.chars().fold(0u8, hash_of)
}

fn apply_operations_to_empty_boxes(input: &'static str) -> Vec<Vec<Lens>> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for op in parse(input).map(Operation::from) {
        match op {
            Remove(label) => {
                let box_num = calculate_hash(label);
                let lenses = boxes.get_mut(box_num as usize).expect("a box of lenses");
                if let Some(index) = lenses.iter().position(|lens| lens.label == label) {
                    lenses.remove(index);
                }
            }
            Insert(new_lens) => {
                let box_num = calculate_hash(new_lens.label);
                let lenses = boxes.get_mut(box_num as usize).expect("a box of lenses");
                if let Some(old_lens) = lenses
                    .iter_mut()
                    .find(|old_lens| old_lens.label == new_lens.label)
                {
                    old_lens.focal_length = new_lens.focal_length;
                } else {
                    lenses.push(new_lens);
                }
            }
        }
    }
    boxes
}

fn hash_of(v: u8, c: char) -> u8 {
    (((v as u16 + c as u16) * 17) % 256) as u8
}

fn focussing_power(lenses: Vec<Lens>) -> usize {
    if lenses.is_empty() {
        0
    } else {
        lenses
            .into_iter()
            .enumerate()
            .map(|(lens_idx, lens)| {
                let lens_num = lens_idx + 1;
                lens_num * lens.focal_length as usize
            })
            .sum::<usize>()
    }
}

type FocalLength = u8;
type Label<'a> = &'a str;

enum Operation<'a> {
    Remove(Label<'a>),
    Insert(Lens<'a>),
}

#[derive(Clone)]
struct Lens<'a> {
    label: Label<'a>,
    focal_length: FocalLength,
}

impl<'a> From<&'static str> for Operation<'a> {
    fn from(value: &'static str) -> Self {
        if let Some(label) = value.strip_suffix('-') {
            Remove(label)
        } else if let Some((label, focal_length)) = value.split_once('=') {
            let focal_length = focal_length.parse().expect("valid focal length");
            let lens = Lens {
                label,
                focal_length,
            };
            Insert(lens)
        } else {
            unreachable!("Invalid input {value}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_calculate_hash() {
        assert_eq!(52, calculate_hash("HASH"));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(1_320, sum_of_hash_values(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(510_801, sum_of_hash_values(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(145, sum_of_focussing_powers(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(212_763, sum_of_focussing_powers(INPUT));
    }
}
