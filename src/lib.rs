#[cfg(test)]
mod tests;

type Label = usize;

pub fn play(labels: &mut [Label], moves: usize) -> &mut [Label] {
    let len = labels.len() as Label;
    let inc = |a: Label, b: Label| (a + b) % len;
    let dec = |a: Label| if a > 1 { a - 1 } else { len };

    let mut round = 0;
    let mut curr: Label = 0;
    while round < moves {
        round += 1;
        let pickups = [inc(curr, 1), inc(curr, 2), inc(curr, 3)];
        let mut dest_label = dec(labels[curr as usize]);
        while pickups.iter().any(|p| labels[*p] == dest_label) {
            dest_label = dec(dest_label);
        }
        let dest = labels.iter().position(|l| *l == dest_label).unwrap();
        // println!(
        //     "curr: {} @ {}, pick ups: {} @ {}, {}, {}, dest {} @ {}",
        //     labels[curr],
        //     curr,
        //     labels[pickups[0]],
        //     pickups[0],
        //     labels[pickups[1]],
        //     labels[pickups[2]],
        //     dest_label,
        //     dest
        // );
        let mut start = pickups[0];
        let mut end = dest;
        if start < end {
            labels[start..=end].rotate_left(3);
        } else {
            let corr = end + 1;
            labels.rotate_left(corr);
            start -= corr;
            end = labels.len() + end - corr;
            labels[start..=end].rotate_left(3);
            labels.rotate_right(corr);
        }

        curr = inc(curr, 1);
    }
    labels
}

pub fn label_part1(result: &mut [Label]) -> &[Label] {
    let pos_of_1 = result.iter().position(|n| *n == 1).unwrap();
    result.rotate_left(pos_of_1);
    &result[1..]
}
