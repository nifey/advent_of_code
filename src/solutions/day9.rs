use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn solve_part1(filename: String) {
    // Cracking XMAS encryption
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let numbers: Vec<i64> = input_data
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut queue: VecDeque<i64> = VecDeque::new();
    let mut sorted: HashSet<i64> = HashSet::new();

    for i in 0..25 {
        queue.push_back(numbers[i]);
        sorted.insert(numbers[i]);
    }

    for i in 25..numbers.len() {
        let current_number = numbers[i];
        let mut valid = false;
        for val in &sorted {
            if sorted.contains(&(current_number - val)) {
                valid = true;
                break;
            }
        }
        if !valid {
            println!("{}", current_number);
            std::process::exit(0);
        } else {
            let old = queue.pop_front().unwrap();
            sorted.remove(&old);
            queue.push_back(current_number);
            sorted.insert(current_number);
        }
    }
}

pub fn solve_part2(filename: String) {
    // Finding encryption weakness in XMAS encryption
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let numbers: Vec<i64> = input_data
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let mut invalid_number = 0;
    let mut queue: VecDeque<i64> = VecDeque::new();
    let mut sorted: HashSet<i64> = HashSet::new();

    for i in 0..25 {
        queue.push_back(numbers[i]);
        sorted.insert(numbers[i]);
    }

    for i in 25..numbers.len() {
        let current_number = numbers[i];
        let mut valid = false;
        for val in &sorted {
            if sorted.contains(&(current_number - val)) {
                valid = true;
                break;
            }
        }
        if !valid {
            invalid_number = current_number;
            break;
        } else {
            let old = queue.pop_front().unwrap();
            sorted.remove(&old);
            queue.push_back(current_number);
            sorted.insert(current_number);
        }
    }

    let mut start: usize = 0;
    let mut end: usize = 0;

    loop {
        let mut sum = numbers[start];
        let mut found = false;
        end = start;
        while sum < invalid_number && end < numbers.len() - 1 {
            end += 1;
            sum += numbers[end];
            if sum == invalid_number {
                found = true;
                break;
            }
        }
        if found {
            break;
        }
        start += 1;
    }

    let mut min = numbers[start];
    let mut max = numbers[end];
    for i in start..=end {
        if numbers[i] > max {
            max = numbers[i];
        }
        if numbers[i] < min {
            min = numbers[i];
        }
    }

    println!("{}", min + max);
}
