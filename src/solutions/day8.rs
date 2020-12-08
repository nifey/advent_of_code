use std::collections::HashSet;
use std::fs;

pub fn solve_part1(filename: String) {
    // Finding acc value at infinite loop
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();

    let mut accumulator: i64 = 0;
    let mut i: i64 = 0;
    let mut executed: HashSet<i64> = HashSet::new();

    loop {
        let mut split = lines[i as usize].trim().split(" ");
        let command = split.next().unwrap();
        let value = split.next().unwrap().parse::<i64>().unwrap();

        if executed.contains(&i) {
            break;
        } else {
            executed.insert(i);
        }

        match command {
            "acc" => {
                accumulator += value;
            }
            "jmp" => {
                i = i + value - 1;
            }
            "nop" => {}
            _ => {}
        }
        i += 1;
    }

    println!("{}", accumulator);
}

pub fn solve_part2(filename: String) {
    // Fixing the infinite loop
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();

    let mut i: i64 = 0;
    let mut executed: HashSet<i64> = HashSet::new();

    let mut fixed_instruction: i64 = -1;
    let mut sequence: Vec<i64> = Vec::new();

    loop {
        if i as usize >= lines.len() {
            break;
        }

        if executed.contains(&i) {
            // Backtrack
            let mut backtrack_to_fixed_instruction = false;
            if fixed_instruction == -1 {
                fixed_instruction = sequence[(sequence.len() as i64 - 1) as usize];
            }
            loop {
                if !backtrack_to_fixed_instruction {
                    while i != fixed_instruction {
                        i = sequence.pop().unwrap();
                        executed.remove(&i);
                    }
                    backtrack_to_fixed_instruction = true;
                }
                i = sequence.pop().unwrap();
                executed.remove(&i);
                i = sequence[(sequence.len() as i64 - 1) as usize];
                match lines[i as usize].trim().split(" ").next().unwrap() {
                    "jmp" => {
                        fixed_instruction = i;
                        break;
                    }
                    "nop" => {
                        fixed_instruction = i;
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            executed.insert(i);
            sequence.push(i);
        }

        let mut split = lines[i as usize].trim().split(" ");
        let command = split.next().unwrap();
        let value = split.next().unwrap().parse::<i64>().unwrap();

        match command {
            "acc" => {}
            "jmp" => {
                if i != fixed_instruction {
                    i = i + value - 1;
                }
            }
            "nop" => {
                if i == fixed_instruction {
                    i = i + value - 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Now simulate the acc instructions with the fixed instruction
    let mut accumulator: i64 = 0;
    i = 0;
    executed.clear();
    loop {
        if i as usize >= lines.len() {
            break;
        }

        let mut split = lines[i as usize].trim().split(" ");
        let command = split.next().unwrap();
        let value = split.next().unwrap().parse::<i64>().unwrap();

        if executed.contains(&i) {
            println!(" {} ", i);
            println!(" {} ", fixed_instruction);
            println!("Something is wrong");
            break;
        } else {
            executed.insert(i);
        }

        match command {
            "acc" => {
                accumulator += value;
            }
            "jmp" => {
                if i != fixed_instruction {
                    i = i + value - 1;
                }
            }
            "nop" => {
                if i == fixed_instruction {
                    i = i + value - 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    println!("{}", accumulator);
}
