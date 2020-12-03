use rayon::prelude::*;
use std::fs;

pub fn solve_part1(filename: String) {
    // Finding number of trees in the sloping path
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut open_space: Vec<Vec<bool>> = vec![];
    for line in input_data.lines() {
        let mut row: Vec<bool> = vec![];
        for space in line.chars() {
            if space == '#' {
                row.push(false);
            } else if space == '.' {
                row.push(true);
            }
        }
        open_space.push(row);
    }

    let height = open_space.len();
    let width = open_space[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    while y < height {
        if !open_space[y][x] {
            tree_count += 1;
        }
        x = (x + 3) % width;
        y = y + 1;
    }
    println!("{}", tree_count);
}

pub fn solve_part2(filename: String) {
    // Finding number of trees in the multiple sloping path
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut open_space: Vec<Vec<bool>> = vec![];
    for line in input_data.lines() {
        let mut row: Vec<bool> = vec![];
        for space in line.chars() {
            if space == '#' {
                row.push(false);
            } else if space == '.' {
                row.push(true);
            }
        }
        open_space.push(row);
    }

    let height = open_space.len();
    let width = open_space[0].len();
    let xdiff = vec![1, 3, 5, 7, 1];
    let ydiff = vec![1, 1, 1, 1, 2];

    let answer: u64 = (0..xdiff.len())
        .into_par_iter()
        .map(|id| {
            let mut x = 0;
            let mut y = 0;
            let mut tree_count = 0;
            while y < height {
                if !open_space[y][x] {
                    tree_count += 1;
                }
                x = (x + xdiff[id]) % width;
                y = y + ydiff[id];
            }
            tree_count
        })
        .product();

    println!("{}", answer);
}
