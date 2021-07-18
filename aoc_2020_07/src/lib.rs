use std::collections::{HashMap, HashSet};

// Returns the number of possibilities allowing a bag of the given target_color
fn number_of_bags_within_bag_of(target_color: &dyn AsRef<str>, rules: &[String]) -> usize {
    let bags_by_color = &convert_to_bags_by_color(rules);
    number_of_bags_within(target_color, bags_by_color) - 1 /* the outer-most bag should not count*/
}
fn number_of_bags_within(
    target_color: &dyn AsRef<str>,
    bags_by_color: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    1 + // 1 for the bag itself,
        // plus the inner bags it contains
        bags_by_color
        .get(target_color.as_ref())
        .unwrap()
        .iter()
        .map(|(color, count)| {
            // println!("{} contains {} * {}", target_color.as_ref(), count, color);
            count * number_of_bags_within(color, bags_by_color)
        })
        .sum::<usize>()
}

// Returns the number of possibilities allowing a bag of the given target_color
fn number_of_possible_bags_that_can_hold(target_color: &dyn AsRef<str>, rules: &[String]) -> usize {
    let bags_by_color = &convert_to_bags_by_color(rules);
    // Bags that can hold the target color directly
    let mut colors = colors_of_bags_that_can_hold(target_color, bags_by_color);
    loop {
        // Nested bags…
        let next = colors
            .iter()
            .map(|color| colors_of_bags_that_can_hold(color, bags_by_color))
            .fold(HashSet::new(), |a, b| a.union(&b).cloned().collect());
        let diff = next.difference(&colors).collect::<HashSet<_>>();
        if !diff.is_empty() {
            println!("Adding {} to previous of {}", diff.len(), colors.len());
            colors.extend(next);
        } else {
            break;
        }
    }

    colors.len()
}

fn colors_of_bags_that_can_hold(
    target_color: &dyn AsRef<str>,
    bags_by_color: &HashMap<String, HashMap<String, usize>>,
) -> HashSet<String> {
    bags_by_color
        .iter()
        .filter_map(|(outer_color, bag_counts_by_color)| {
            if bag_counts_by_color.contains_key(target_color.as_ref()) {
                // println!("'{}' can hold '{}'", outer_color, target_color.as_ref());
                Some(outer_color.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn convert_to_bags_by_color(rules: &[String]) -> HashMap<String, HashMap<String, usize>> {
    rules
        .iter()
        .map(|rule| {
            // Example rule: "light red bags contain 1 bright white bag, 2 muted yellow bags."
            if let Some((outer_color, bags_desc)) = rule.split_once(" bags contain ") {
                (String::from(outer_color), bag_counts_by_color(bags_desc))
            } else {
                panic!("Invalid rule: '{}'", rule);
            }
        })
        .collect()
}

// Returns a HashMap<color, count> of the described bags
fn bag_counts_by_color(bags_desc: &str) -> HashMap<String, usize> {
    if bags_desc == "no other bags." {
        HashMap::new()
    } else {
        let mut count_by_color = HashMap::new();
        // Example: "1 bright white bag, 2 muted yellow bags."
        let bags_desc: Vec<_> = bags_desc.split(", ").collect();
        for desc in bags_desc {
            // Examples: "1 bright white bag" or "2 muted yellow bags."
            if let Some((count_n_color, _whatever_suffix)) = desc.split_once(" bag") {
                // Examples: "1 bright white" or "2 muted yellow"
                if let Some((count, color)) = count_n_color.split_once(' ') {
                    count_by_color.insert(
                        color.to_string(),
                        count.parse().expect("bag count is a number"),
                    );
                } else {
                    panic!("Invalid count & color desc: '{}'", count_n_color);
                }
            } else {
                panic!("Invalid bag desc: '{}'", desc);
            }
        }
        count_by_color
    }
}

#[cfg(test)]
mod tests {
    use crate::{number_of_bags_within_bag_of, number_of_possible_bags_that_can_hold};
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn part1_example() {
        assert_eq!(
            number_of_possible_bags_that_can_hold(&"shiny gold", &read_str_to_lines(EXAMPLE_1)),
            4
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            number_of_possible_bags_that_can_hold(&"shiny gold", &read_file_to_lines("input.txt")),
            192
        );
    }

    const EXAMPLE_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part2_example1() {
        assert_eq!(
            number_of_bags_within_bag_of(&"shiny gold", &read_str_to_lines(EXAMPLE_1)),
            32
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            number_of_bags_within_bag_of(&"shiny gold", &read_str_to_lines(EXAMPLE_2)),
            126
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            number_of_bags_within_bag_of(&"shiny gold", &read_file_to_lines("input.txt")),
            12128
        );
    }
}
