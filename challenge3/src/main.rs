use std::fs::File;
use std::path::Path;
// use std::error::Error;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn get_symbols(lines: &Vec<String>) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), Vec<i32>>) {
    let mut locs: HashSet<(i32, i32)> = HashSet::new();
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for (i, line) in lines.into_iter().enumerate() {
        let i = i as i32;
        for (j, c) in line.chars().enumerate() {
            let j = j as i32;
            if ! c.is_digit(10) && c != '.' {
                locs.insert((i, j));
            }
            if c == '*' {
                gears.insert((i, j), vec!());
            }
        }
    }

    return (locs, gears);
}

fn check_if_valid(num_start: (i32, i32), len: i32, symbols: &HashSet<(i32, i32)>) -> bool {
    for i in (num_start.0 - 1)..(num_start.0 + 2) {
        if i < 0 {
            continue;
        }
        for j in (num_start.1 - 1)..(num_start.1 + len + 1) {
            if j < 0 {
                continue;
            }
            if symbols.contains(&(i, j)) {
                return true;
            }
        }
    }
    return false;
}

fn get_numbers(lines: &Vec<String>) -> Vec<(i32, (i32, i32), i32)> {
    let mut numbers: Vec<(i32, (i32, i32), i32)> = vec!(); // number, start, len

    for (i, line) in lines.into_iter().enumerate() {
        println!("{}", line);

        let mut in_num = false;
        let mut num_start: usize = 0;
        for (j, c) in line.chars().enumerate() {
            match (in_num, c.is_digit(10)) {
                (true, true) => continue,
                (true, false) => {
                    let num: i32 = line[num_start..j].parse().unwrap();
                    let loc = (i as i32, num_start as i32);
                    let length = (j - num_start) as i32;
                    numbers.push((num, loc, length));
                    in_num = false;
                },
                (false, true) => {
                    in_num = true;
                    num_start = j;
                },
                (false, false) => continue,
            }
        }

        if in_num {
            // Number appears at the end of the line
            let num: i32 = line[num_start..].parse().unwrap();
            let loc = (i as i32, num_start as i32);
            let length = line.len() as i32;
            numbers.push((num, loc, length));
        }
    }

    return numbers;
}

fn add_part_to_gears(num: i32, num_start: (i32, i32), len: i32, gears: &mut HashMap<(i32, i32), Vec<i32>>) {
    for i in (num_start.0 - 1)..(num_start.0 + 2) {
        if i < 0 {
            continue;
        }
        for j in (num_start.1 - 1)..(num_start.1 + len + 1) {
            if j < 0 {
                continue;
            }
            match gears.get_mut(&(i, j)) {
                Some(parts) => parts.push(num),
                None => (),
            }
        }
    }
}

fn get_gear_ratio(parts: &Vec<i32>) -> i32 {
    if parts.len() != 2 {
        return 0;
    }

    return parts[0] * parts[1];
}

fn main() {
    let path = Path::new("big_input.txt");

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let (symbols, mut gears) = get_symbols(&lines);
    let numbers = get_numbers(&lines);

    let mut part_total = 0;
    for (num, loc, length) in numbers {
        if check_if_valid(loc, length, &symbols) {
            part_total += num;
        }

        add_part_to_gears(num, loc, length, &mut gears)
    }
    println!("part total: {}", part_total);

    println!("{:?}", gears);

    let mut gear_total = 0;
    for (_, parts) in gears.iter() {
        gear_total += get_gear_ratio(&parts);
    }
    println!("gear ratio total {}", gear_total);
}
