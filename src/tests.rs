use crate::{allergen_free_ingredient_appearance_count, ingredient_appearance_count, Food};
use line_reader::*;
const EXAMPLE_1: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

#[test]
fn food_from_str() {
    let to_strings = |str_vec: Vec<&str>| str_vec.into_iter().map(str::to_string).collect();
    let actual = Food::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)");
    let expected = Food {
        ingredients: to_strings(vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]),
        allergens: to_strings(vec!["dairy", "fish"]),
    };
    assert_eq!(actual, expected);
}

#[test]
fn example_1() {
    assert_eq!(
        allergen_free_ingredient_appearance_count(&read_str_to_lines(EXAMPLE_1)),
        5
    );
}

#[test]
fn example_1_ingredient_appearance_count() {
    let foods: Vec<Food> = read_str_to_lines(EXAMPLE_1)
        .iter()
        .map(Food::from)
        .collect();
    let allergen_free_ingredients = vec![
        "kfcds".to_string(),
        "nhms".to_string(),
        "sbzzf".to_string(),
        "trh".to_string(),
    ];
    assert_eq!(
        ingredient_appearance_count(&foods, allergen_free_ingredients.iter().cloned().collect()),
        5
    );
}

#[test]
fn part1() {
    assert_eq!(
        allergen_free_ingredient_appearance_count(&read_file_to_lines("input.txt")),
        192 // 200 ingredients - 8 allergens (is too low of course)
            // 2804 is to high
    );
}
