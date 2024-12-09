use std::collections::HashSet;
use std::iter;
use Region::{Free, Occupied};

const INPUT: &str = include_str!("../../2024/input/day09.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

type FileId = u16;

#[derive(Debug, Clone)]
enum Block {
    PartOfFile(FileId),
    Free,
}

fn solve_part1(input: &str) -> usize {
    let mut line = parse(input);
    if line.len() % 2 == 1 {
        line.push(0);
    }
    let mut blocks: Vec<Block> = Vec::with_capacity(line.len() * 5);
    for (i, w) in line.windows(2).enumerate().step_by(2) {
        let id = (i / 2) as FileId;
        let file_size = w[0];
        let free_space = w[1];
        for _ in 0..file_size {
            blocks.push(Block::PartOfFile(id));
        }
        for _ in 0..free_space {
            blocks.push(Block::Free);
        }
    }
    let mut first_free = blocks.iter().position(Block::is_free).unwrap();
    let mut last_full = blocks.iter().rposition(Block::is_part_of_file).unwrap();
    while first_free < last_full {
        blocks.swap(first_free, last_full);
        while blocks[first_free].is_part_of_file() {
            first_free += 1;
        }
        while blocks[last_full].is_free() {
            last_full -= 1;
        }
    }
    // println!("Final blocks: {}", blocks_to_string(&blocks));
    calculate_checksum(&blocks)
}

fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

impl Block {
    fn is_part_of_file(&self) -> bool {
        matches!(*self, Self::PartOfFile(_))
    }
    fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }
}

#[expect(dead_code)]
fn blocks_to_string(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(|v| match v {
            Block::Free => ".".to_string(),
            Block::PartOfFile(id) => id.to_string(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn calculate_checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, block)| match block {
            Block::PartOfFile(id) => Some(pos * *id as usize),
            Block::Free => None,
        })
        .sum()
}

type BlockCount = usize;

#[derive(Debug)]
enum Region {
    Occupied(FileId, BlockCount),
    Free(BlockCount),
}
fn solve_part2(input: &str) -> usize {
    let mut line = parse(input);
    if line.len() % 2 == 1 {
        line.push(0);
    }
    let mut regions: Vec<_> = Vec::with_capacity(line.len());
    for (i, w) in line.windows(2).enumerate().step_by(2) {
        let id = (i / 2) as FileId;
        let file_size = w[0] as usize;
        let space_size = w[1] as usize;
        regions.push(Occupied(id, file_size));
        if space_size > 0 {
            regions.push(Free(space_size));
        }
    }

    let mut back = regions.len() - 1;
    let mut touched = HashSet::new();
    while back > 0 {
        match regions[back] {
            Occupied(id, file_size) => {
                if !touched.insert(id) {
                    // Skip already touched files
                } else if let Some(front) = regions.iter().position(
                    |region| matches!(region, Free(space_size) if *space_size >= file_size),
                ) {
                    if front >= back {
                        // No space found
                    } else if let Free(space_size) = regions[front] {
                        regions.swap(front, back);
                        if space_size > file_size {
                            // Insert free space after the moved file to account for the difference
                            let extra_space = space_size - file_size;
                            regions.insert(front + 1, Free(extra_space));
                            back += 1;

                            // The space moved to the back should only be as large as the file was
                            let mut back_space_size = file_size;

                            // Defrag: Check if there's free space right before the inserted space
                            if let Free(before_size) = regions[back - 1] {
                                back_space_size += before_size;
                                regions.remove(back - 1);
                                back -= 1;
                            }

                            // Defrag: Check if there's free space right after the inserted space
                            if let Some(Free(after_size)) = regions.get(back + 1) {
                                back_space_size += after_size;
                                regions.remove(back + 1);
                            }
                            match regions[back] {
                                Free(ref mut space_size) => {
                                    *space_size = back_space_size;
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
            Free(_) => {}
        }
        back -= 1;
    }
    // println!("Final regions: {}", region_to_string(&regions));
    let blocks: Vec<_> = regions
        .into_iter()
        .flat_map(|region| match region {
            Occupied(id, file_size) => iter::repeat(Block::PartOfFile(id)).take(file_size),
            Free(space_size) => iter::repeat(Block::Free).take(space_size),
        })
        .collect();
    // println!("Final blocks: {}", blocks_to_string(&blocks));
    calculate_checksum(&blocks)
}

#[expect(dead_code)]
fn region_to_string(regions: &[Region]) -> String {
    regions
        .iter()
        .map(|region| match region {
            Occupied(id, file_size) => id.to_string().repeat(*file_size),
            Free(space_size) => ".".repeat(*space_size),
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2333133121414131402
";

    #[test]
    fn test_part1_example() {
        assert_eq!(1928, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(6279058075753, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2858, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6301361958738, solve_part2(INPUT));
    }
}
