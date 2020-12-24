use std::collections::HashSet;
use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let tiles = input_data
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let now = Instant::now();
    match part {
        1 => solve_part1(tiles),
        2 => solve_part2(tiles),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_millis());
}

fn solve_part1(tiles: Vec<String>) {
    let mut black_tiles: HashSet<(i64, i64)> = HashSet::new();
    for tile in tiles {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut index = 0;
        let chars = tile.chars().collect::<Vec<char>>();
        while index < chars.len() {
            match chars[index] {
                'e' => {
                    x += 1;
                    index += 1;
                }
                'w' => {
                    x -= 1;
                    index += 1;
                }
                'n' => match chars[index + 1] {
                    'e' => {
                        if y.abs() % 2 == 0 {
                            x += 1;
                        }
                        y += 1;
                        index += 2;
                    }
                    'w' => {
                        if y.abs() % 2 == 1 {
                            x -= 1;
                        }
                        y += 1;
                        index += 2;
                    }
                    _ => {}
                },
                's' => match chars[index + 1] {
                    'e' => {
                        if y.abs() % 2 == 0 {
                            x += 1;
                        }
                        y -= 1;
                        index += 2;
                    }
                    'w' => {
                        if y.abs() % 2 == 1 {
                            x -= 1;
                        }
                        y -= 1;
                        index += 2;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if black_tiles.contains(&(x, y)) {
            black_tiles.remove(&(x, y));
        } else {
            black_tiles.insert((x, y));
        }
    }

    println!("{}", black_tiles.len());
}

fn solve_part2(tiles: Vec<String>) {
    let mut black_tiles: HashSet<(i64, i64)> = HashSet::new();
    let mut min_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    for tile in tiles {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut index = 0;
        let chars = tile.chars().collect::<Vec<char>>();
        while index < chars.len() {
            match chars[index] {
                'e' => {
                    x += 1;
                    index += 1;
                }
                'w' => {
                    x -= 1;
                    index += 1;
                }
                'n' => match chars[index + 1] {
                    'e' => {
                        if y.abs() % 2 == 0 {
                            x += 1;
                        }
                        y += 1;
                        index += 2;
                    }
                    'w' => {
                        if y.abs() % 2 == 1 {
                            x -= 1;
                        }
                        y += 1;
                        index += 2;
                    }
                    _ => {}
                },
                's' => match chars[index + 1] {
                    'e' => {
                        if y.abs() % 2 == 0 {
                            x += 1;
                        }
                        y -= 1;
                        index += 2;
                    }
                    'w' => {
                        if y.abs() % 2 == 1 {
                            x -= 1;
                        }
                        y -= 1;
                        index += 2;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if black_tiles.contains(&(x, y)) {
            black_tiles.remove(&(x, y));
        } else {
            black_tiles.insert((x, y));
        }
    }

    for (x, y) in &black_tiles {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    let mut old_gen = black_tiles;
    for iter in 0..100 {
        let mut new_gen: HashSet<(i64, i64)> = HashSet::new();

        // Check black tiles
        for (x, y) in old_gen.clone() {
            let mut count = 0;
            if y.abs() % 2 == 0 {
                if old_gen.contains(&(x, y - 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x, y + 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x + 1, y - 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x + 1, y + 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x - 1, y)) {
                    count += 1;
                }
                if old_gen.contains(&(x + 1, y)) {
                    count += 1;
                }
            } else {
                if old_gen.contains(&(x, y - 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x, y + 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x - 1, y - 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x - 1, y + 1)) {
                    count += 1;
                }
                if old_gen.contains(&(x - 1, y)) {
                    count += 1;
                }
                if old_gen.contains(&(x + 1, y)) {
                    count += 1;
                }
            }
            if count == 1 || count == 2 {
                new_gen.insert((x, y));
            }
        }

        // Check white tiles
        let mut curr_x = min_x - 1;
        while curr_x <= max_x + 1 {
            let mut curr_y = min_y - 1;
            while curr_y <= max_y + 1 {
                let mut count = 0;
                if curr_y.abs() % 2 == 0 {
                    if old_gen.contains(&(curr_x, curr_y - 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x, curr_y + 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x + 1, curr_y - 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x + 1, curr_y + 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x - 1, curr_y)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x + 1, curr_y)) {
                        count += 1;
                    }
                } else {
                    if old_gen.contains(&(curr_x, curr_y - 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x, curr_y + 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x - 1, curr_y - 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x - 1, curr_y + 1)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x - 1, curr_y)) {
                        count += 1;
                    }
                    if old_gen.contains(&(curr_x + 1, curr_y)) {
                        count += 1;
                    }
                }
                if count == 2 {
                    new_gen.insert((curr_x, curr_y));
                    if curr_x < min_x {
                        min_x = curr_x;
                    }
                    if curr_x > max_x {
                        max_x = curr_x;
                    }
                    if curr_y < min_y {
                        min_y = curr_y;
                    }
                    if curr_y > max_y {
                        max_y = curr_y;
                    }
                }
                curr_y += 1;
            }
            curr_x += 1;
        }

        old_gen = new_gen;
    }

    println!("{}", old_gen.len());
}
