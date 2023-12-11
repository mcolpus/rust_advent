use std::fs::File;
use std::path::Path;
// use std::error::Error;
use std::io::{BufRead, BufReader};

const SPELT_NUMBERS: [(&str, i32); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

fn get_numbers(line: &str) -> Vec<i32> {
    // println!("Line: {}", line);
    let mut numbers: Vec<(i32, i32, i32)> = vec!(); // from, to, value
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            let from: i32 = i.try_into().unwrap();
            let n: i32 = c.to_digit(10).unwrap().try_into().unwrap();
            numbers.push((from, from, n));
        }
    }

    for (word, value) in SPELT_NUMBERS {
        for (index, _) in line.match_indices(word) {
            let from: i32 = index.try_into().unwrap();
            let to: i32 = from + i32::try_from(word.len()).unwrap() - 1;
            numbers.push((from, to, value));
        }
    }

    numbers.sort_by_key(|k| k.0);
    // println!("{:?}", numbers);

    let mut just_numbers: Vec<i32> = vec!();
    let mut prev_end: i32 = -1;
    for (from, to, value) in numbers {
        just_numbers.push(value);
        if from > prev_end {
            prev_end = to;
        }
        else {
            // overlapping so ignore
            println!("overlap in {} \n{} {} {}", line, from, to, value);
        }
    }
    // println!("{:?}", just_numbers);
    return just_numbers
}

fn calibration(numbers: &Vec<i32>) -> i32 {
    return 10 * numbers.first().unwrap() + numbers.last().unwrap()
}

fn main() {
    let path = Path::new("big_input.txt");

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers = get_numbers(&line);
        let calibration = calibration(&numbers);
        println!("Calibration: {}", calibration);
        total += calibration;
    }
    println!("Total: {}", total);
}
