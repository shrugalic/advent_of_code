use std::iter;

const INPUT: &str = include_str!("../../2024/input/day09.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

type FileId = u16;

type BlockCount = u8;

struct SizePair {
    file_size: BlockCount,
    free: BlockCount,
}

#[derive(Debug, Clone)]
enum Block {
    PartOfFile(FileId),
    Free,
}

fn solve_part1(input: &str) -> usize {
    let dense_representation = parse(input);
    let mut blocks: Vec<Block> = dense_representation
        .into_iter()
        .enumerate()
        .flat_map(|(id, SizePair { file_size, free })| {
            iter::repeat(Block::PartOfFile(id as FileId))
                .take(file_size as usize)
                .chain(iter::repeat(Block::Free).take(free as usize))
        })
        .collect();

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

fn parse(input: &str) -> Vec<SizePair> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|n| n as u8))
        .chain(iter::once(0)) // make windows(2) work for odd-lengths
        .collect::<Vec<_>>()
        .windows(2)
        .step_by(2)
        .map(|w| SizePair {
            file_size: w[0],
            free: w[1],
        })
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

#[derive(Debug)]
struct File {
    id: FileId,
    start: usize,
    size: BlockCount,
}
impl File {
    fn next_block(&self) -> usize {
        self.start + self.size as usize
    }
    fn check_sum(&self) -> usize {
        self.id as usize * (self.start..self.start + self.size as usize).sum::<usize>()
    }
}

fn solve_part2(input: &str) -> usize {
    let dense_representation = parse(input);
    let mut files = Vec::with_capacity(dense_representation.len());
    let mut start = 0;
    for (i, SizePair { file_size, free }) in dense_representation.into_iter().enumerate() {
        files.push(File {
            id: i as FileId,
            start,
            size: file_size,
        });
        start += (file_size + free) as usize;
    }

    let mut file_ids: Vec<_> = files.iter().map(|f| f.id).collect();
    while let Some(id) = file_ids.pop() {
        let idx = files.iter().position(|f| f.id == id).unwrap();
        let file = &files[idx];
        if let Some(start_of_free_space) = files.windows(2).find_map(|f| {
            let space_between = f[1].start - f[0].next_block();
            (space_between >= file.size as usize && f[1].start <= file.start)
                .then(|| f[0].next_block())
        }) {
            files[idx].start = start_of_free_space;
            files.sort_by_key(|f| f.start);
        }
    }
    files.iter().map(File::check_sum).sum()
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
