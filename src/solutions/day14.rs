use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

enum Command {
    Mask(String),  // The parameter is the mask
    Mem(u64, u64), // The parameters are the address and value written
}

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let commands: Vec<Command> = input_data
        .lines()
        .map(|x| {
            let current_line = x.trim();
            if current_line.starts_with("mask") {
                let mask: String = x.split(" = ").skip(1).next().unwrap().to_string();
                Command::Mask(mask)
            } else {
                let mut parts = x.split(" = ");
                let address: u64 = parts
                    .next()
                    .unwrap()
                    .split("[")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split("]")
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let data: u64 = parts.next().unwrap().parse::<u64>().unwrap();
                Command::Mem(address, data)
            }
        })
        .collect();

    let now = Instant::now();
    match part {
        1 => solve_part1(commands),
        2 => solve_part2(commands),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn solve_part1(commands: Vec<Command>) {
    // Sum of values in memory
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and_mask = 0;
    let mut or_mask = 0;
    for command in commands {
        match command {
            Command::Mask(mask) => {
                and_mask = mask
                    .chars()
                    .map(|x| match x {
                        'X' => 1,
                        '1' => 1,
                        '0' => 0,
                        _ => 0,
                    })
                    .fold(0, |x, y| x * 2 + y);
                or_mask = mask
                    .chars()
                    .map(|x| match x {
                        'X' => 0,
                        '1' => 1,
                        '0' => 0,
                        _ => 0,
                    })
                    .fold(0, |x, y| x * 2 + y);
            }
            Command::Mem(address, data) => {
                let write_data = (data & and_mask) | or_mask;
                memory.insert(address, write_data);
            }
        }
    }

    let sum: u64 = memory.values().sum();
    println!("{}", sum);
}

fn compute_addresses(mask: Vec<char>, address: u64) -> HashSet<u64> {
    let mut addresses: HashSet<u64> = HashSet::new();
    for (index, bit) in mask.iter().enumerate() {
        match bit {
            'X' => {
                let mut new_mask = mask.clone();
                new_mask[index] = '0';
                let new_address = address & (!(1 << (35 - index)));
                addresses = addresses
                    .union(&compute_addresses(new_mask.clone(), new_address))
                    .map(|x| *x)
                    .collect();
                let new_address = address | (1 << (35 - index));
                addresses = addresses
                    .union(&compute_addresses(new_mask, new_address))
                    .map(|x| *x)
                    .collect();
            }
            _ => {}
        }
    }
    addresses.insert(address);
    addresses
}

fn solve_part2(commands: Vec<Command>) {
    // Sum of values in memory with decoder 2
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask: Vec<char> = Vec::new();
    let mut or_mask: u64 = 0;
    for command in commands {
        match command {
            Command::Mask(mask) => {
                current_mask = mask.chars().collect();
                or_mask = mask
                    .chars()
                    .map(|x| match x {
                        'X' => 0,
                        '1' => 1,
                        '0' => 0,
                        _ => 0,
                    })
                    .fold(0, |x, y| x * 2 + y);
            }
            Command::Mem(address, data) => {
                for address in compute_addresses(current_mask.clone(), address | or_mask) {
                    memory.insert(address, data);
                }
            }
        }
    }

    let sum: u64 = memory.values().sum();
    println!("{}", sum);
}
