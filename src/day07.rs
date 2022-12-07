use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day07.txt");

pub(crate) fn day07_part1() -> usize {
    let file_list = generate_file_list(INPUT);
    let dir_size_by_path = sum_of_directory_sizes(file_list);
    sum_of_directories_smaller_than_100_000(dir_size_by_path)
}

pub(crate) fn day07_part2() -> usize {
    let file_name_and_size_by_path = generate_file_list(INPUT);
    let dir_size_by_path = sum_of_directory_sizes(file_name_and_size_by_path);
    smallest_directory_large_enough(dir_size_by_path)
}

fn generate_file_list(input: &'static str) -> Vec<FileOrDir> {
    let mut path = vec![];
    let mut files_and_directories = Vec::new();
    for line in input.trim().lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    "/" => path.push("/".to_string()),
                    ".." => {
                        path.pop();
                    }
                    dir => path.push(format!("{dir}/")),
                },
                "ls" => {}
                cmd => unreachable!("unexpected command '{}'", cmd),
            },
            "dir" => {
                files_and_directories.push(FileOrDir::directory(&path.join(""), parts[1]));
            }
            file_size => {
                let file_size = file_size.parse().expect("a file size");
                files_and_directories.push(FileOrDir::file(&path.join(""), parts[1], file_size))
            }
        }
    }
    files_and_directories
}

fn sum_of_directory_sizes(file_list: Vec<FileOrDir>) -> HashMap<String, usize> {
    // max-depth to start directory size accumulation
    let mut depth: usize = file_list
        .iter()
        .map(FileOrDir::depth)
        .max()
        .expect("at least one directory");
    let mut dir_size_by_path: HashMap<String, usize> = HashMap::new();
    while depth > 0 {
        for file_or_dir in file_list.iter().filter(|&file| file.depth() == depth) {
            match file_or_dir {
                FileOrDir::File { size, .. } => {
                    *dir_size_by_path.entry(file_or_dir.parent()).or_default() += size
                }
                FileOrDir::Directory { .. } => {
                    // Append trailing slash to directory path to look same as parent
                    let path = format!("{}/", file_or_dir.path());
                    let dir_size = *dir_size_by_path
                        .get(&path)
                        .expect("directory to be present");
                    *dir_size_by_path.entry(file_or_dir.parent()).or_default() += dir_size;
                }
            }
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
const NEEDED_SPACE: usize = 30_000_000;
fn smallest_directory_large_enough(dir_size_by_path: HashMap<String, usize>) -> usize {
    let total_used = dir_size_by_path.get("/").expect("a root directory");
    let free_space = DISK_SIZE - total_used;
    let min_space_to_clear = NEEDED_SPACE - free_space;
    *dir_size_by_path
        .values()
        .filter(|&&dir_size| dir_size >= min_space_to_clear)
        .min()
        .expect("at least one entry to get min of")
}

enum FileOrDir {
    File { size: usize, path: String },
    Directory { path: String },
}
impl FileOrDir {
    fn directory(parent: &str, name: &str) -> Self {
        let path = format!("{parent}{name}");
        FileOrDir::Directory { path }
    }
    fn file(parent: &str, name: &str, size: usize) -> Self {
        let path = format!("{parent}{name}");
        FileOrDir::File { size, path }
    }
    fn depth(&self) -> usize {
        self.path().chars().filter(|&c| c == '/').count()
    }
    fn path(&self) -> &str {
        match self {
            FileOrDir::File { path, .. } => path,
            FileOrDir::Directory { path } => path,
        }
    }
    fn parent(&self) -> String {
        let path = self.path();
        if let Some(i) = path.rfind('/') {
            path[0..=i].to_string()
        } else {
            unreachable!();
        }
    }
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
        let file_name_and_size_by_path = generate_file_list(EXAMPLE);
        let dir_size_by_path = sum_of_directory_sizes(file_name_and_size_by_path);
        assert_eq!(
            94853 + 584, // = 95437
            sum_of_directories_smaller_than_100_000(dir_size_by_path)
        );
    }

    #[test]
    fn part2_example() {
        let file_name_and_size_by_path = generate_file_list(EXAMPLE);
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
