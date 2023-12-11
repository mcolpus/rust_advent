use std::fs::File;
use std::path::Path;
use std::cmp::max;
use std::io::{BufRead, BufReader};

// Make a data object that stores three integers red, blue, green
#[derive(Debug)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,
}

fn parse_game(line: &str) -> (i32, Vec<Round>) {
    // split line by ':'
    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() != 2 {
        panic!("Invalid line: {}", line);
    }

    let game_str = parts[0].trim_start_matches("Game ");
    let game: i32 = game_str.parse().unwrap();

    let rounds_str = parts[1];
    let rounds: Vec<Round> = rounds_str.split(';').map(|round_str| {
        let mut round = Round { red: 0, blue: 0, green: 0 };

        for round_colour in round_str.trim().split(',') {
            if round_colour.contains("red") {
                let red: i32 = round_colour.trim_end_matches("red").trim().parse().unwrap();
                round.red = red;
            } else if round_colour.contains("blue") {
                let blue: i32 = round_colour.trim_end_matches("blue").trim().parse().unwrap();
                round.blue = blue;
            } else if round_colour.contains("green") {
                let green: i32 = round_colour.trim_end_matches("green").trim().parse().unwrap();
                round.green = green;
            } else {
                panic!("Invalid round colour: {}", round_colour);
            }
        }
        round
    }).collect();

    return (game, rounds);
}

fn is_possible(round: &Round) -> bool {
    return round.red <= 12 && round.blue <= 14 && round.green <= 13;
}

fn min_needed(rounds: &Vec<Round>) -> Round {
    let mut min_round = Round { red: 0, blue: 0, green: 0 };
    for round in rounds {
        min_round.red = max(min_round.red, round.red);
        min_round.blue = max(min_round.blue, round.blue);
        min_round.green = max(min_round.green, round.green);
    }
    return min_round;
}

fn main() {
    let path = Path::new("big_input.txt");

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let mut r = Round { red: 0, blue: 0, green: 0 };
    r.red = 1;
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        let (game, rounds) = parse_game(&line);
        let min_round = min_needed(&rounds);
        let power: i32 = min_round.red * min_round.blue * min_round.green;
        total += power;
        println!("Game: {}", game);
        println!("Rounds: {:?}", rounds);
    }
    println!("Total: {}", total);
}
