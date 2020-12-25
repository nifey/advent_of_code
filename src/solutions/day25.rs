use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut numbers = input_data
        .lines()
        .map(|x| x.to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let key1 = numbers.pop().unwrap();
    let key2 = numbers.pop().unwrap();

    let now = Instant::now();
    match part {
        1 => solve_part1(key1, key2),
        2 => solve_part2(key1, key2),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_millis());
}

fn solve_part1(key1: u64, key2: u64) {
    // Diffie Hellman key exchange
    let g = 7;
    let p = 20201227;

    let mut found1: bool = false;
    let mut found2: bool = false;
    let mut a = 0;
    let mut b = 0;
    let mut current = 1;
    let mut iter = 1;
    while !found1 && !found2 {
        current = (current * g) % p;
        if current == key1 {
            found1 = true;
            a = iter;
        }
        if current == key2 {
            found2 = true;
            b = iter;
        }
        iter += 1;
    }

    if found1 {
        current = 1;
        for _ in 0..a {
            current = (current * key2) % p;
        }
    }

    if found2 {
        current = 1;
        for _ in 0..b {
            current = (current * key1) % p;
        }
    }

    println!("{}", current);
}

fn solve_part2(key1: u64, key2: u64) {}
