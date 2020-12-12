use std::fs;

enum Direction {
    North = 0,
    East,
    South,
    West,
}

pub fn solve_part1(filename: String) {
    // Manhattan distance
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let instructions: Vec<(char, i64)> = input_data
        .lines()
        .map(|x| {
            x.trim();
            (
                x.chars().next().unwrap(),
                x.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap(),
            )
        })
        .collect();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut current_direction: Direction = Direction::East;
    for (command, argument) in instructions {
        match command {
            'N' => y += argument,
            'S' => y -= argument,
            'E' => x += argument,
            'W' => x -= argument,
            'L' => {
                let current_degrees = match current_direction {
                    Direction::North => 0,
                    Direction::East => 90,
                    Direction::South => 180,
                    Direction::West => 270,
                };
                let mut degrees = current_degrees - argument;
                if degrees < 0 {
                    degrees += 360;
                }
                current_direction = match degrees {
                    0 => Direction::North,
                    90 => Direction::East,
                    180 => Direction::South,
                    270 => Direction::West,
                    _ => Direction::North,
                }
            }
            'R' => {
                let current_degrees = match current_direction {
                    Direction::North => 0,
                    Direction::East => 90,
                    Direction::South => 180,
                    Direction::West => 270,
                };
                let mut degrees = current_degrees + argument;
                if degrees > 360 {
                    degrees -= 360;
                }
                current_direction = match degrees {
                    0 => Direction::North,
                    90 => Direction::East,
                    180 => Direction::South,
                    270 => Direction::West,
                    _ => Direction::North,
                }
            }
            'F' => match current_direction {
                Direction::North => {
                    y += argument;
                }
                Direction::East => {
                    x += argument;
                }
                Direction::South => {
                    y -= argument;
                }
                Direction::West => {
                    x -= argument;
                }
            },
            _ => {}
        }
    }
    println!("{}", x.abs() + y.abs());
}

pub fn solve_part2(filename: String) {
    // Manhattan distance using waypoint
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let instructions: Vec<(char, i64)> = input_data
        .lines()
        .map(|x| {
            x.trim();
            (
                x.chars().next().unwrap(),
                x.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap(),
            )
        })
        .collect();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut wp_x: i64 = 10;
    let mut wp_y: i64 = 1;
    for (command, argument) in instructions {
        match command {
            'N' => wp_y += argument,
            'S' => wp_y -= argument,
            'E' => wp_x += argument,
            'W' => wp_x -= argument,
            'L' => match argument / 90 {
                1 => {
                    let old_wp_x = wp_x;
                    let old_wp_y = wp_y;
                    wp_x = -old_wp_y;
                    wp_y = old_wp_x;
                }
                2 => {
                    let old_wp_x = wp_x;
                    let old_wp_y = wp_y;
                    wp_x = -old_wp_x;
                    wp_y = -old_wp_y;
                }
                3 => {
                    let old_wp_x = wp_x;
                    let old_wp_y = wp_y;
                    wp_x = old_wp_y;
                    wp_y = -old_wp_x;
                }
                _ => {}
            },
            'R' => match argument / 90 {
                1 => {
                    let old_wp_x = wp_x;
                    let old_wp_y = wp_y;
                    wp_x = old_wp_y;
                    wp_y = -old_wp_x;
                }
                2 => {
                    let old_wp_x = wp_x;
                    let old_wp_y = wp_y;
                    wp_x = -old_wp_x;
                    wp_y = -old_wp_y;
                }
                3 => {
                    let old_wp_x = wp_x;
                    let old_wp_y = wp_y;
                    wp_x = -old_wp_y;
                    wp_y = old_wp_x;
                }
                _ => {}
            },
            'F' => {
                x += argument * wp_x;
                y += argument * wp_y;
            }
            _ => {}
        }
    }
    println!("{}", x.abs() + y.abs());
}
