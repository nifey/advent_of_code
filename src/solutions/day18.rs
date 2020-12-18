use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines = input_data
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let now = Instant::now();
    match part {
        1 => solve_part1(lines),
        2 => solve_part2(lines),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn evaluate1(expr: &String, start: usize, end: usize) -> u64 {
    let chars: Vec<char> = expr.chars().collect();
    let mut last_number = 0;
    let mut last_operation = ' ';
    let mut i = start;
    while i < end {
        match chars[i] {
            '+' => last_operation = '+',
            '*' => last_operation = '*',
            '(' => {
                let start = i;
                let mut paran_count = 0;
                while i < chars.len() {
                    match chars[i] {
                        '(' => paran_count += 1,
                        ')' => paran_count -= 1,
                        _ => {}
                    }
                    if paran_count == 0 {
                        break;
                    }
                    i += 1;
                }
                let new_number = evaluate1(&expr, start + 1, i);
                match last_operation {
                    ' ' => last_number = new_number,
                    '+' => last_number += new_number,
                    '*' => last_number *= new_number,
                    _ => {}
                }
            }
            ' ' => {}
            _ => {
                let start = i;
                while chars[i].is_digit(10) {
                    i += 1;
                    if i >= end {
                        break;
                    }
                }
                let number: &str = &expr[start..i];
                let new_number = number.parse::<u64>().unwrap();
                match last_operation {
                    ' ' => last_number = new_number,
                    '+' => last_number += new_number,
                    '*' => last_number *= new_number,
                    _ => {}
                }
                i -= 1;
            }
        }
        i += 1;
    }
    last_number
}

fn solve_part1(lines: Vec<String>) {
    // Evaluate "new" math expressions
    println!(
        "{}",
        lines.iter().map(|x| evaluate1(x, 0, x.len())).sum::<u64>()
    );
}

fn evaluate2(expr: &String, start: usize, end: usize) -> u64 {
    let chars: Vec<char> = expr.chars().collect();
    let mut last_numbers = vec![0];
    let mut current_number = 0;
    let mut last_operation = ' ';
    let mut i = start;
    while i < end {
        match chars[i] {
            '+' => last_operation = '+',
            '*' => last_operation = '*',
            '(' => {
                let start = i;
                let mut paran_count = 0;
                while i < chars.len() {
                    match chars[i] {
                        '(' => paran_count += 1,
                        ')' => paran_count -= 1,
                        _ => {}
                    }
                    if paran_count == 0 {
                        break;
                    }
                    i += 1;
                }
                let new_number = evaluate2(&expr, start + 1, i);
                match last_operation {
                    ' ' => last_numbers[current_number] = new_number,
                    '+' => last_numbers[current_number] += new_number,
                    '*' => {
                        last_numbers.push(new_number);
                        current_number += 1;
                    }
                    _ => {}
                }
            }
            ' ' => {}
            _ => {
                let start = i;
                while chars[i].is_digit(10) {
                    i += 1;
                    if i >= end {
                        break;
                    }
                }
                let number: &str = &expr[start..i];
                let new_number = number.parse::<u64>().unwrap();
                match last_operation {
                    ' ' => last_numbers[current_number] = new_number,
                    '+' => last_numbers[current_number] += new_number,
                    '*' => {
                        last_numbers.push(new_number);
                        current_number += 1;
                    }
                    _ => {}
                }
                i -= 1;
            }
        }
        i += 1;
    }
    last_numbers.iter().product()
}

fn solve_part2(lines: Vec<String>) {
    // Evaluate advanced math expressions
    println!(
        "{}",
        lines.iter().map(|x| evaluate2(x, 0, x.len())).sum::<u64>()
    );
}
