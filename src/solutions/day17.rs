use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let grid: Vec<Vec<bool>> = input_data
        .lines()
        .map(|x| {
            x.trim()
                .chars()
                .map(|x| match x {
                    '#' => true,
                    '.' => false,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let now = Instant::now();
    match part {
        1 => solve_part1(grid),
        2 => solve_part2(grid),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn index(x: usize, y: usize, z: usize, x_dim: usize, y_dim: usize) -> usize {
    z * (x_dim * y_dim) + y * (x_dim) + x
}

/*
fn print(grid: &Vec<bool>, x_dim: usize, y_dim: usize, z_dim: usize) {
    for z in 0..z_dim {
        println!("z = {}", z as i64 - 7);
        for y in 0..y_dim {
            for x in 0..x_dim {
                if grid[index(x, y, z, x_dim, y_dim)] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}
*/

fn solve_part1(grid: Vec<Vec<bool>>) {
    // Simulate 6 cycles
    let x_dim = grid[0].len();
    let y_dim = grid.len();
    let tot_x_dim = x_dim + 16;
    let tot_y_dim = y_dim + 16;
    let tot_z_dim = 17;
    let mut current_gen = vec![false; (tot_x_dim) * (tot_y_dim) * tot_z_dim];
    let mut next_gen = vec![false; (tot_x_dim) * (tot_y_dim) * tot_z_dim];

    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            if grid[x][y] {
                current_gen[index(
                    x + (tot_x_dim / 2) - (x_dim / 2),
                    y + (tot_y_dim / 2) - (y_dim / 2),
                    tot_z_dim / 2,
                    tot_x_dim,
                    tot_y_dim,
                )] = true;
            }
        }
    }

    for gen in 1..=6 {
        // Evaluate next gen using current gen
        for z in (tot_z_dim / 2 - gen)..=(tot_z_dim / 2 + gen) {
            for y in (tot_y_dim / 2 - y_dim / 2 - gen)..=(tot_y_dim / 2 + y_dim / 2 + gen) {
                for x in (tot_x_dim / 2 - x_dim / 2 - gen)..=(tot_x_dim / 2 + x_dim / 2 + gen) {
                    let mut count = 0;
                    for z_diff in 0..=2 {
                        for y_diff in 0..=2 {
                            for x_diff in 0..=2 {
                                if x_diff == y_diff && x_diff == z_diff && x_diff == 1 {
                                    continue;
                                }
                                if current_gen[index(
                                    (x as i64 + x_diff as i64 - 1) as usize,
                                    (y as i64 + y_diff as i64 - 1) as usize,
                                    (z as i64 + z_diff as i64 - 1) as usize,
                                    tot_x_dim,
                                    tot_y_dim,
                                )] {
                                    count += 1;
                                }
                            }
                        }
                    }

                    let current_index = index(x, y, z, tot_x_dim, tot_y_dim);
                    if current_gen[current_index] {
                        if count == 2 || count == 3 {
                            next_gen[current_index] = true;
                        } else {
                            next_gen[current_index] = false;
                        }
                    } else {
                        if count == 3 {
                            next_gen[current_index] = true;
                        } else {
                            next_gen[current_index] = false;
                        }
                    }
                }
            }
        }

        // Copy next gen to current gen
        std::mem::swap(&mut current_gen, &mut next_gen);
    }

    //print(&current_gen, tot_x_dim, tot_y_dim, tot_z_dim);

    let mut count = 0;
    for z in (tot_z_dim / 2 - 6)..=(tot_z_dim / 2 + 6) {
        for y in (tot_y_dim / 2 - y_dim / 2 - 6)..=(tot_y_dim / 2 + y_dim / 2 + 6) {
            for x in (tot_x_dim / 2 - x_dim / 2 - 6)..=(tot_x_dim / 2 + x_dim / 2 + 6) {
                if current_gen[index(x, y, z, tot_x_dim, tot_y_dim)] {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn index4d(
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    x_dim: usize,
    y_dim: usize,
    z_dim: usize,
) -> usize {
    w * (x_dim * y_dim * z_dim) + z * (x_dim * y_dim) + y * (x_dim) + x
}

/*
fn print4d(grid: &Vec<bool>, x_dim: usize, y_dim: usize, z_dim: usize, w_dim: usize) {
    for w in 0..w_dim {
        println!("w = {}", w as i64 - 7);
        for z in 0..z_dim {
            println!("z = {}", z as i64 - 7);
            for y in 0..y_dim {
                for x in 0..x_dim {
                    if grid[index4d(x, y, z, w, x_dim, y_dim, z_dim)] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
        }
    }
}
*/

fn solve_part2(grid: Vec<Vec<bool>>) {
    // Simulate 6 cycles on 4D
    let x_dim = grid[0].len();
    let y_dim = grid.len();
    let tot_x_dim = x_dim + 16;
    let tot_y_dim = y_dim + 16;
    let tot_z_dim = 17;
    let tot_w_dim = 17;
    let mut current_gen = vec![false; tot_x_dim * tot_y_dim * tot_z_dim * tot_w_dim];
    let mut next_gen = vec![false; tot_x_dim * tot_y_dim * tot_z_dim * tot_w_dim];

    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            if grid[x][y] {
                current_gen[index4d(
                    x + (tot_x_dim / 2) - (x_dim / 2),
                    y + (tot_y_dim / 2) - (y_dim / 2),
                    tot_z_dim / 2,
                    tot_w_dim / 2,
                    tot_x_dim,
                    tot_y_dim,
                    tot_z_dim,
                )] = true;
            }
        }
    }

    for gen in 1..=6 {
        // Evaluate next gen using current gen
        for w in (tot_w_dim / 2 - gen)..=(tot_w_dim / 2 + gen) {
            for z in (tot_z_dim / 2 - gen)..=(tot_z_dim / 2 + gen) {
                for y in (tot_y_dim / 2 - y_dim / 2 - gen)..=(tot_y_dim / 2 + y_dim / 2 + gen) {
                    for x in (tot_x_dim / 2 - x_dim / 2 - gen)..=(tot_x_dim / 2 + x_dim / 2 + gen) {
                        let mut count = 0;
                        for w_diff in 0..=2 {
                            for z_diff in 0..=2 {
                                for y_diff in 0..=2 {
                                    for x_diff in 0..=2 {
                                        if x_diff == y_diff
                                            && x_diff == z_diff
                                            && x_diff == w_diff
                                            && x_diff == 1
                                        {
                                            continue;
                                        }
                                        if current_gen[index4d(
                                            (x as i64 + x_diff as i64 - 1) as usize,
                                            (y as i64 + y_diff as i64 - 1) as usize,
                                            (z as i64 + z_diff as i64 - 1) as usize,
                                            (w as i64 + w_diff as i64 - 1) as usize,
                                            tot_x_dim,
                                            tot_y_dim,
                                            tot_z_dim,
                                        )] {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }

                        let current_index = index4d(x, y, z, w, tot_x_dim, tot_y_dim, tot_z_dim);
                        if current_gen[current_index] {
                            if count == 2 || count == 3 {
                                next_gen[current_index] = true;
                            } else {
                                next_gen[current_index] = false;
                            }
                        } else {
                            if count == 3 {
                                next_gen[current_index] = true;
                            } else {
                                next_gen[current_index] = false;
                            }
                        }
                    }
                }
            }
        }

        // Copy next gen to current gen
        std::mem::swap(&mut current_gen, &mut next_gen);
    }

    //print4d(&current_gen, tot_x_dim, tot_y_dim, tot_z_dim, tot_w_dim);

    let mut count = 0;
    for w in (tot_w_dim / 2 - 6)..=(tot_w_dim / 2 + 6) {
        for z in (tot_z_dim / 2 - 6)..=(tot_z_dim / 2 + 6) {
            for y in (tot_y_dim / 2 - y_dim / 2 - 6)..=(tot_y_dim / 2 + y_dim / 2 + 6) {
                for x in (tot_x_dim / 2 - x_dim / 2 - 6)..=(tot_x_dim / 2 + x_dim / 2 + 6) {
                    if current_gen[index4d(x, y, z, w, tot_x_dim, tot_y_dim, tot_z_dim)] {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
