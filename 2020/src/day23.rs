pub type Label = usize;

pub(crate) fn play(labels: &mut [Label], moves: usize) -> Vec<Label> {
    let len = labels.len() as Label;
    let mut next_labels = convert_to_next_label_indices(labels);
    // println!("curr = {}, indices = {:?}", next_labels[0], next_labels);

    let mut round = 0;
    let mut curr = next_labels[0];
    while round < moves {
        round += 1;
        // println!("Curr {} points to pickup {}", curr, next_labels[curr]);
        // Get pickups
        let pickups = [
            next_labels[curr],
            next_labels[next_labels[curr]],
            next_labels[next_labels[next_labels[curr]]],
        ];

        // Remove pickups by letting curr point to what pickup_3 pointed to
        // println!(
        //     "Last pickup points to {}, let curr point to there",
        //     next_labels[pickups[2]]
        // );
        next_labels[curr] = next_labels[pickups[2]];

        // choose destination
        let mut dest = (len + curr - 2) % len + 1;
        // println!("Pickups = {:?}, tentative destination = {}", pickups, dest);
        while pickups.iter().any(|p| *p == dest) {
            dest = (len + dest - 2) % len + 1;
            // println!("Destination was pickup, new destination = {}", dest);
        }
        // println!(
        //     "Definite destination {} points to {}",
        //     dest, next_labels[dest]
        // );

        // insert pickups at destination
        // let pickup_3 point to what destination pointed to
        // println!(
        //     "Let last pickup {} point to {} instead",
        //     pickups[2], next_labels[dest]
        // );
        next_labels[pickups[2]] = next_labels[dest];
        // let destination point to pickup_1
        // println!(
        //     "And let destination {} point to first pickup {}",
        //     dest, pickups[0]
        // );
        next_labels[dest] = pickups[0];

        // let curr be one to the right
        curr = next_labels[curr];
    }
    next_labels
}

pub(crate) fn convert_back_to_labels(next_label: Vec<Label>) -> Vec<usize> {
    // Position 0 contains a copy of the current cup
    // Set it to 0 because we want to get the position of the original
    let mut labels = vec![0; next_label.len() - 1]; // len-1 because 0-based instead of 1-based
    labels[0] = next_label[0];
    // println!("Label[0] = {}", labels[0]);
    for i in 1..next_label.len() - 1 {
        // println!("Label[{}] = {}", i, next_label[labels[i - 1]]);
        labels[i] = next_label[labels[i - 1]];
    }
    labels
}

/// Returns a 1-based array, where the index is the label,
/// and the value is the next index/label
/// The 0th value holds a copy of the first/current index/label
fn convert_to_next_label_indices(labels: &[Label]) -> Vec<Label> {
    let len = labels.len();
    let mut next_label = vec![0; len + 1];
    labels.windows(2).for_each(|pair| {
        // println!("index {}: label {} & next {}", idx, pair[0], pair[1]);
        next_label[pair[0]] = pair[1];
    });
    next_label
        .iter_mut()
        .skip(len + 1)
        .enumerate()
        .for_each(|(i, v)| *v = i);

    let first = labels[0];
    let last = labels[len - 1];
    // Store first element at index 0, to let the caller know this info
    next_label[0] = first;
    // Let the last label point to first label, it's a circle after all
    next_label[last] = first;

    // println!("next_label = {:?}", next_label);
    next_label
}

pub(crate) fn label_part1(next_label: Vec<Label>) -> Vec<Label> {
    let mut labels = convert_back_to_labels(next_label);
    let pos_of_1 = labels.iter().position(|n| *n == 1).unwrap();
    labels.rotate_left(pos_of_1 + 1);
    labels.pop();
    labels
}

pub(crate) fn label_part2(next_label: Vec<Label>) -> usize {
    next_label[1] * next_label[next_label[1]]
}

pub(crate) const DAY23_PUZZLE_INPUT: &str = "327465189";

// 1_000 rounds take 23s, so 10M would take around 64 hours!
pub(crate) const DAY23_ROUND_COUNT: usize = 10_000_000;

pub(crate) fn input_to_vec(input: &str) -> Vec<Label> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Label)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "389125467";

    #[test]
    fn part1_example1_0rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 0)),
            input_to_vec("389125467")
        );
    }

    #[test]
    fn part1_example1_1round() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 1)),
            input_to_vec("328915467")
        );
    }

    #[test]
    fn part1_example1_2rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 2)),
            input_to_vec("325467891")
        );
    }

    #[test]
    fn part1_example1_3rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 3)),
            input_to_vec("346725891")
        );
    }

    #[test]
    fn part1_example1_4rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 4)),
            input_to_vec("325846791")
        );
    }

    #[test]
    fn part1_example1_5rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 5)),
            input_to_vec("367925841")
        );
    }

    #[test]
    fn part1_example1_6rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 6)),
            input_to_vec("367258419")
        );
    }

    #[test]
    fn part1_example1_7rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 7)),
            input_to_vec("367419258")
        );
    }

    #[test]
    fn part1_example1_8rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 8)),
            input_to_vec("392674158")
        );
    }

    #[test]
    fn part1_example1_9rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 9)),
            input_to_vec("392657418")
        );
    }

    #[test]
    fn part1_example1_10rounds() {
        assert_eq!(
            convert_back_to_labels(play(&mut input_to_vec(EXAMPLE_1), 10)),
            input_to_vec("374192658")
        );
    }

    #[test]
    fn part1_example1_label_10rounds() {
        assert_eq!(
            label_part1(play(&mut input_to_vec(EXAMPLE_1), 10)),
            input_to_vec("92658374")
        );
    }

    #[test]
    fn part1_example1_label_100rounds() {
        assert_eq!(
            label_part1(play(&mut input_to_vec(EXAMPLE_1), 100)),
            input_to_vec("67384529")
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            label_part1(play(&mut input_to_vec(DAY23_PUZZLE_INPUT), 100)),
            input_to_vec("82934675")
        );
    }

    #[test]
    fn part2_example1_label_10_000_000rounds() {
        let mut cups: Vec<Label> = (1..=1_000_000).into_iter().collect();
        input_to_vec(EXAMPLE_1)
            .iter()
            .enumerate()
            .for_each(|(i, v)| cups[i] = *v);

        assert_eq!(
            label_part2(play(&mut cups, DAY23_ROUND_COUNT)),
            934001 * 159792
        );
    }

    #[test]
    fn part2() {
        let mut cups: Vec<Label> = (1..=1_000_000).into_iter().collect();
        input_to_vec(DAY23_PUZZLE_INPUT)
            .iter()
            .enumerate()
            .for_each(|(i, v)| cups[i] = *v);

        assert_eq!(
            label_part2(play(&mut cups, DAY23_ROUND_COUNT)),
            749102 * 633559
        );
    }
}
