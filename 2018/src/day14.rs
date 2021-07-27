pub(crate) fn score_of_10_recipes_after(count: usize) -> String {
    let mut list = Vec::with_capacity(count + 10 + 1);
    list.push(3);
    list.push(7);

    let mut first = 0;
    let mut second = 1;
    while list.len() < count + 10 {
        let sum = list[first] + list[second];
        if sum < 10 {
            list.push(sum);
        } else {
            list.push(sum / 10);
            list.push(sum % 10);
        }
        first = (first + 1 + list[first]) % list.len();
        second = (second + 1 + list[second]) % list.len();
    }
    // println!("{:?}, {}, {}", list, first, second);
    list.iter()
        .skip(count)
        .take(10)
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_9_recipes() {
        assert_eq!("5158916779", score_of_10_recipes_after(9));
    }

    #[test]
    fn part1_example_5_recipes() {
        assert_eq!("0124515891", score_of_10_recipes_after(5));
    }

    #[test]
    fn part1_example_18_recipes() {
        assert_eq!("9251071085", score_of_10_recipes_after(18));
    }

    #[test]
    fn part1_example_2018_recipes() {
        assert_eq!("5941429882", score_of_10_recipes_after(2018));
    }

    #[test]
    fn part1() {
        assert_eq!("1411383621", score_of_10_recipes_after(760_221));
    }
}
