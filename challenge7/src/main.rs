#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::path::Path;
// use std::error::Error;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use std::iter::zip;

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum Type {
    High,
    Pair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

// const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

impl Hand<'_> {
    fn get_type(&self) -> Type {
        let mut card_counts: HashMap<char, usize> = HashMap::new();
        let mut jokers = 0;
        for c in CARDS {
            let count = self.cards.match_indices(c).collect::<Vec<_>>().len();
            if count > 0 {
                if c == 'J' {
                    jokers += count;
                }
                else {
                    card_counts.insert(c, count);
                }
            }
        }
        println!("{:?}", card_counts);
        if jokers == 5 {
            return Type::Five;
        }
        let mut just_counts: Vec<usize> = card_counts.values().cloned().collect();
        just_counts.sort();
        just_counts.reverse();
        just_counts[0] += jokers;
        println!("{:?}", just_counts);


        if just_counts.len() == 1 {
            // let t = Type::Five(self.cards.chars().next().unwrap());
            return Type::Five;
        }
        else if just_counts.len() == 2 {
            for v in just_counts {
                if v == 1 || v == 4 {
                    return Type::Four;
                }
                return Type::House;
            }  
        }
        else if just_counts.len() == 3 {
            for v in just_counts {
                if v == 3 {
                    return Type::Three;
                }
                if v == 2 {
                    return Type::TwoPair;
                }
            }
            panic!("Should not get here!");
        }
        else if just_counts.len() == 4 {
            return Type::Pair;
        }
        return Type::High;
    }

    fn get_score(&self) -> usize {
        let mut score: usize = 0;
        for c in self.cards.chars() {
            score += 14 - CARDS.iter().position(|&c2| c2 == c).unwrap();
            score *= 20;
        }
        return score;
    }
}

fn read_hands(lines: &Vec<String>) -> Vec<Hand> {
    let hands: Vec<Hand> = lines.iter().map (|l| {
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.len() != 2 {
            panic!("should have two parts");
        }
        Hand {cards: parts[0], bid: parts[1].parse().unwrap()}
    })
    .collect();
    println!("{:?}", hands);

    return hands;
}

fn solve_part_one(hands: &Vec<Hand>) {
    let mut hand_scores: Vec<(&Hand, Type, usize)> = hands.iter()
        .map (|hand| {
            (hand, hand.get_type(), hand.get_score())
        })
        .collect();

    hand_scores.sort_by(|a, b| {
        let cmp_second = b.1.cmp(&a.1);
        if cmp_second == std::cmp::Ordering::Equal {
            b.2.cmp(&a.2)
        } else {
            cmp_second
        }
    });
    println!("{:?}", hand_scores);
    
    let mut rank = hands.len();
    let mut total = 0;
    for (hand, _, _) in hand_scores {
        total += hand.bid * rank;
        rank -= 1;
    }
    
    println!("{}", total);
}

fn main() {
    let path = Path::new("big_input.txt");

    let file: File = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let hands = read_hands(&lines);
    solve_part_one(&hands);
}
