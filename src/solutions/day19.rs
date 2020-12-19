use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Rule {
    Zero(char),
    One(Vec<usize>),
    Two(Vec<usize>, Vec<usize>),
}

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines = input_data.lines().collect::<Vec<&str>>();
    let mut i = 0;
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    while lines[i].len() != 0 {
        let data = lines[i].split(": ").collect::<Vec<&str>>();
        let number = data[0].parse::<usize>().unwrap();
        let parts: Vec<String> = data[1].split(" | ").map(|x| x.to_string()).collect();

        if parts.len() == 1 && parts[0].starts_with("\"") {
            rules.insert(number, Rule::Zero(parts[0].chars().skip(1).next().unwrap()));
        } else if parts.len() == 1 {
            let mut part0 = Vec::new();
            for val in parts[0].split(" ") {
                part0.push(val.parse::<usize>().unwrap());
            }
            rules.insert(number, Rule::One(part0));
        } else if parts.len() == 2 {
            let mut part0 = Vec::new();
            let mut part1 = Vec::new();
            for val in parts[0].split(" ") {
                part0.push(val.parse::<usize>().unwrap());
            }
            for val in parts[1].split(" ") {
                part1.push(val.parse::<usize>().unwrap());
            }
            rules.insert(number, Rule::Two(part0, part1));
        }
        i += 1;
    }

    let mut strings: Vec<String> = Vec::new();
    for index in i + 1..lines.len() {
        strings.push(lines[index].to_string());
    }

    let now = Instant::now();
    match part {
        1 => solve_part1(rules, strings),
        2 => solve_part2(rules, strings),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn match_rule(
    rules: &HashMap<usize, Rule>,
    rule: usize,
    string: &Vec<char>,
    start: usize,
) -> (bool, usize) {
    // Tries to match the rule to the string
    // Returns the number of characters matched
    if start >= string.len() {
        return (false, 0);
    }
    let mut index = start;
    let current_rule: &Rule = rules.get(&rule).unwrap();
    match current_rule {
        Rule::Zero(c) => {
            if string[index] == *c {
                (true, 1)
            } else {
                (false, 0)
            }
        }
        Rule::One(rule1) => {
            let mut fail: bool = false;
            for i in 0..rule1.len() {
                let result = match_rule(&rules, rule1[i], &string, index);
                if result.0 {
                    index += result.1;
                } else {
                    fail = true;
                    break;
                }
            }
            if fail {
                (false, 0)
            } else {
                (true, index - start)
            }
        }
        Rule::Two(rule1, rule2) => {
            // Try rule 1
            let mut fail: bool = false;
            for i in 0..rule1.len() {
                let result = match_rule(&rules, rule1[i], &string, index);
                if result.0 {
                    index += result.1;
                } else {
                    fail = true;
                    break;
                }
            }
            if !fail {
                return (true, index - start);
            }
            // Try rule 2
            index = start;
            fail = false;
            for i in 0..rule2.len() {
                let result = match_rule(&rules, rule2[i], &string, index);
                if result.0 {
                    index += result.1;
                } else {
                    fail = true;
                    break;
                }
            }
            if fail {
                (false, 0)
            } else {
                (true, index - start)
            }
        }
    }
}

fn solve_part1(rules: HashMap<usize, Rule>, strings: Vec<String>) {
    // Check strings for rules
    println!(
        "{:?}",
        strings
            .iter()
            .map(|x| {
                let result = match_rule(&rules, 0, &x.chars().collect(), 0);
                match result.0 {
                    true => {
                        if result.1 == x.len() {
                            1
                        } else {
                            0
                        }
                    }
                    false => 0,
                }
            })
            .sum::<u64>()
    );
}

fn solve_part2(mut rules: HashMap<usize, Rule>, strings: Vec<String>) {
    // Check strings for rules with recursive rules
    let mut count = 0;
    rules.remove(&8);
    rules.remove(&11);
    rules.remove(&0);
    for string in strings {
        // 0  => 8 11
        // 8  => 42 | 42 8
        // 11 => 42 31 | 42 11 31
        let mut num_42 = 0;
        let mut num_31 = 0;
        let mut index = 0;
        loop {
            let result = match_rule(&rules, 42, &string.chars().collect(), index);
            if result.0 {
                num_42 += 1;
                index += result.1;
            } else {
                break;
            }
        }
        loop {
            let result = match_rule(&rules, 31, &string.chars().collect(), index);
            if result.0 {
                num_31 += 1;
                index += result.1;
            } else {
                break;
            }
        }

        if num_31 > 0 && num_42 > num_31 && index == string.len() {
            count += 1;
        }
    }
    println!("{}", count);
}
