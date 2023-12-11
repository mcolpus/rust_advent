use std::fs::File;
use std::path::Path;
// use std::error::Error;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};


fn get_numbers(line: &str) -> (HashSet<i32>, HashSet<i32>){
    let number_part: &str = line.split(':').collect::<Vec<&str>>()[1];
    println!("{}", number_part);
    let parts: Vec<&str> = number_part.split('|').collect();
    if parts.len() != 2 {
        panic!("Parts {:?} should have length 2", parts);
    }
    let targets: HashSet<i32> = parts[0].trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
    println!("{:?}", targets);
    let my_numbers: HashSet<i32> = parts[1].trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
    println!("{:?}", my_numbers);

    return (targets, my_numbers);
}

fn get_matches(targets: &HashSet<i32>, my_numbers: &HashSet<i32>) -> u32{
    let intersection: HashSet<_> = targets.intersection(&my_numbers).collect();
    println!("{:?}", intersection);
    return intersection.len() as u32;
}

fn get_points(lines: &Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for line in lines {
        let (targets, my_numbers) = get_numbers(&line);
        let matches = get_matches(&targets, &my_numbers);
        if matches > 0 {
            total += u32::pow(2, matches - 1);
        }
    }
    return total;
}

fn count_cards(lines: &Vec<String>) -> u32 {
    let mut card_total: u32 = 0;
    let mut card_copies: HashMap<u32, u32> = HashMap::new();

    for (game, line) in lines.iter().enumerate() {
        let game = game as u32;
        println!("game: {} -- {}", game, line);
        let copies = match card_copies.get(&game) {
            Some(&number) => number + 1,
            _ => 1,
        };
        card_total += copies;
        
        let (targets, my_numbers) = get_numbers(line);
        let matches = get_matches(&targets, &my_numbers);
        for card in (game+1)..(game + matches + 1) {
            match card_copies.get(&card) {
                Some(&number) => card_copies.insert(card, number + copies),
                _ => card_copies.insert(card, copies),
            };
        }
    }

    return card_total;
}

fn main() {
    let path = Path::new("big_input.txt");

    let file: File = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("{}", get_points(&lines));
    println!("{}", count_cards(&lines))
}
