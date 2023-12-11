#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::path::Path;
// use std::error::Error;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use std::iter::zip;

#[derive(Debug, Copy, Clone)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_winning_moves(&self) -> Vec<usize> {
        let moves: Vec<usize> = (0..self.time).filter(|&t| {
            t * (self.time - t) > self.distance
        }).collect();
        return moves;
    }
}

fn read_races(lines: &Vec<String>) -> Vec<Race> {
    if lines.len() != 2 {
        panic!("Expects 2 lines for input")
    }
    let times: Vec<usize> = lines[0].strip_prefix("Time:").expect("Should be Time: at start")
        .split_whitespace()
        .map (|s| {
            s.parse().expect("expects number")
        })
        .collect();
    let distances: Vec<usize> = lines[1].strip_prefix("Distance:").expect("Should be Distance: at start")
        .split_whitespace()
        .map (|s| {
            s.parse().expect("expects number")
        })
        .collect();

    let races: Vec<Race> = zip(times, distances)
        .map(|(t, d)| {Race {time: t, distance: d}})
        .collect();

    return races
}

fn solve_part_one(races: &Vec<Race>) {
    let mut total = 1;
    for race in races {
        total *= race.get_winning_moves().len();
    }
    println!("{}", total);
}

fn main() {
    let path = Path::new("big_input2.txt");

    let file: File = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let races = read_races(&lines);
    solve_part_one(&races);

}
