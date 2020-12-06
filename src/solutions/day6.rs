use std::collections::HashSet;
use std::fs;

pub fn solve_part1(filename: String) {
    // Finding sum of custom form anyone answered
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();
    let mut sum = 0;
    let mut flags: HashSet<char> = HashSet::new();
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            sum += flags.len();
            flags.clear();
        } else {
            let data = lines[i].chars().collect::<Vec<char>>();
            for entry in data {
                flags.insert(entry);
            }
        }
    }
    sum += flags.len();
    println!("{}", sum);
}

pub fn solve_part2(filename: String) {
    // Finding sum of custom form anyone answered
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();
    let mut sum = 0;
    let mut flags: HashSet<char> = HashSet::new();
    let mut first = true;
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            sum += flags.len();
            flags.clear();
            first = true;
        } else {
            let data = lines[i].chars().collect::<HashSet<char>>();
            let mut newflags: HashSet<char> = HashSet::new();
            if !first {
                for entry in flags.intersection(&data) {
                    newflags.insert(*entry);
                }
                flags = newflags;
            } else {
                flags = data;
                first = false;
            }
        }
    }
    sum += flags.len();
    println!("{}", sum);
}
