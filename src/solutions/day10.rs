use std::fs;

pub fn solve_part1(filename: String) {
    // Charging adaptors
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut numbers: Vec<i64> = input_data
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    numbers.sort();
    numbers.push(numbers[numbers.len() - 1] + 3);
    numbers.insert(0, 0);

    let mut one_jolt = 0;
    let mut three_jolt = 0;
    for i in 0..numbers.len() - 1 {
        match numbers[i + 1] - numbers[i] {
            1 => one_jolt += 1,
            3 => three_jolt += 1,
            _ => {}
        }
    }

    println!("{}", one_jolt * three_jolt);
}

pub fn solve_part2(filename: String) {
    // Charging adaptors
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let mut numbers: Vec<i64> = input_data
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    numbers.sort();
    numbers.insert(0, 0);
    let mut num_ways: Vec<u64> = vec![0; numbers.len()];
    num_ways[numbers.len() - 1] = 1;

    for i in (0..(numbers.len() - 1)).rev() {
        let mut count = 0;
        let number = numbers[i];
        for j in (i + 1)..(i + 4) {
            if j < numbers.len() && numbers[j] <= number + 3 {
                count += num_ways[j];
            }
        }
        num_ways[i] = count;
    }

    println!("{}", num_ways[0]);
}
