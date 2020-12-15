use std::collections::HashMap;
use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let numbers: Vec<u64> = input_data
        .trim()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let now = Instant::now();
    match part {
        1 => solve_part1(numbers),
        2 => solve_part2(numbers),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn solve_part1(numbers: Vec<u64>) {
    let mut last_index: HashMap<u64, usize> = HashMap::new();
    for i in 0..(numbers.len() - 1) {
        last_index.insert(numbers[i], i);
    }

    let mut last_number = numbers[(numbers.len() - 1)];
    for i in (numbers.len() - 1)..2019 {
        let old_number = last_number;
        if let Some(index) = last_index.get(&last_number) {
            last_number = (i - *index) as u64;
        } else {
            last_number = 0;
        }
        last_index.insert(old_number, i);
    }
    println!("{}", last_number);
}

fn solve_part2(numbers: Vec<u64>) {
    let mut last_index: HashMap<u64, usize> = HashMap::new();
    for i in 0..(numbers.len() - 1) {
        last_index.insert(numbers[i], i);
    }

    let mut last_number = numbers[(numbers.len() - 1)];
    for i in (numbers.len() - 1)..29999999 {
        let old_number = last_number;
        if let Some(index) = last_index.get(&last_number) {
            last_number = (i - *index) as u64;
        } else {
            last_number = 0;
        }
        last_index.insert(old_number, i);
    }
    println!("{}", last_number);
}
