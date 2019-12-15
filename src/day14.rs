use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Recipe { inputs: Vec<(String, u64)>, quantity: u64, substance: String }

fn parse_quantity_substance(quantity_substance_string : &str) -> (String, u64) {
    let mut iter = quantity_substance_string.split(' ');
    let quantity = iter.next().unwrap().parse::<u64>().unwrap();
    let substance = iter.next().unwrap().to_string();
    return (substance, quantity)
}

fn parse_recipe(recipe_string : &str) -> Recipe {
    let mut iter_halves = recipe_string.split(" => ");
    let inputs : Vec<(String, u64)> = iter_halves.next().unwrap().split(", ").map(|s| parse_quantity_substance(s)).collect();
    let (substance, quantity) = parse_quantity_substance(iter_halves.next().unwrap());
    Recipe { inputs, quantity, substance}
}

fn ore_count(recipes: &HashMap<String, Recipe>, mut stocks: &mut HashMap<String, u64>, goal_quantity: u64, goal_substance: &str) -> u64 {
    if goal_substance == "ORE" {
        return goal_quantity;
    }
    let already_made = *(stocks.get(&goal_substance.to_string()).unwrap_or(&0));
    if (already_made >= goal_quantity) {
        let new_stocks = already_made - goal_quantity;
        if new_stocks > 0 {
            stocks.insert(goal_substance.to_string(), new_stocks);
        } else {
            stocks.remove(&goal_substance.to_string());
        }
        return 0;
    } else {
        stocks.remove(&goal_substance.to_string());
        let goal_quantity = goal_quantity - already_made;

        let recipe = recipes.get(goal_substance).unwrap();
        let multiplier = (goal_quantity + recipe.quantity - 1) / recipe.quantity;
        let excess = recipe.quantity * multiplier - goal_quantity;

        let ore_count = recipe.inputs.iter()
            .map(|(input_substance, input_quantity)| ore_count(recipes, &mut stocks, multiplier * input_quantity, input_substance))
            .sum();
        if (excess != 0) {
            stocks.insert(goal_substance.to_string(), excess);
        }
        return ore_count;
    }
}

fn fuel_ore_count(recipes: &HashMap<String, Recipe>, goal_quantity: u64) -> u64 {
    let mut stocks : HashMap<String, u64> = HashMap::new();
    return ore_count(recipes, &mut stocks, goal_quantity, "FUEL");
}

fn main() -> io::Result<()> {
    let lines_result : io::Result<Vec<String>> = io::stdin().lock().lines().collect();
    let lines = lines_result?;
    let parsed_recipes = lines.iter().map(|line| parse_recipe(&line));
    let recipes : HashMap<String, Recipe> = parsed_recipes.map(|recipe| (recipe.substance.clone(), recipe.clone())).collect();
    println!("{:?}", fuel_ore_count(&recipes, 1));


    // thought about more analytical ways of doing this for a while, but
    // a binary search will do nicely
    let ore : u64 = 1000000000000;
    let mut min : u64 = 0;
    let mut max : u64 = 1000000000;
    let mut mid = max / 2;
    loop {
        println!("{:?} {:?} {:?}", min, mid, max);
        if fuel_ore_count(&recipes, mid) > ore {
            max = mid;
        } else if fuel_ore_count(&recipes, mid + 1) < ore {
            min = mid;
        } else {
            println!("GOTCHA, {:?}", mid);
            break;
        }
        mid = min + (max - min) / 2;
    }
    Ok(())
}
