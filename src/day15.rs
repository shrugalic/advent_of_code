use line_reader::read_file_to_lines;
use std::ops::AddAssign;

pub(crate) fn day15_part1() -> isize {
    let input = read_file_to_lines("input/day15.txt");
    find_high_score_ignore_calories(input)
}

pub(crate) fn day15_part2() -> isize {
    let input = read_file_to_lines("input/day15.txt");
    find_high_score_fix_calories(input)
}

fn find_high_score_ignore_calories(input: Vec<String>) -> isize {
    find_high_score(input, false)
}

fn find_high_score_fix_calories(input: Vec<String>) -> isize {
    find_high_score(input, true)
}

const TOTAL_AMOUNT: isize = 100;
fn find_high_score(input: Vec<String>, fix_calories: bool) -> isize {
    let ingredients = parse_ingredients(input);
    let mut score = 0;
    for i0 in 0..=TOTAL_AMOUNT {
        if ingredients.len() == 2 {
            let i1 = TOTAL_AMOUNT - i0;
            let amounts = vec![i0, i1];
            score = score.max(calc_score(&ingredients, &amounts, fix_calories));
        } else {
            for i1 in 0..=(TOTAL_AMOUNT - i0) {
                for i2 in 0..=(TOTAL_AMOUNT - i0 - i1) {
                    let i3 = TOTAL_AMOUNT - i0 - i1 - i2;
                    let amounts = vec![i0, i1, i2, i3];
                    score = score.max(calc_score(&ingredients, &amounts, fix_calories));
                }
            }
        }
    }
    score
}

const TARGET_CALORIES: isize = 500;
fn calc_score(ingredients: &[Ingredient], amounts: &[isize], fix_calories: bool) -> isize {
    let mut total = Ingredient::default();
    for i in 0..ingredients.len() {
        total += ingredients[i].scaled_by(amounts[i]);
    }
    if fix_calories && total.calories != TARGET_CALORIES {
        0
    } else {
        total.score()
    }
}

fn parse_ingredients(input: Vec<String>) -> Vec<Ingredient> {
    input.into_iter().map(Ingredient::from).collect()
}

#[derive(Debug)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}
impl From<String> for Ingredient {
    // Example:
    // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    fn from(s: String) -> Self {
        let parts: Vec<_> = s.split(|c| c == ' ' || c == ',').collect();
        Ingredient {
            capacity: parts[2].parse().unwrap(),
            durability: parts[5].parse().unwrap(),
            flavor: parts[8].parse().unwrap(),
            texture: parts[11].parse().unwrap(),
            calories: parts[14].parse().unwrap(),
        }
    }
}
impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
}
impl AddAssign for Ingredient {
    fn add_assign(&mut self, rhs: Self) {
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
    }
}
impl Ingredient {
    fn score(&self) -> isize {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
    fn scaled_by(&self, factor: isize) -> Self {
        Ingredient {
            capacity: self.capacity * factor,
            durability: self.durability * factor,
            flavor: self.flavor * factor,
            texture: self.texture * factor,
            calories: self.calories * factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn part1_example() {
        let input = read_str_to_lines(EXAMPLE);
        assert_eq!(62842880, find_high_score_ignore_calories(input));
    }
    #[test]
    fn part1() {
        assert_eq!(13882464, day15_part1());
    }

    #[test]
    fn part2_example() {
        let input = read_str_to_lines(EXAMPLE);
        assert_eq!(57600000, find_high_score_fix_calories(input));
    }

    #[test]
    fn part2() {
        assert_eq!(11171160, day15_part2());
    }
}
