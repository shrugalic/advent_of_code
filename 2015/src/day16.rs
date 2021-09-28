use line_reader::read_file_to_lines;
use std::collections::HashMap;

pub(crate) fn day16_part1() -> usize {
    let input = read_file_to_lines("input/day16.txt");
    find_matching_memory_part1(input)
}

pub(crate) fn day16_part2() -> usize {
    let input = read_file_to_lines("input/day16.txt");
    find_matching_memory_part2(input)
}

fn find_matching_memory_part1(input: Vec<String>) -> usize {
    let analysis_result = analysis_result();
    for line in input {
        let (sue_number, memories) = extract_memories(line);
        if analysis_result.part1_matches(memories) {
            return sue_number;
        }
    }
    unreachable!()
}

fn find_matching_memory_part2(input: Vec<String>) -> usize {
    let analysis_result = analysis_result();
    for line in input {
        let (sue_number, memories) = extract_memories(line);
        if analysis_result.part2_matches(memories) {
            return sue_number;
        }
    }
    unreachable!()
}

fn analysis_result() -> HashMap<String, usize> {
    [
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]
    .iter()
    .cloned()
    .collect()
}

fn extract_memories(line: String) -> (usize, Vec<(String, usize)>) {
    let parts: Vec<_> = line.split_ascii_whitespace().collect();
    let extract_pair = |idx: usize| {
        (
            parts[idx].trim_end_matches(':').to_string(),
            parts[idx + 1].trim_end_matches(',').parse().unwrap(),
        )
    };
    let sue_number = parts[1].trim_end_matches(':').parse().unwrap();
    let memories = vec![extract_pair(2), extract_pair(4), extract_pair(6)];
    (sue_number, memories)
}

trait AnalysisResultMatchesMemories {
    fn part1_matches(&self, memories: Vec<(String, usize)>) -> bool {
        self.matches(memories, true)
    }
    fn part2_matches(&self, memories: Vec<(String, usize)>) -> bool {
        self.matches(memories, false)
    }
    fn matches(&self, memories: Vec<(String, usize)>, all_matches_equal: bool) -> bool {
        memories.into_iter().all(|(thing, count)| {
            if all_matches_equal {
                self.equal_match(thing, count)
            } else {
                self.unequal_match(thing, count)
            }
        })
    }
    fn equal_match(&self, thing: String, count: usize) -> bool;
    fn unequal_match(&self, thing: String, count: usize) -> bool;
}

impl AnalysisResultMatchesMemories for HashMap<String, usize> {
    fn equal_match(&self, thing: String, count: usize) -> bool {
        self.get(&thing).unwrap() == &count
    }
    fn unequal_match(&self, thing: String, count: usize) -> bool {
        let target = self.get(&thing).unwrap();
        match thing.as_str() {
            "cats" | "trees" => target < &count,
            "pomeranians" | "goldfish" => target > &count,
            _ => target == &count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(373, day16_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(260, day16_part2());
    }
}
