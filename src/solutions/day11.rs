use std::fs;

fn print_grid(gen: &Vec<Vec<bool>>, seats: &Vec<Vec<bool>>) {
    let rows = gen.len();
    let cols = gen.len();
    for i in 0..rows {
        println!(
            "{}",
            (0..cols)
                .map(|j| {
                    if !seats[i][j] {
                        '.'
                    } else if gen[i][j] {
                        'L'
                    } else {
                        '#'
                    }
                })
                .collect::<String>()
        );
    }
    println!("{}", (0..cols).map(|_| '=').collect::<String>());
}

pub fn solve_part1(filename: String) {
    // Seat simulation
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let seats: Vec<Vec<bool>> = input_data
        .lines()
        .map(|x| {
            x.trim()
                .chars()
                .map(|y| match y {
                    'L' => true,
                    '.' => false,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    let rows = seats.len();
    let cols = seats[0].len();

    let mut prev_gen = seats.clone();
    loop {
        let next_gen: Vec<Vec<bool>> = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| {
                        // Count neighbors
                        let mut occupied = 0;
                        for ki in 0..=2 {
                            for kj in 0..=2 {
                                if ki == 1 && kj == 1 {
                                    continue;
                                }
                                if (i as i64 + ki as i64 - 1) < rows as i64
                                    && (i as i64 + ki as i64 - 1) >= 0
                                    && (j as i64 + kj as i64 - 1) < cols as i64
                                    && (j as i64 + kj as i64 - 1) >= 0
                                    && seats[i + ki - 1][j + kj - 1]
                                {
                                    if !prev_gen[i + ki - 1][j + kj - 1] {
                                        occupied += 1;
                                    }
                                }
                            }
                        }
                        if seats[i][j] && occupied == 0 {
                            false
                        } else if seats[i][j] && !prev_gen[i][j] && occupied >= 4 {
                            true
                        } else {
                            prev_gen[i][j]
                        }
                    })
                    .collect()
            })
            .collect();

        if next_gen == prev_gen {
            print_grid(&next_gen, &seats);
            let mut count = 0;
            for i in 0..rows {
                for j in 0..cols {
                    if seats[i][j] && !next_gen[i][j] {
                        count += 1;
                    }
                }
            }
            println!("{}", count);
            break;
        }
        prev_gen = next_gen;
    }
}

pub fn solve_part2(filename: String) {
    // Seat simulation 2
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let seats: Vec<Vec<bool>> = input_data
        .lines()
        .map(|x| {
            x.trim()
                .chars()
                .map(|y| match y {
                    'L' => true,
                    '.' => false,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    let rows = seats.len();
    let cols = seats[0].len();

    let mut prev_gen = seats.clone();
    loop {
        let next_gen: Vec<Vec<bool>> = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| {
                        // Count neighbors
                        let mut occupied = 0;
                        for ki in 0..=2 {
                            for kj in 0..=2 {
                                let row_diff: i64 = ki - 1;
                                let col_diff: i64 = kj - 1;
                                if row_diff == 0 && col_diff == 0 {
                                    continue;
                                }
                                let mut curr_row = i as i64 + row_diff;
                                let mut curr_col = j as i64 + col_diff;
                                while curr_row < rows as i64
                                    && curr_row >= 0
                                    && curr_col < cols as i64
                                    && curr_col >= 0
                                    && !seats[curr_row as usize][curr_col as usize]
                                {
                                    curr_row += row_diff;
                                    curr_col += col_diff;
                                }

                                if curr_row < rows as i64
                                    && curr_row >= 0
                                    && curr_col < cols as i64
                                    && curr_col >= 0
                                    && seats[curr_row as usize][curr_col as usize]
                                {
                                    if !prev_gen[curr_row as usize][curr_col as usize] {
                                        occupied += 1;
                                    }
                                }
                            }
                        }
                        if seats[i][j] && occupied == 0 {
                            false
                        } else if seats[i][j] && !prev_gen[i][j] && occupied >= 5 {
                            true
                        } else {
                            prev_gen[i][j]
                        }
                    })
                    .collect()
            })
            .collect();

        if next_gen == prev_gen {
            print_grid(&next_gen, &seats);
            let mut count = 0;
            for i in 0..rows {
                for j in 0..cols {
                    if seats[i][j] && !next_gen[i][j] {
                        count += 1;
                    }
                }
            }
            println!("{}", count);
            break;
        }
        prev_gen = next_gen;
    }
}
