use std::env;

mod solutions;

fn aoc(day: u64, part: u64, filename: String) {
    match day {
        1 => {
            if part == 1 {
                solutions::day1::solve_part1(filename);
            } else if part == 2 {
                solutions::day1::solve_part2(filename);
            }
        }
        2 => {
            if part == 1 {
                solutions::day2::solve_part1(filename);
            } else if part == 2 {
                solutions::day2::solve_part2(filename);
            }
        }
        3 => {
            if part == 1 {
                solutions::day3::solve_part1(filename);
            } else if part == 2 {
                solutions::day3::solve_part2(filename);
            }
        }
        4 => {
            if part == 1 {
                solutions::day4::solve_part1(filename);
            } else if part == 2 {
                solutions::day4::solve_part2(filename);
            }
        }
        5 => {
            if part == 1 {
                solutions::day5::solve_part1(filename);
            } else if part == 2 {
                solutions::day5::solve_part2(filename);
            }
        }
        6 => {
            if part == 1 {
                solutions::day6::solve_part1(filename);
            } else if part == 2 {
                solutions::day6::solve_part2(filename);
            }
        }
        7 => {
            if part == 1 {
                solutions::day7::solve_part1(filename);
            } else if part == 2 {
                solutions::day7::solve_part2(filename);
            }
        }
        8 => {
            if part == 1 {
                solutions::day8::solve_part1(filename);
            } else if part == 2 {
                solutions::day8::solve_part2(filename);
            }
        }
        9 => {
            if part == 1 {
                solutions::day9::solve_part1(filename);
            } else if part == 2 {
                solutions::day9::solve_part2(filename);
            }
        }
        10 => {
            if part == 1 {
                solutions::day10::solve_part1(filename);
            } else if part == 2 {
                solutions::day10::solve_part2(filename);
            }
        }
        _ => {}
    }
}

fn main() {
    let params: Vec<String> = env::args().collect();
    if params.len() < 4 {
        println!("Specify the day number, part and input filename");
        return;
    }
    if let Ok(day) = params[1].parse::<u64>() {
        if let Ok(part) = params[2].parse::<u64>() {
            println!("=============================");
            println!("   Advent of Code : Day {:2} ", day);
            println!("=============================");
            let filename = params[3].clone();
            aoc(day, part, filename);
        } else {
            println!("Incorrect part number");
        }
    } else {
        println!("Incorrect day number");
    }
}
