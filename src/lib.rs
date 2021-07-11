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
    let allergen_free_ingredient_count = ingredients.len() - allergens.len();
    println!(
        "{} allergens contained in {} ingredients of {} foods:\n{:?}",
        allergens.len(),
        ingredients.len(),
        foods.len(),
        allergens,
    );
    // (allergen_free_ingredient_count, allergens.len())
    let mut allergen_free_ingredients: HashSet<String> = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .cloned()
        .collect();
    let mut ingredient_by_allergen: HashMap<String, String> = HashMap::new();

    if false {
        let mut sorted: Vec<(usize, usize, usize)> = foods
            .iter()
            .map(|f| {
                (
                    f.ingredients.len() - f.allergens.len(),
                    f.ingredients.len(),
                    f.allergens.len(),
                )
            })
            .collect();
        sorted.sort_by_key(|s| s.0);
        println!("{:?}", sorted);
    }
    // println!("{} foods {:?}", foods_count, foods);
    while allergen_free_ingredients.len() != allergen_free_ingredient_count
        && ingredient_by_allergen.len() != allergen_count
    {
        for _ in 0..foods.len() {
            let mut food = foods.remove(0);
            for mut other in foods.iter_mut() {
                // Remove resolved ingredients from foods
                ingredient_by_allergen.values().for_each(|ingredient| {
                    food.ingredients.remove(ingredient);
                    other.ingredients.remove(ingredient);
                });
                resolve_shared_ingredients(&mut food, &mut other, &mut ingredient_by_allergen);
                resolve_single_ingredients(&mut ingredient_by_allergen, &mut food);
                resolve_single_ingredients(&mut ingredient_by_allergen, &mut other);
            } // inner for
            foods.push(food);
        } // outer for

        if true {
            break;
        }
    }

    if allergen_free_ingredients.len() < allergen_free_ingredient_count {
        ingredient_by_allergen.values().for_each(|i| {
            allergen_free_ingredients.remove(i);
        });
    }
    (allergen_free_ingredients, foods)
}

fn intersect(set1: &HashSet<String>, set2: &HashSet<String>) -> HashSet<String> {
    set1.intersection(&set2).cloned().collect()
}

fn resolve_shared_ingredients(
    food1: &mut Food,
    food2: &mut Food,
    ingredient_by_allergen: &mut HashMap<String, String>,
) {
    let shared_ingredients = intersect(&food1.ingredients, &food2.ingredients);
    if shared_ingredients.is_empty() {
        return;
    }
    let shared_allergens = intersect(&food1.allergens, &food2.allergens);
    if shared_allergens.is_empty() {
        return;
    } else {
        println!(
            "Two foods share {} ingredients and {} allergens: {:?}",
            shared_ingredients.len(),
            shared_allergens.len(),
            shared_allergens
        );
    }

    if shared_ingredients.len() == food1.allergens.len()
        || shared_ingredients.len() == food2.allergens.len()
    {
        let (food, other) = if shared_ingredients.len() == food1.allergens.len() {
            (food1, food2)
        } else {
            (food2, food1)
        };

        if shared_ingredients.len() == 1 {
            let allergen = food.allergens.iter().next().unwrap();
            if other.allergens.contains(allergen) {
                let ingredient = shared_ingredients.iter().next().unwrap();
                println!(
                    "Resolved by shared ingredient: {} contains {}",
                    ingredient, allergen
                );
                ingredient_by_allergen.insert(allergen.to_string(), ingredient.to_string());
                // Remove the only allergen, and its ingredient
                food.allergens = HashSet::new();
                food.ingredients.remove(ingredient);
            }
        } else {
            // TODO
            println!(
                "Food 1: {:?}\nFood 2: {:?}\nShared ingredients {:?}",
                food, other, shared_ingredients
            );
        }
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
