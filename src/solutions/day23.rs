use std::collections::HashSet;
use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let numbers = input_data
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_string().parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let now = Instant::now();
    match part {
        1 => solve_part1(numbers),
        2 => solve_part2(numbers),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn solve_part1(mut numbers: Vec<u16>) {
    let mut current_index = 0;
    let mut dst_index = 0;

    for _ in 0..100 {
        let mut search_value = numbers[current_index] - 1;
        if search_value == 0 {
            search_value = numbers.len() as u16;
        }
        let mut found = false;
        while !found {
            found = true;
            let mut found_index = 0;
            for i in 0..numbers.len() {
                if numbers[i] == search_value {
                    found_index = i;
                    break;
                }
            }

            let mut next_index = current_index + 1;
            for _ in 0..3 {
                if next_index >= numbers.len() {
                    next_index -= numbers.len();
                }
                if next_index == found_index {
                    found = false;
                    break;
                }
                next_index += 1;
            }

            if search_value == 1 {
                search_value = numbers.len() as u16;
            } else {
                search_value -= 1;
            }

            dst_index = found_index + 1;
            if dst_index >= numbers.len() {
                dst_index -= numbers.len();
            }
        }

        let mut new_numbers = numbers.clone();

        let mut src_index = (current_index + 1) % numbers.len();
        while (src_index + 3) % numbers.len() != dst_index % numbers.len() {
            new_numbers[src_index] = numbers[(src_index + 3) % numbers.len()];
            src_index = (src_index + 1) % numbers.len();
        }

        dst_index = src_index;
        src_index = (current_index + 1) % numbers.len();
        for _ in 0..3 {
            new_numbers[dst_index] = numbers[src_index];
            dst_index = (dst_index + 1) % numbers.len();
            src_index = (src_index + 1) % numbers.len();
        }

        numbers = new_numbers;
        current_index += 1;
        if current_index >= numbers.len() {
            current_index = 0;
        }
    }

    while numbers[0] != 1 {
        numbers.rotate_left(1);
    }

    for i in 1..numbers.len() {
        print!("{}", numbers[i]);
    }
    println!("");
}

fn solve_part2(mut numbers: Vec<u16>) {
    let mut current_index = 0;
    let mut dst_index = 0;

    for num in numbers.len()..=1000000 {
        numbers.push(num as u16);
    }

    for iteration in 0..10000000 {
        let mut search_value = numbers[current_index] - 1;
        if search_value == 0 {
            search_value = numbers.len() as u16;
        }
        let mut found = false;
        while !found {
            found = true;
            let mut found_index = 0;
            for i in 0..numbers.len() {
                if numbers[i] == search_value {
                    found_index = i;
                    break;
                }
            }

            let mut next_index = current_index + 1;
            for _ in 0..3 {
                if next_index >= numbers.len() {
                    next_index -= numbers.len();
                }
                if next_index == found_index {
                    found = false;
                    break;
                }
                next_index += 1;
            }

            if search_value == 1 {
                search_value = numbers.len() as u16;
            } else {
                search_value -= 1;
            }

            dst_index = found_index + 1;
            if dst_index >= numbers.len() {
                dst_index -= numbers.len();
            }
        }

        let mut new_numbers = numbers.clone();

        let mut src_index = (current_index + 1) % numbers.len();
        while (src_index + 3) % numbers.len() != dst_index % numbers.len() {
            new_numbers[src_index] = numbers[(src_index + 3) % numbers.len()];
            src_index = (src_index + 1) % numbers.len();
        }

        dst_index = src_index;
        src_index = (current_index + 1) % numbers.len();
        for _ in 0..3 {
            new_numbers[dst_index] = numbers[src_index];
            dst_index = (dst_index + 1) % numbers.len();
            src_index = (src_index + 1) % numbers.len();
        }

        numbers = new_numbers;
        current_index += 1;
        if current_index >= numbers.len() {
            current_index = 0;
        }

        if iteration % 10000 == 0 {
            println!("Iter: {}", iteration);
        }
    }

    let one_index = numbers.iter().position(|&x| x == 1).unwrap();
    let answer =
        numbers[(one_index + 1) % numbers.len()] * numbers[(one_index + 1) % numbers.len()];

    println!("{}", answer);
}
