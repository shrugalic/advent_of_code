use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day07.txt");

/// A path to directory or file
type Path = Vec<String>;

/// A directory or file name and size
type NameAndSize = (&'static str, usize);

pub(crate) fn day07_part1() -> usize {
    let lines = parse(INPUT);
    let file_file_name_and_size_by_path = file_file_name_and_size_by_path(lines);
    let dir_size_by_path = sum_of_directory_sizes(file_file_name_and_size_by_path);
    sum_of_directories_smaller_than_100_000(dir_size_by_path)
}

pub(crate) fn day07_part2() -> usize {
    let lines = parse(INPUT);
    let file_name_and_size_by_path = file_file_name_and_size_by_path(lines);
    let dir_size_by_path = sum_of_directory_sizes(file_name_and_size_by_path);
    smallest_directory_large_enough(dir_size_by_path)
}

fn sum_of_directory_sizes(
    file_name_and_size_by_path: HashMap<Path, Vec<NameAndSize>>,
) -> HashMap<String, usize> {
    // max-depth to start directory size accumulation
    let mut depth: usize = file_name_and_size_by_path
        .keys()
        .map(|path| path.len())
        .max()
        .expect("at least one directory");
    let mut dir_size_by_path: HashMap<String, usize> = HashMap::new();
    while depth > 0 {
        for (path, sizes_and_names) in file_name_and_size_by_path
            .iter()
            .filter(|(path, _)| path.len() == depth)
        {
            let dir = path.join("");
            let mut total_size = 0;
            for (name, file_size) in sizes_and_names {
                if *file_size == 0 {
                    // directory -> find its size
                    let sub_dir = format!("{dir}{name}/");
                    total_size += dir_size_by_path
                        .get(&sub_dir)
                        .expect("directory to be present");
                } else {
                    // file -> has known size
                    total_size += file_size;
                }
            }
            dir_size_by_path.insert(dir, total_size);
        }
        depth -= 1;
    }
    dir_size_by_path
}

fn sum_of_directories_smaller_than_100_000(dir_size_by_path: HashMap<String, usize>) -> usize {
    dir_size_by_path
        .values()
        .filter(|&&size| size <= 100_000)
        .sum()
}

const DISK_SIZE: usize = 70_000_000;
const TOTAL_NEEDED_DISK_SPACE: usize = 30_000_000;

fn smallest_directory_large_enough(dir_size_by_path: HashMap<String, usize>) -> usize {
    let total_size = dir_size_by_path.get("/").expect("a root directory");
    let free_disk_space = DISK_SIZE - total_size;
    let needed_disk_space = TOTAL_NEEDED_DISK_SPACE - free_disk_space;
    *dir_size_by_path
        .values()
        .filter(|&&size| size >= needed_disk_space)
        .min()
        .expect("at least one entry to get min of")
}

fn file_file_name_and_size_by_path(lines: Vec<&str>) -> HashMap<Path, Vec<(&str, usize)>> {
    let mut directory_stack = vec![];
    let mut file_name_and_size_by_path: HashMap<Path, Vec<(&str, usize)>> = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    "/" => directory_stack.push("/".to_string()),
                    ".." => {
                        directory_stack.pop();
                    }
                    dir => directory_stack.push(format!("{dir}/")),
                },
                "ls" => {}
                cmd => unreachable!("unexpected command '{}'", cmd),
            },
            dir_or_file => {
                let contents = file_name_and_size_by_path
                    .entry(directory_stack.clone())
                    .or_default();
                let size = if dir_or_file == "dir" {
                    0
                } else {
                    dir_or_file.parse().expect("a file size")
                };
                contents.push((parts[1], size));
            }
        }
    }
    file_name_and_size_by_path
}

fn parse(lines: &str) -> Vec<&str> {
    lines.trim().lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_example() {
        let lines = parse(EXAMPLE);
        let file_name_and_size_by_path = file_file_name_and_size_by_path(lines);
        let dir_size_by_path = sum_of_directory_sizes(file_name_and_size_by_path);
        assert_eq!(
            94853 + 584, // = 95437
            sum_of_directories_smaller_than_100_000(dir_size_by_path)
        );
    }

    #[test]
    fn part2_example() {
        let lines = parse(EXAMPLE);
        let file_name_and_size_by_path = file_file_name_and_size_by_path(lines);
        let dir_size_by_path = sum_of_directory_sizes(file_name_and_size_by_path);
        assert_eq!(
            24_933_642,
            smallest_directory_large_enough(dir_size_by_path)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(1_543_140, day07_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1_117_448, day07_part2());
    }
}
