use std::collections::HashSet;
use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines = input_data.lines().collect::<Vec<&str>>();
    let mut ingredients: HashSet<String> = HashSet::new();
    let mut allergens: HashSet<String> = HashSet::new();
    let mut mappings: Vec<(HashSet<usize>, HashSet<usize>)> = Vec::new();

    for line in lines.clone() {
        let mut split_data = line.split("(contains ");
        let ingredient_data = split_data.next().unwrap();
        let allergen_data = split_data.next().unwrap().split(")").next().unwrap();
        for ingredient in ingredient_data.trim().split(" ") {
            ingredients.insert(ingredient.to_string());
        }
        for allergen in allergen_data.trim_start().split(",") {
            allergens.insert(allergen.to_string());
        }
    }

    for line in lines {
        let mut split_data = line.split("(contains ");
        let ingredient_data = split_data.next().unwrap();
        let allergen_data = split_data.next().unwrap().split(")").next().unwrap();
        let mut ingredient_set: HashSet<usize> = HashSet::new();
        let mut allergen_set: HashSet<usize> = HashSet::new();
        for ingredient in ingredient_data.trim().split(" ") {
            ingredient_set.insert(
                ingredients
                    .iter()
                    .position(|x| *x == ingredient.to_string())
                    .unwrap(),
            );
        }
        for allergen in allergen_data.trim_start().split(",") {
            allergen_set.insert(
                allergens
                    .iter()
                    .position(|x| *x == allergen.to_string())
                    .unwrap(),
            );
        }
        mappings.push((ingredient_set, allergen_set));
    }

    let now = Instant::now();
    match part {
        1 => solve_part1(ingredients.len(), allergens.len(), mappings),
        2 => solve_part2(),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn solve_part1(
    ingredients_count: usize,
    allergens_count: usize,
    mappings: Vec<(HashSet<usize>, HashSet<usize>)>,
) {
    // Find allergen free ingredients
}

fn solve_part2() {}
