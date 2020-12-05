use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

fn get_seat_id(bp: &str) -> u64 {
    let mut low_row = 0;
    let mut high_row = 127;
    let mut low_col = 0;
    let mut high_col = 7;
    let letters: Vec<char> = bp.chars().collect();
    for i in 0..7 {
        if letters[i] == 'F' {
            high_row = (low_row + high_row) / 2;
        } else if letters[i] == 'B' {
            low_row = ((low_row + high_row) / 2) + 1;
        }
    }
    for i in 7..10 {
        if letters[i] == 'L' {
            high_col = (low_col + high_col) / 2;
        } else if letters[i] == 'R' {
            low_col = ((low_col + high_col) / 2) + 1;
        }
    }

    low_row * 8 + low_col
}

pub fn solve_part1(filename: String) {
    // Finding max seat id
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let max_seat_id = input_data
        .lines()
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|x| get_seat_id(x))
        .max()
        .unwrap();
    println!("{}", max_seat_id);
}

pub fn solve_part2(filename: String) {
    // Finding max seat id
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let occupied_seats: HashSet<u64> = input_data
        .lines()
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|x| get_seat_id(x))
        .filter(|x| *x > 7 && *x <= 1016)
        .collect();
    let all_seats: HashSet<u64> = (8..1017).clone().collect();
    let empty_seats: Vec<u64> = all_seats.difference(&occupied_seats).map(|x| *x).collect();
    for seat in empty_seats.iter() {
        if !empty_seats.contains(&(seat - 1)) && !empty_seats.contains(&(seat + 1)) {
            println!("{}", seat);
        }
    }
}
