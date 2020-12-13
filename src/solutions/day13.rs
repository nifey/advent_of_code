use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<&str> = input_data.lines().map(|x| x.trim()).collect();
    let depart_time: u64 = lines[0].parse::<u64>().unwrap();
    let bus_times: Vec<u64> = lines[1]
        .split(",")
        .map(|x| if x == "x" { "0" } else { x })
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let now = Instant::now();
    match part {
        1 => solve_part1(depart_time, bus_times),
        2 => solve_part2(bus_times),
        _ => {}
    }
    println!("Time: {} microseconds", now.elapsed().as_micros());
}

fn solve_part1(depart_time: u64, bus_times: Vec<u64>) {
    // Finding the earliest bus we can catch
    let mut buses: Vec<_> = bus_times
        .iter()
        .filter(|x| !(**x == 0))
        .map(|x| (x - (depart_time % x), x))
        .collect();
    buses.sort();
    println!("{:?}", buses[0].0 * buses[0].1);
}

fn coprime(num1: u64, num2: u64) -> bool {
    let mut a;
    let mut b;
    if num1 > num2 {
        a = num1;
        b = num2;
    } else {
        a = num2;
        b = num1;
    }
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a == 1;
}

fn euclid(a: i128, b: i128) -> (i128, i128) {
    if a == 0 {
        (0, 1)
    } else {
        let subresult = euclid(b % a, a);
        ((subresult.1 - (b / a) * subresult.0), subresult.0)
    }
}

fn inverse(num1: u64, num2: u64) -> u64 {
    if num2 == 0 {
        0
    } else {
        let result = euclid(num1 as i128, num2 as i128);
        ((result.0 + num2 as i128) % num2 as i128) as u64
    }
}

fn solve_part2(bus_times: Vec<u64>) {
    // Chinese remainder theorm
    // First check if all pairs are coprime (GCD == 1)
    // Then find the number using Chinese remainder theorm
    let mut bus_list = Vec::new();
    for i in 0..bus_times.len() {
        if bus_times[i] != 0 {
            bus_list.push((i, bus_times[i]));
        }
    }

    // Check if pairwise coprime
    for bus in &bus_list {
        if !bus_list
            .iter()
            .filter(|&x| x.1 != bus.1)
            .map(|&x| coprime(x.1, bus.1))
            .fold(true, |x, y| x && y)
        {
            println!("Not pairwise coprime");
            std::process::exit(0);
        }
    }

    let big_n: u64 = bus_list.iter().map(|x| x.1).product();
    let n = bus_list.iter().map(|x| big_n / x.1).collect::<Vec<u64>>();
    let inv = (0..bus_list.len())
        .map(|x| inverse(n[x], bus_list[x].1))
        .collect::<Vec<u64>>();
    let answer = (0..bus_list.len())
        .map(|x| {
            let mut val = bus_list[x].1 as i64 - bus_list[x].0 as i64;
            if val < 0 {
                val += big_n as i64
            }
            (val as u128 * n[x] as u128 * inv[x] as u128) % big_n as u128
        })
        .fold(0, |x, y| (x + y) % big_n as u128);
    println!("{}", answer);
}
