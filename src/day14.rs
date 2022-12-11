pub(crate) fn day14_part1() -> String {
    score_of_10_recipes_after(760_221)
}

pub(crate) fn day14_part2() -> usize {
    recipe_count_until_this_score_appears("760_221")
}

fn score_of_10_recipes_after(count: usize) -> String {
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

fn recipe_count_until_this_score_appears<T: AsRef<str>>(score: T) -> usize {
    let digits = score
        .as_ref()
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut list = vec![3, 7];
    let mut first = 0;
    let mut second = 1;

    let mut matched_digits = 0;
    loop {
        let sum = list[first] + list[second];
        if sum < 10 {
            list.push(sum);
        } else {
            list.push(sum / 10);
            if sum / 10 == digits[matched_digits] {
                matched_digits += 1;
                // Stop if we reached the goal with only the first of 2 new digits
                if matched_digits == digits.len() {
                    break;
                }
            } else if matched_digits > 0 {
                matched_digits = 0;
            }
            list.push(sum % 10);
        }
        // This also works for the sum < 10 part
        if sum % 10 == digits[matched_digits] {
            matched_digits += 1;
        } else if matched_digits > 0 {
            matched_digits = 0;
            // The following is when only the second of 2 new digits matches
            if sum % 10 == digits[matched_digits] {
                matched_digits += 1;
            } else {
                matched_digits = 0;
            }
        }
        if matched_digits == digits.len() {
            break;
        }
        first = (first + 1 + list[first]) % list.len();
        second = (second + 1 + list[second]) % list.len();
    }
    list.len() - digits.len()
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
        assert_eq!("1411383621", day14_part1());
    }

    #[test]
    fn part2_example_9_recipes() {
        assert_eq!(9, recipe_count_until_this_score_appears("51589"));
    }

    #[test]
    fn part2_example_5_recipes() {
        assert_eq!(5, recipe_count_until_this_score_appears("01245"));
    }

    #[test]
    fn part2_example_18_recipes() {
        assert_eq!(18, recipe_count_until_this_score_appears("92510"));
    }

    #[test]
    fn part2_example_2018_recipes() {
        assert_eq!(2018, recipe_count_until_this_score_appears("59414"));
    }

    #[test]
    fn part2() {
        assert_eq!(20_177_474, day14_part2());
    }
}
