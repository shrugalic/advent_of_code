use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl<T: AsRef<str>> From<T> for Food {
    fn from(input: T) -> Self {
        let (ingredients, allergens) = input.as_ref().split_once(" (contains ").unwrap();
        let ingredients = ingredients.split(' ').map(str::to_string).collect();
        let allergens = allergens
            .split(", ")
            .map(|s| s.trim_end_matches(')'))
            .map(str::to_string)
            .collect();
        Food {
            ingredients,
            allergens,
        }
    }
}

pub(crate) fn allergen_free_ingredient_appearance_count(input: &[String]) -> usize {
    let mut foods: Vec<Food> = input.iter().map(Food::from).collect();
    let allergen_ingredient_pairs = find_allergen_ingredient_pairs(&mut foods);
    let allergen_free_ingredients = allergen_free_ingredients(allergen_ingredient_pairs, &foods);
    ingredient_appearance_count(&foods, allergen_free_ingredients)
}

pub(crate) fn canonical_dangerous_ingredient_list(input: &[String]) -> String {
    let mut foods: Vec<Food> = input.iter().map(Food::from).collect();
    let allergen_ingredient_pairs = find_allergen_ingredient_pairs(&mut foods);
    let mut sorted_allergen_ingredient_pairs: Vec<&(String, String)> =
        allergen_ingredient_pairs.iter().collect();
    sorted_allergen_ingredient_pairs.sort_by_key(|(allergen, _)| allergen);
    sorted_allergen_ingredient_pairs
        .iter()
        .map(|(_, ingredient)| ingredient.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn allergen_free_ingredients(
    allergen_ingredient_pairs: HashSet<(String, String)>,
    foods: &[Food],
) -> HashSet<String> {
    let mut allergen_free_ingredients: HashSet<String> = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .cloned()
        .collect();
    allergen_ingredient_pairs
        .iter()
        .for_each(|(_, ingredient)| {
            allergen_free_ingredients.remove(ingredient);
        });
    allergen_free_ingredients
}

/// - Each allergen is found in exactly one ingredient.
/// - Each ingredient contains zero or one allergen.
/// - Allergens aren't always marked; when they're listed, the ingredient that contains
///   each listed allergen will be somewhere in the corresponding ingredients list.
///   However, even if an allergen isn't listed, the ingredient that contains that
///   allergen could still be present: maybe they forgot to label it,
///   or maybe it was labeled in a language you don't know.
fn find_allergen_ingredient_pairs(foods: &mut Vec<Food>) -> HashSet<(String, String)> {
    let allergens: HashSet<&String> = foods.iter().flat_map(|f| f.allergens.iter()).collect();
    let allergen_count = allergens.len();
    // let ingredients: HashSet<&String> = foods.iter().flat_map(|f| f.ingredients.iter()).collect();
    // println!(
    //     "{} allergens contained in {} ingredients of {} foods:\n{:?}",
    //     allergens.len(),
    //     ingredients.len(),
    //     foods.len(),
    //     allergens,
    // );

    let mut allergen_ingredient_pairs: HashSet<(String, String)> = HashSet::new();
    let mut ingredients_by_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    while allergen_ingredient_pairs.len() != allergen_count {
        for _ in 0..foods.len() {
            // Remove food so the borrow checker won't complain about borrowing other food mutably
            let mut food = foods.remove(0);
            for mut other in foods.iter_mut() {
                // Remove resolved ingredients from foods
                allergen_ingredient_pairs
                    .iter()
                    .for_each(|(_, ingredient)| {
                        food.ingredients.remove(ingredient);
                        other.ingredients.remove(ingredient);
                    });
                resolve_shared_ingredients_and_allergens(
                    &mut food,
                    &mut other,
                    &mut allergen_ingredient_pairs,
                    &mut ingredients_by_allergen,
                );
                resolve_single_ingredients(&mut allergen_ingredient_pairs, &mut food);
                resolve_single_ingredients(&mut allergen_ingredient_pairs, &mut other);
            }
            foods.push(food);
        }
    }
    allergen_ingredient_pairs
}

fn intersect(set1: &HashSet<String>, set2: &HashSet<String>) -> HashSet<String> {
    set1.intersection(set2).cloned().collect()
}

fn resolve_shared_ingredients_and_allergens(
    food1: &mut Food,
    food2: &mut Food,
    allergen_ingredient_pairs: &mut HashSet<(String, String)>,
    ingredients_by_allergen: &mut HashMap<String, HashSet<String>>,
) {
    let shared_allergens = intersect(&food1.allergens, &food2.allergens);
    if shared_allergens.is_empty() {
        return;
    }
    let shared_ingredients = intersect(&food1.ingredients, &food2.ingredients);
    if shared_ingredients.is_empty() {
        return;
    }
    if shared_allergens.len() == 1 {
        if let Some((the_allergen, the_ingredient)) = narrow_down_possible_ingredients(
            ingredients_by_allergen,
            shared_allergens,
            &shared_ingredients,
        ) {
            allergen_ingredient_pairs.insert((the_allergen, the_ingredient));
        }
    }

    if shared_ingredients.len() == 1 && (food1.allergens.len() == 1 || food2.allergens.len() == 1) {
        resolve_single_shared_ingredient(
            food1,
            food2,
            allergen_ingredient_pairs,
            shared_ingredients,
        );
    }
}

fn narrow_down_possible_ingredients(
    ingredients_by_allergen: &mut HashMap<String, HashSet<String>>,
    shared_allergens: HashSet<String>,
    shared_ingredients: &HashSet<String>,
) -> Option<(String, String)> {
    let the_allergen = shared_allergens.iter().next().unwrap().clone();
    if let Some(ingredients) = ingredients_by_allergen.remove(&the_allergen) {
        let common_ingredients = intersect(&ingredients, shared_ingredients);
        if common_ingredients.len() == 1 {
            let the_ingredient = common_ingredients.iter().next().unwrap().to_string();
            ingredients_by_allergen.insert(the_allergen.clone(), common_ingredients);
            return Some((the_allergen, the_ingredient));
        }
        ingredients_by_allergen.insert(the_allergen, common_ingredients);
    } else {
        ingredients_by_allergen.insert(the_allergen, shared_ingredients.clone());
    }
    None
}

fn resolve_single_shared_ingredient(
    food1: &mut Food,
    food2: &mut Food,
    allergen_ingredient_pairs: &mut HashSet<(String, String)>,
    shared_ingredients: HashSet<String>,
) {
    // Make `food` the one with the single matching allergen
    let (matching_food, other_food) = match (food1.allergens.len(), food2.allergens.len()) {
        (1, _) => (food1, food2),
        (_, 1) => (food2, food1),
        (_, _) => unreachable!(),
    };

    let the_allergen = matching_food.allergens.iter().next().unwrap();
    if other_food.allergens.contains(the_allergen) {
        let the_ingredient = shared_ingredients.iter().next().unwrap();
        // println!(
        //     "Resolved by shared ingredient: {} contains {}",
        //     the_ingredient, the_allergen
        // );
        allergen_ingredient_pairs.insert((the_allergen.to_string(), the_ingredient.to_string()));
        // Remove the only allergen, and its ingredient
        matching_food.allergens = HashSet::new();
        matching_food.ingredients.remove(the_ingredient);
    }
}

fn resolve_single_ingredients(
    allergen_ingredient_pairs: &mut HashSet<(String, String)>,
    food: &mut Food,
) {
    if food.ingredients.len() == 1 && food.allergens.len() == 1 {
        let ingredient = food.ingredients.iter().next().unwrap();
        let allergen = food.allergens.iter().next().unwrap();
        // println!(
        //     "Resolved by single ingredient: {} contains {}",
        //     ingredient, allergen
        // );
        allergen_ingredient_pairs.insert((allergen.to_string(), ingredient.to_string()));
        food.allergens = HashSet::new();
        food.ingredients = HashSet::new();
    }
}

fn ingredient_appearance_count(foods: &[Food], ingredients: HashSet<String>) -> usize {
    foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|i| ingredients.contains(*i))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_reader::*;
    const EXAMPLE_1: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn food_from_str() {
        let to_strings = |str_vec: Vec<&str>| -> HashSet<String> {
            str_vec.into_iter().map(str::to_string).collect()
        };
        let actual = Food::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)");
        let expected = Food {
            ingredients: to_strings(vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]),
            allergens: to_strings(vec!["dairy", "fish"]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_example_1() {
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
            ingredient_appearance_count(
                &foods,
                allergen_free_ingredients.iter().cloned().collect()
            ),
            5
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            allergen_free_ingredient_appearance_count(&read_file_to_lines("input/day21.txt")),
            2517
        );
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(
            canonical_dangerous_ingredient_list(&read_str_to_lines(EXAMPLE_1)),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            canonical_dangerous_ingredient_list(&read_file_to_lines("input/day21.txt")),
            "rhvbn,mmcpg,kjf,fvk,lbmt,jgtb,hcbdb,zrb".to_string()
        );
    }
}
