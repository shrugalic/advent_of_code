use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests;

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

pub fn allergen_free_ingredient_appearance_count(input: &[String]) -> usize {
    let foods: Vec<Food> = input.iter().map(Food::from).collect();
    let (allergen_free_ingredients, foods) = find_allergen_free_ingredients(foods);
    ingredient_appearance_count(&foods, allergen_free_ingredients)
}

/// - Each allergen is found in exactly one ingredient.
/// - Each ingredient contains zero or one allergen.
/// - Allergens aren't always marked; when they're listed, the ingredient that contains
///   each listed allergen will be somewhere in the corresponding ingredients list.
///   However, even if an allergen isn't listed, the ingredient that contains that
///   allergen could still be present: maybe they forgot to label it,
///   or maybe it was labeled in a language you don't know.
fn find_allergen_free_ingredients(mut foods: Vec<Food>) -> (HashSet<String>, Vec<Food>) {
    let allergens: HashSet<&String> = foods.iter().flat_map(|f| f.allergens.iter()).collect();
    let allergen_count = allergens.len();
    let ingredients: HashSet<&String> = foods.iter().flat_map(|f| f.ingredients.iter()).collect();
    println!(
        "{} allergens contained in {} ingredients of {} foods:\n{:?}",
        allergens.len(),
        ingredients.len(),
        foods.len(),
        allergens,
    );

    let mut ingredient_by_allergen: HashMap<String, String> = HashMap::new();
    let mut ingredients_by_allergen: HashMap<String, HashSet<String>> = HashMap::new();

    while ingredient_by_allergen.len() != allergen_count {
        for _ in 0..foods.len() {
            // Remove food so the borrow checker won't complain about borrowing other food mutably
            let mut food = foods.remove(0);
            for mut other in foods.iter_mut() {
                // Remove resolved ingredients from foods
                ingredient_by_allergen.values().for_each(|ingredient| {
                    food.ingredients.remove(ingredient);
                    other.ingredients.remove(ingredient);
                });
                resolve_shared_ingredients_and_allergens(
                    &mut food,
                    &mut other,
                    &mut ingredient_by_allergen,
                    &mut ingredients_by_allergen,
                );
                resolve_single_ingredients(&mut ingredient_by_allergen, &mut food);
                resolve_single_ingredients(&mut ingredient_by_allergen, &mut other);
            }
            foods.push(food);
        }
    }
    let mut allergen_free_ingredients: HashSet<String> = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .cloned()
        .collect();
    ingredient_by_allergen.values().for_each(|i| {
        allergen_free_ingredients.remove(i);
    });
    (allergen_free_ingredients, foods)
}

fn intersect(set1: &HashSet<String>, set2: &HashSet<String>) -> HashSet<String> {
    set1.intersection(&set2).cloned().collect()
}

fn resolve_shared_ingredients_and_allergens(
    food1: &mut Food,
    food2: &mut Food,
    ingredient_by_allergen: &mut HashMap<String, String>,
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
            ingredient_by_allergen.insert(the_allergen, the_ingredient);
        }
    }

    if shared_ingredients.len() == 1 && (food1.allergens.len() == 1 || food2.allergens.len() == 1) {
        resolve_single_shared_ingredient(food1, food2, ingredient_by_allergen, shared_ingredients);
    }
}

fn narrow_down_possible_ingredients(
    ingredients_by_allergen: &mut HashMap<String, HashSet<String>>,
    shared_allergens: HashSet<String>,
    shared_ingredients: &HashSet<String>,
) -> Option<(String, String)> {
    let the_allergen = shared_allergens.iter().next().unwrap().clone();
    if let Some(ingredients) = ingredients_by_allergen.remove(&the_allergen) {
        let common_ingredients = intersect(&ingredients, &shared_ingredients);
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
    ingredient_by_allergen: &mut HashMap<String, String>,
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
        println!(
            "Resolved by shared ingredient: {} contains {}",
            the_ingredient, the_allergen
        );
        ingredient_by_allergen.insert(the_allergen.to_string(), the_ingredient.to_string());
        // Remove the only allergen, and its ingredient
        matching_food.allergens = HashSet::new();
        matching_food.ingredients.remove(the_ingredient);
    }
}

fn resolve_single_ingredients(
    ingredient_by_allergen: &mut HashMap<String, String>,
    food: &mut Food,
) {
    if food.ingredients.len() == 1 && food.allergens.len() == 1 {
        let ingredient = food.ingredients.iter().next().unwrap();
        let allergen = food.allergens.iter().next().unwrap();
        println!(
            "Resolved by single ingredient: {} contains {}",
            ingredient, allergen
        );
        ingredient_by_allergen.insert(allergen.to_string(), ingredient.to_string());
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
