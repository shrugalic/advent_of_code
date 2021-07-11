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
    let foods_count = foods.len();
    // println!("{} foods {:?}", foods_count, foods);
    while allergen_free_ingredients.len() != allergen_free_ingredient_count
        && ingredient_by_allergen.len() != allergen_count
    {
        for _i in 0..foods_count {
            let mut food = foods.remove(0);
            for _i in 0..foods_count - 1 {
                let mut other = foods.remove(0);

                let mut resolved_ingredients: HashSet<String> =
                    ingredient_by_allergen.values().cloned().collect();
                resolved_ingredients.iter().for_each(|ingredient| {
                    food.ingredients.remove(ingredient);
                    other.ingredients.remove(ingredient);
                });

                // let resolved_allergens: HashSet<&String> =
                //     ingredient_by_allergen.keys().cloned().collect();
                let common_ingredients: HashSet<String> = food
                    .ingredients
                    .intersection(&other.ingredients)
                    .cloned()
                    .collect();
                // remove resolved ingredients
                let common_ingredients: HashSet<String> = common_ingredients
                    .difference(&resolved_ingredients)
                    .cloned()
                    .collect();

                if common_ingredients.len() == food.allergens.len() {
                    if common_ingredients.len() == 1 {
                        let ingredient = common_ingredients.iter().next().unwrap();
                        let allergen = food.allergens.iter().next().unwrap();
                        if other.allergens.contains(allergen) {
                            println!("{} contains {}", ingredient, allergen);
                            ingredient_by_allergen
                                .insert(allergen.to_string(), ingredient.to_string());
                            // Remove the only allergen, and its ingredient
                            food.allergens = HashSet::new();
                            food.ingredients.remove(ingredient);
                            resolved_ingredients.insert(ingredient.to_string());
                            // println!(
                            //     "Remaining ingredients without allergens {:?}",
                            //     food.ingredients
                            // );
                        }
                    } else {
                        // TODO
                    }
                } else if common_ingredients.len() == other.allergens.len() {
                    if common_ingredients.len() == 1 {
                        let ingredient = common_ingredients.iter().next().unwrap();
                        let allergen = other.allergens.iter().next().unwrap();
                        if food.allergens.contains(allergen) {
                            println!("{} contains {}", ingredient, allergen);
                            ingredient_by_allergen
                                .insert(allergen.to_string(), ingredient.to_string());
                            // Remove the only allergen, and its ingredient
                            other.allergens = HashSet::new();
                            other.ingredients.remove(ingredient);
                            resolved_ingredients.insert(ingredient.to_string());
                            // println!(
                            //     "Remaining ingredients without allergens {:?}",
                            //     other.ingredients
                            // );
                        }
                    } else {
                        // TODO
                    }
                }
                // resolve single-item food
                if food.ingredients.len() == 1 && food.allergens.len() == 1 {
                    println!("food 1! {:?}", food);
                    let ingredient = food.ingredients.iter().next().unwrap();
                    let allergen = food.allergens.iter().next().unwrap();
                    println!("{} contains {}", ingredient, allergen);
                    ingredient_by_allergen.insert(allergen.to_string(), ingredient.to_string());
                    resolved_ingredients.insert(ingredient.to_string());
                    food.allergens = HashSet::new();
                    food.ingredients = HashSet::new();
                }
                // resolve single-item other
                if other.ingredients.len() == 1 && other.allergens.len() == 1 {
                    println!("food 1! {:?}", other);
                    let ingredient = other.ingredients.iter().next().unwrap();
                    let allergen = other.allergens.iter().next().unwrap();
                    println!("{} contains {}", ingredient, allergen);
                    ingredient_by_allergen.insert(allergen.to_string(), ingredient.to_string());
                    resolved_ingredients.insert(ingredient.to_string());
                    other.allergens = HashSet::new();
                    other.ingredients = HashSet::new();
                }
                foods.push(other);
            } // inner for
            foods.push(food);
        } // outer for

        // if true {
        //     break;
        // }
    }

    if allergen_free_ingredients.len() < allergen_free_ingredient_count {
        ingredient_by_allergen.values().for_each(|i| {
            allergen_free_ingredients.remove(i);
        });
    }
    (allergen_free_ingredients, foods)
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
