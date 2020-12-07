use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve_part1(filename: String) {
    // Finding all color bags that can carry a shiny gold bag
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();
    let re = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bags?").unwrap();

    let mut contained_in: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines {
        let mut split = line.split(" contain ");
        let outer_bag = (split.next().unwrap()).split(" bag").next().unwrap();
        let inner_bags = split.next().unwrap();
        for parts in re.captures_iter(inner_bags) {
            let inner_bag = parts[2].to_string();
            if !contained_in.contains_key(&inner_bag) {
                contained_in.insert(inner_bag.clone(), HashSet::new());
            }
            let contained_in_set = contained_in.get_mut(&inner_bag).unwrap();
            contained_in_set.insert(outer_bag.to_string());
        }
    }

    let mut possible_outer_bags: HashSet<String> = HashSet::new();
    let start_bag = "shiny gold".to_string();
    let mut queue: Vec<String> = Vec::new();

    for container_bag in contained_in.get(&start_bag).unwrap() {
        queue.push(container_bag.clone());
        possible_outer_bags.insert(container_bag.to_string());
    }

    while queue.len() > 0 {
        let current_bag = queue.pop().unwrap();
        if current_bag == start_bag {
            continue;
        }
        if let Some(container_bags) = contained_in.get(&current_bag) {
            for container_bag in container_bags {
                if !possible_outer_bags.contains(container_bag) {
                    queue.push(container_bag.clone());
                    possible_outer_bags.insert(container_bag.to_string());
                }
            }
        }
    }

    println!("{}", possible_outer_bags.len());
}

pub fn count_bags(
    contains: &HashMap<String, HashSet<(u64, String)>>,
    mut bag_counts: HashMap<String, u64>,
    color: String,
) -> (u64, HashMap<String, u64>) {
    if bag_counts.contains_key(&color) {
        (*bag_counts.get(&color).unwrap(), bag_counts)
    } else {
        let mut count = 1;
        if let Some(inner_bags) = contains.get(&color) {
            for inner_bag in inner_bags {
                let tuple = count_bags(&contains, bag_counts, inner_bag.1.clone());
                bag_counts = tuple.1;
                count += (inner_bag.0) * tuple.0;
            }
        }
        bag_counts.insert(color, count);
        (count, bag_counts)
    }
}

pub fn solve_part2(filename: String) {
    // Finding count of all bags that need to go in a shiny gold bag
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();
    let re = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bags?").unwrap();

    let mut contains: HashMap<String, HashSet<(u64, String)>> = HashMap::new();
    for line in lines {
        let mut split = line.split(" contain ");
        let outer_bag = (split.next().unwrap()).split(" bag").next().unwrap();
        let inner_bags = split.next().unwrap();
        let mut set = HashSet::new();
        for parts in re.captures_iter(inner_bags) {
            let inner_bag = parts[2].to_string();
            let count: u64 = parts[1].parse::<u64>().unwrap();
            set.insert((count, inner_bag.to_string()));
        }
        contains.insert(outer_bag.to_string(), set);
    }

    let bag_counts: HashMap<String, u64> = HashMap::new();
    let tuple = count_bags(&contains, bag_counts, "shiny gold".to_string());

    // We subtract 1 because it also includes the shiny gold bag
    println!("{:?}", tuple.0 - 1);
}
