#[cfg(test)]
mod tests;

type Label = usize;

pub fn play(labels: &mut [Label], moves: usize) -> Vec<Label> {
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

pub fn convert_back_to_labels(next_label: Vec<Label>) -> Vec<usize> {
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

pub fn label_part1(next_label: Vec<Label>) -> Vec<Label> {
    let mut labels = convert_back_to_labels(next_label);
    let pos_of_1 = labels.iter().position(|n| *n == 1).unwrap();
    labels.rotate_left(pos_of_1 + 1);
    labels.pop();
    labels
}

pub fn label_part2(next_label: Vec<Label>) -> usize {
    next_label[1] * next_label[next_label[1]]
}
