use std::fs;

fn check_fields(fields: Vec<&str>) -> bool {
    if fields.len() == 8 {
        return true;
    } else if fields.len() == 7 {
        let mut cid_found = false;
        for field in &fields {
            if field.starts_with("cid") {
                cid_found = true;
                break;
            }
        }
        if !cid_found {
            return true;
        }
    }
    return false;
}

pub fn solve_part1(filename: String) {
    // Finding number of valid passwords
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();
    let mut valid_passports = 0;
    let mut fields: Vec<&str> = vec![];
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            if check_fields(fields.clone()) {
                valid_passports += 1;
            }
            fields.clear();
        } else {
            let data = lines[i].split(" ").collect::<Vec<&str>>();
            for entry in data {
                fields.push(entry);
            }
        }
    }
    if check_fields(fields) {
        valid_passports += 1;
    }
    println!("{}", valid_passports);
}

fn check_valid(fields: Vec<&str>) -> bool {
    let ecls = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for field in &fields {
        if field.starts_with("byr") || field.starts_with("iyr") || field.starts_with("eyr") {
            let year_str = field.split(":").into_iter().skip(1).next().unwrap();
            if year_str.len() != 4 {
                return false;
            }
            let year = year_str.parse::<u64>().unwrap();
            if field.starts_with("byr") {
                if year < 1920 || year > 2002 {
                    return false;
                }
            }
            if field.starts_with("iyr") {
                if year < 2010 || year > 2020 {
                    return false;
                }
            }
            if field.starts_with("eyr") {
                if year < 2020 || year > 2030 {
                    return false;
                }
            }
        } else if field.starts_with("hgt") {
            let mut height = field
                .split(":")
                .into_iter()
                .skip(1)
                .next()
                .unwrap()
                .to_string();
            if height.ends_with("cm") {
                height.pop();
                height.pop();
                let value = height.parse::<u64>().unwrap();
                if value < 150 || value > 193 {
                    return false;
                }
            } else if height.ends_with("in") {
                height.pop();
                height.pop();
                let value = height.parse::<u64>().unwrap();
                if value < 59 || value > 76 {
                    return false;
                }
            } else {
                return false;
            }
        } else if field.starts_with("hcl") {
            let color = field.split(":").into_iter().skip(1).next().unwrap();
            if !color.starts_with("#") {
                return false;
            } else {
                let hex = color.trim_start_matches("#");
                if hex.len() != 6 {
                    return false;
                }
                if let Err(_) = u64::from_str_radix(hex, 16) {
                    return false;
                }
            }
        } else if field.starts_with("ecl") {
            let color = field.split(":").skip(1).next().unwrap();
            let mut color_found = false;
            for ecl in &ecls {
                if *ecl == color {
                    color_found = true;
                    break;
                }
            }
            if !color_found {
                return false;
            }
        } else if field.starts_with("pid") {
            let pid = field.split(":").skip(1).next().unwrap();
            if pid.len() != 9 {
                return false;
            } else if let Err(_) = u64::from_str_radix(pid, 10) {
                return false;
            }
        }
    }

    if fields.len() == 8 {
        return true;
    } else if fields.len() == 7 {
        let mut cid_found = false;
        for field in &fields {
            if field.starts_with("cid") {
                cid_found = true;
                break;
            }
        }
        if !cid_found {
            return true;
        }
    }
    return false;
}

pub fn solve_part2(filename: String) {
    // Finding number of valid passwords
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines: Vec<_> = input_data.lines().collect();
    let mut valid_passports = 0;
    let mut fields: Vec<&str> = vec![];
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            if check_valid(fields.clone()) {
                valid_passports += 1;
            }
            fields.clear();
        } else {
            let data = lines[i].split(" ").collect::<Vec<&str>>();
            for entry in data {
                fields.push(entry);
            }
        }
    }
    if check_valid(fields) {
        valid_passports += 1;
    }
    println!("{}", valid_passports);
}
