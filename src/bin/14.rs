use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let mut file = File::open("input/14/input").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| {
        let line = line.unwrap();
        line
    });

    let recipes = lines_to_recipes(lines);
    println!("{:?}", p2(&recipes, 1_000_000_000_000));
    println!("{}", 1_000_000_000_000i64);
    //println!("{:?}", p2(&recipes, 100_000_000));
}

fn p2(recipes: &[Recipe], ore_start_amount: i64) -> i64 {
    let recipes: RecipeBook = recipes.iter().map(|r| (r.get_output(), r.clone())).collect();
    let mut leftovers = HashMap::new();
    get_ore_needed(
        "FUEL".to_string(),
        3209254,
        &mut leftovers,
        &recipes)

    /*

    let start = 3206000;
    let mut prev_ore_needed = 0;
    let range = 3300000 - start;
    for i in 0..range {
        let ore_needed = get_ore_needed(
            "FUEL".to_string(),
            start + i,
            &mut leftovers,
            &recipes);

        if ore_needed > ore_start_amount {
            return start + i - 1;
        }

        prev_ore_needed = ore_needed;
    }
    0
    */
}

fn p1(recipes: &[Recipe]) -> i64 {
    let recipes: RecipeBook = recipes.iter().map(|r| (r.get_output(), r.clone())).collect();
    let fuel_recipe = recipes.get(&"FUEL".to_string()).unwrap();

    let mut leftovers = HashMap::new();
    fuel_recipe.get_ingredients()
        .iter()
        .map(|x| {
            get_ore_needed(
                x.0.to_string(),
                x.1,
                &mut leftovers,
                &recipes,
                )
        }).sum()
}

fn get_ore_needed(
    ingredient: Ingredient,
    amount_needed: Amount,
    leftovers: &mut HashMap<Ingredient, Amount>,
    recipe_book: &RecipeBook,
) -> i64 {

    if &ingredient == "ORE" {
        return amount_needed
    }

    let mut amount_needed = amount_needed;
    if let Some(amount_leftover) = leftovers.get(&ingredient) {
        if *amount_leftover >= amount_needed {
            let leftover = amount_leftover - amount_needed;
            leftovers.insert(ingredient.clone(), leftover);
            return 0;
        } else {
            amount_needed -= amount_leftover;
            leftovers.insert(ingredient.clone(), 0);
        }
    }

    let recipe = recipe_book.get(&ingredient).unwrap();

    let amount_created = recipe.get_amount_created();
    let batches = if amount_created >= amount_needed {
        1
    } else {
        ((amount_needed as f64)/(amount_created as f64)).ceil() as i64
    };

    let leftover = (batches * amount_created) - amount_needed;
    if let Some(already_available) = leftovers.get(&ingredient) {
        leftovers.insert(ingredient.clone(), leftover + already_available);
    } else {
        leftovers.insert(ingredient.clone(), leftover);
    }

    recipe.get_ingredients()
        .iter()
        .map(|x| {
            get_ore_needed(
                x.0.to_string(),
                x.1 * batches,
                leftovers,
                recipe_book,
                )
        }).sum()
}

fn lines_to_recipes(lines: impl Iterator<Item = String>) -> Vec<Recipe> {
    lines
        .map(|line| {
            let splits: Vec<&str> = line.split("=>").map(|x| x.trim()).collect();
            let ingredients = splits[0];
            let output = splits[1];

            let ingredients: Vec<(Ingredient, Amount)> = ingredients
                .split(",")
                .map(|x| {
                    let s: Vec<&str> = x.trim().split(" ").map(|x| x.trim()).collect();
                    (s[1].to_string(), s[0].parse::<Amount>().unwrap())
                })
                .collect();

            let output: Vec<_> = output.split(" ").map(|x| x.trim()).collect();
            let output = (output[1].to_string(), output[0].parse::<Amount>().unwrap());

            Recipe::new(&ingredients, output)
        })
        .collect()
}

type Ingredient = String;
type Amount = i64;
type RecipeBook = HashMap<Ingredient, Recipe>;

#[derive(Debug, Clone)]
struct Recipe {
    output: (Ingredient, Amount),
    input: Vec<(Ingredient, Amount)>,
}

impl Recipe {
    pub fn new(input: &[(Ingredient, Amount)], output: (Ingredient, Amount)) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
        }
    }

    pub fn get_output(&self) -> Ingredient {
        self.output.0.clone()
    }

    pub fn get_amount_created(&self) -> i64 {
        self.output.1
    }

    pub fn get_ingredients(&self) -> Vec<(Ingredient, Amount)> {
        self.input.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1_1() {
        let l = "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL";
        let recipes = lines_to_recipes(l.split('\n').map(|s| s.to_string()));
        assert_eq!(p1(&recipes), 31);
    }

    #[test]
    fn test_2_1() {
        let l = "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL";
        let recipes = lines_to_recipes(l.split('\n').map(|s| s.to_string()));
        assert_eq!(p2(&recipes, 1000), 4);
    }
}
