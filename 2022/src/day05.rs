const INPUT: &str = include_str!("../input/day05.txt");

pub(crate) fn day05_part1() -> String {
    let (crates, commands) = parse(INPUT);
    move_crates_one_by_one(crates, commands)
}

pub(crate) fn day05_part2() -> String {
    let (crates, commands) = parse(INPUT);
    move_crates_in_bulk(crates, commands)
}

// Our crate index starts at 0, and their first element is the bottom-most one
type CrateConfig = Vec<Vec<char>>;

struct MoveCommand {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (CrateConfig, Vec<MoveCommand>) {
    let (crate_configs, commands) = input.split_once("\n\n").unwrap();

    // Reverse to get bottom-most crates first, and skip the line numbering the crate stacks
    let crate_configs: Vec<_> = crate_configs.lines().rev().skip(1).collect();
    // Example line: "[Z] [M] [P]"
    // Last column does not have trailing space -> add 1 to get proper column count
    const BYTES_PER_COLUMN: usize = 4;
    let count = (crate_configs.first().unwrap().as_bytes().len() + 1) / BYTES_PER_COLUMN;
    let mut crate_stacks = vec![Vec::with_capacity(crate_configs.len()); count];
    for line in crate_configs {
        for (i, stack) in line.as_bytes().chunks(BYTES_PER_COLUMN).enumerate() {
            // Example stack: "[Z] "
            let char = stack[1] as char;
            if char.is_alphabetic() {
                crate_stacks[i].push(char);
            }
        }
    }

    let commands: Vec<_> = commands
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            // Example: move 7 from 6 to 8
            // Index:   0    1 2    3 4  5
            let count = parts[1].parse().expect("contains count");
            // - 1 because our stack indices are 0-based
            let from = parts[3].parse::<usize>().expect("contains count") - 1;
            let to = parts[5].parse::<usize>().expect("contains count") - 1;
            MoveCommand { count, from, to }
        })
        .collect();
    (crate_stacks, commands)
}

fn move_crates_one_by_one(mut crates: CrateConfig, commands: Vec<MoveCommand>) -> String {
    for MoveCommand { count, from, to } in commands {
        let index = crates[from].len() - count;
        let removed: Vec<_> = crates[from].drain(index..).rev().collect();
        crates[to].extend(removed);
    }
    list_top_crates(&crates)
}

fn move_crates_in_bulk(mut crates: CrateConfig, commands: Vec<MoveCommand>) -> String {
    for MoveCommand { count, from, to } in commands {
        let index = crates[from].len() - count;
        let removed: Vec<_> = crates[from].drain(index..).collect();
        crates[to].extend(removed);
    }
    list_top_crates(&crates)
}

fn list_top_crates(crates: &CrateConfig) -> String {
    crates.iter().filter_map(|stack| stack.last()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        let (crate_config, commands) = parse(EXAMPLE);
        assert_eq!("CMZ", move_crates_one_by_one(crate_config, commands));
    }

    #[test]
    fn part2_example() {
        let (crate_config, commands) = parse(EXAMPLE);
        assert_eq!("MCD", move_crates_in_bulk(crate_config, commands));
    }

    #[test]
    fn part1() {
        assert_eq!("RTGWZTHLD", day05_part1());
    }

    #[test]
    fn part2() {
        assert_eq!("STHGRZZFR", day05_part2());
    }
}
