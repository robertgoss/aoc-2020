use std::collections::HashSet;
use std::collections::HashMap;

struct Food {
    ingredients : HashSet<String>,
    allergens : HashSet<String>
}

pub struct Menu {
    items : Vec<Food>
}

impl Food {
    pub fn from_string(string : &str) -> Option<Food> {
        string.split_once(" (contains ").and_then(
            |(ingredients_str, rest)| rest.strip_suffix(")").map(
                |allergens_str| Food {
                    ingredients : ingredients_str.split(" ").map(|s| s.to_string()).collect(),
                    allergens : allergens_str.split(", ").map(|s| s.to_string()).collect()
                }
            )
        )
    }

    pub fn count_ingredient(self : &Self, ingredient : &str) -> usize {
        self.ingredients.iter().filter(
            |food_ingredient| ingredient == *food_ingredient 
        ).count()
    }
}

impl Menu {
    pub fn from_lines<I>(lines : I) -> Menu
      where I : Iterator<Item = String>
    {
        Menu {
            items : lines.filter_map(
                |line| Food::from_string(&line)
            ).collect()
        }
    }

    fn allergens(self : &Self) -> HashSet<String> {
        let mut allergens :  HashSet<String> = HashSet::new();
        for item in self.items.iter() {
            allergens.extend(
                item.allergens.iter().cloned()
            );
        }
        allergens
    }

    fn ingredients(self : &Self) -> HashSet<String> {
        let mut ingredients :  HashSet<String> = HashSet::new();
        for item in self.items.iter() {
            ingredients.extend(
                item.ingredients.iter().cloned()
            );
        }
        ingredients
    }

    pub fn count_ingredient(self : &Self, ingredient : &str) -> usize {
        self.items.iter().map(
            |food| food.count_ingredient(ingredient)
        ).sum()
    }

    pub fn count_ingredients_no_allergens(self : &Self) -> usize {
        self.ingredients_no_allergens().iter().map(
            |ingredient| self.count_ingredient(ingredient)
        ).sum()
    }

    fn possible_ingredients_for_allegen(self : &Self, allergen : &String) -> HashSet<String> {
        let mut ingredients = self.ingredients();
        for item in self.items.iter() {
            if item.allergens.contains(allergen) {
                ingredients = ingredients.intersection(
                    &item.ingredients
                ).cloned().collect();
            }
        }
        ingredients
    }

    fn possible_ingredients(self : &Self) -> HashMap<String, HashSet<String>> {
        let mut possible : HashMap<String, HashSet<String>> = HashMap::new();
        for allergen in self.allergens() {
            possible.insert(
                allergen.clone(),
                self.possible_ingredients_for_allegen(&allergen)
            );
        }
        possible
    }

    pub fn ingredients_no_allergens(self: &Self) -> HashSet<String> {
        let possible_ingredients = self.possible_ingredients();
        let ingredients : HashSet<String> = 
            self.ingredients().into_iter().filter(
                |ingredient| possible_ingredients.values().all(
                    |set| !set.contains(ingredient)
                )
            ).collect();
        ingredients
    }

    fn ingredients_allergens(self : &Self) -> HashMap<String, String> {
        let mut possible_ingredients = self.possible_ingredients();
        let mut ingredient_map : HashMap<String, String> = HashMap::new();
        while !possible_ingredients.is_empty() {
            let (allergen, ingredient) : (String, String) = possible_ingredients.iter().filter(
                |(_, ingredients)| ingredients.len() == 1
            ).map(
                |(allergen, ingredients)| (allergen.to_string(), ingredients.iter().next().unwrap().to_string())
            ).next().unwrap();
            possible_ingredients.remove(&allergen);
            for ingredients in possible_ingredients.values_mut() {
                ingredients.remove(&ingredient);
            }
            ingredient_map.insert(allergen, ingredient);
        }
        ingredient_map
    }

    pub fn ordered_ingredients_allergens(self : &Self) -> Vec<String> {
        let ingredient_map = self.ingredients_allergens();
        let mut allergens : Vec<String> 
          = ingredient_map.keys().map(|s| s.to_string()).collect();
        allergens.sort();
        allergens.iter().map(
            |allergen| ingredient_map[allergen].to_string()
        ).collect()
    }
}