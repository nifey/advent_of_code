use std::env;

mod solutions;

fn aoc(day: u64) {
    match day {
        1 => solutions::day1::solve(),
        _ => {}
    }
}

fn main() {
    let mut params = env::args();
    if params.len() < 2 {
        println!("Specify the day number");
        return;
    }
    if let Ok(day) = params.nth(1).unwrap().parse::<u64>() {
        println!("=============================");
        println!("   Advent of Code : Day {:2} ", day);
        println!("=============================");
        aoc(day);
    }
}
