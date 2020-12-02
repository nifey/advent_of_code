use rayon::prelude::*;
use regex::Regex;
use std::fs;

pub fn solve_part1(filename: String) {
    // Finding number of valid passwords
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let lines: Vec<&str> = input_data.lines().collect();
    let num_valid: u64 = lines
        .par_iter()
        .map(|x| {
            for data in re.captures_iter(x) {
                let min = data[1].parse::<u64>().unwrap();
                let max = data[2].parse::<u64>().unwrap();
                let character: char = data[3].to_string().chars().next().unwrap();
                let text = &data[4];
                let mut count = 0;
                for letter in text.chars() {
                    if letter == character {
                        count += 1;
                    }
                }
                if count <= max && count >= min {
                    return 1;
                }
            }
            0
        })
        .sum();
    println!("{}", num_valid);
}

pub fn solve_part2(filename: String) {
    // Finding number of valid passwords
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let lines: Vec<&str> = input_data.lines().collect();
    let num_valid: u64 = lines
        .par_iter()
        .map(|x| {
            for data in re.captures_iter(x) {
                let pos1 = data[1].parse::<u64>().unwrap() - 1;
                let pos2 = data[2].parse::<u64>().unwrap() - 1;
                let character: char = data[3].to_string().chars().next().unwrap();
                let text = &data[4];
                let characters: Vec<char> = text.chars().collect();
                if characters[pos1 as usize] == character {
                    if characters[pos2 as usize] == character {
                        return 0;
                    } else {
                        return 1;
                    }
                } else if characters[pos2 as usize] == character {
                    return 1;
                } else {
                    return 0;
                }
            }
            0
        })
        .sum();
    println!("{}", num_valid);
}
