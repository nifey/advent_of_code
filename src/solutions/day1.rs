use std::fs;

pub fn solve_part1(filename: String) {
    // Finding two numbers that add up to 2020
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut numbers: Vec<u64> = input_data
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    numbers.sort();
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        if numbers[left] + numbers[right] == 2020 {
            println!("{}", numbers[left] * numbers[right]);
            break;
        } else if numbers[left] < 2020 - numbers[right] {
            while numbers[left] < 2020 - numbers[right] {
                left += 1;
            }
        } else {
            while numbers[left] > 2020 - numbers[right] {
                right -= 1;
            }
        }
    }
}

pub fn solve_part2(filename: String) {
    // Finding three numbers that add up to 2020
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut numbers: Vec<i64> = input_data
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    numbers.sort();
    for fixed in 0..numbers.len() {
        let target = 2020 - numbers[fixed];
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            if numbers[left] + numbers[right] == target {
                println!("{}", numbers[left] * numbers[right] * numbers[fixed]);
                std::process::exit(0);
            } else if numbers[left] < target - numbers[right] {
                while numbers[left] < target - numbers[right] {
                    left += 1;
                }
            } else {
                while numbers[left] > target - numbers[right] {
                    right -= 1;
                }
            }
            if left == fixed {
                left += 1;
            }
            if right == fixed {
                right -= 1;
            }
        }
    }
}
