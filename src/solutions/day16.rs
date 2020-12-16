use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<&str> = input_data.lines().collect();

    let now = Instant::now();
    match part {
        1 => solve_part1(lines),
        2 => solve_part2(lines),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn solve_part1(lines: Vec<&str>) {
    let mut iter = lines.iter();
    let re = Regex::new(r"[a-zA-Z ]*: (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    loop {
        let line = iter.next().unwrap();
        if line.len() == 0 {
            break;
        }
        for cap in re.captures_iter(line) {
            let num1 = cap[1].parse::<u64>().unwrap();
            let num2 = cap[2].parse::<u64>().unwrap();
            let num3 = cap[3].parse::<u64>().unwrap();
            let num4 = cap[4].parse::<u64>().unwrap();
            ranges.push((num1, num2));
            ranges.push((num3, num4));
        }
    }

    let mut skipped_iter = iter.skip(4);
    let mut numbers: Vec<u64> = Vec::new();

    while let Some(line) = skipped_iter.next() {
        numbers.append(
            &mut line
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    for range in ranges {
        numbers = numbers
            .iter()
            .filter_map(|&x| {
                if x < range.0 || x > range.1 {
                    Some(x)
                } else {
                    None
                }
            })
            .collect();
    }
    println!("{}", numbers.iter().sum::<u64>());
}

fn solve_part2(lines: Vec<&str>) {
    let mut iter = lines.iter();
    let re = Regex::new(r"([a-zA-Z ]*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut rules: Vec<(String, u64, u64, u64, u64)> = Vec::new();
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    loop {
        let line = iter.next().unwrap();
        if line.len() == 0 {
            break;
        }
        for cap in re.captures_iter(line) {
            let field = cap[1].to_string();
            let num1 = cap[2].parse::<u64>().unwrap();
            let num2 = cap[3].parse::<u64>().unwrap();
            let num3 = cap[4].parse::<u64>().unwrap();
            let num4 = cap[5].parse::<u64>().unwrap();
            ranges.push((num1, num2));
            ranges.push((num3, num4));
            rules.push((field, num1, num2, num3, num4))
        }
    }

    let mut skipped_iter = iter.skip(1);
    let my_ticket = skipped_iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut skipped_iter = skipped_iter.skip(2);
    let mut tickets: Vec<Vec<u64>> = Vec::new();
    while let Some(line) = skipped_iter.next() {
        tickets.push(
            line.split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    tickets = tickets
        .iter()
        .filter_map(|v| {
            let mut valid;
            for &x in v {
                valid = false;
                for range in ranges.iter() {
                    if x >= range.0 && x <= range.1 {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    return None;
                }
            }
            Some(v.clone())
        })
        .collect();

    let mut valid_fields: Vec<HashSet<usize>> = Vec::new();

    for i in 0..my_ticket.len() {
        let data = tickets.iter().map(|x| x[i]).collect::<Vec<u64>>();
        let mut valid: HashSet<usize> = HashSet::new();
        for j in 0..rules.len() {
            if data
                .iter()
                .filter_map(|&x| {
                    if (x >= rules[j].1 && x <= rules[j].2) || (x >= rules[j].3 && x <= rules[j].4)
                    {
                        None
                    } else {
                        Some(x)
                    }
                })
                .collect::<Vec<u64>>()
                .len()
                == 0
            {
                valid.insert(j);
            }
        }
        valid_fields.push(valid);
    }

    let mut mappings: Vec<usize> = vec![0; my_ticket.len()];

    for i in 0..mappings.len() {
        for i in 0..valid_fields.len() {
            if valid_fields[i].len() == 1 {
                let only_valid: usize = *valid_fields[i].iter().next().unwrap();
                mappings[i] = only_valid;
                for set in &mut valid_fields {
                    set.remove(&only_valid);
                }
                break;
            }
        }
    }

    let count_fields = rules.iter().enumerate().filter_map(|x| 
         if x.1.0.starts_with("departure") {
             Some(x.0)
         } else {
             None
         }
        ).collect::<HashSet<usize>>();

    let mut answer = 1;
    for i in  0..mappings.len() {
        if count_fields.contains(&mappings[i]) {
            answer *= my_ticket[i];
        }
    }

    println!("{:?}", answer);
}
