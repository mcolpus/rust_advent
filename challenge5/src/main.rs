#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::path::Path;
// use std::error::Error;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};

#[derive(Debug)]
struct MapUnit {
    target_start: usize,
    source_start: usize,
    count: usize
}

impl MapUnit {
    fn target_end(&self) -> usize {
        return self.target_start + self.count - 1;
    }

    fn source_end(&self) -> usize {
        return self.source_start + self.count - 1;
    }
}

#[derive(Debug)]
struct Mapping {
    map_units: Vec<MapUnit>
}

impl Mapping {
    fn map_value(&self, source: usize) -> usize {
        for m in &self.map_units {
            if source >= m.source_start && source < (m.source_start + m.count) {
                // This is the map unit to use
                return m.target_start + (source - m.source_start);
            }
        }

        return source;
    }

    fn map_range(&self, source_range: Range) -> Vec<Range> {
        let mut results: Vec<Range> = Vec::<Range>::new(); 
        let mut ranges_to_map: Vec<Range> = vec!(source_range);
        println!("{:?}", ranges_to_map);
        for m in &self.map_units {
            let mut keep_mapping: Vec<Range> = Vec::<Range>::new();
            println!("map: {:?}", m);
            
            for range in ranges_to_map {
                let overlap_start = max(range.start, m.source_start);
                let overlap_end = min(range.end(), m.source_end());
                println!("{} to {}", overlap_start, overlap_end);
                if overlap_start > overlap_end {
                    keep_mapping.push(range);
                    continue;
                }
                
                if range.start < overlap_start {
                    keep_mapping.push(
                        Range { start: range.start, count: overlap_start - range.start }
                    );
                }
                results.push(
                    Range { start: (overlap_start - m.source_start) + m.target_start, count: overlap_end - overlap_start + 1 }
                );
                if overlap_end < range.end() {
                    keep_mapping.push(
                        Range { start: overlap_end + 1, count: range.end() - overlap_end }
                    );
                }
                
            }
            
            ranges_to_map = keep_mapping;
            println!("{:?}", ranges_to_map);
        }
        
        results.append(&mut ranges_to_map);
        println!("results: {:?}", results);
        return results;
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    count: usize
}

impl Range {
    fn end(&self) -> usize {
        return self.start + self.count - 1;
    }
}

fn create_mapping(lines: &[String]) -> Mapping {
    let mut map_units: Vec<MapUnit> = vec!();
    for line in lines {
        if line.contains("map") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            panic!("Parts {:?} should have 3 parts", parts);
        }
        let unit = MapUnit {
            target_start: parts[0].parse().expect("Part 1 should be number"),
            source_start: parts[1].parse().expect("Part 2 should be number"),
            count: parts[2].parse().expect("Part 3 should be number"),
        };
        map_units.push(unit);
    }

    let mapping = Mapping { map_units: map_units };
    return mapping;
}

fn parse_seeds(line: &str) -> Vec<usize> {
    let l = line.strip_prefix("seeds: ").expect("This is not the seeds line");
    let seeds: Vec<usize> = l.split_whitespace()
        .map(|word| { word.parse().expect("each word should be a number")})
        .collect();
    return seeds;
}

fn parse_seeds_as_range(line: &str) -> Vec<Range> {
    let l = line.strip_prefix("seeds: ").expect("This is not the seeds line");
    let ranges: Vec<Range> = l.split_whitespace()
        .map(|word| { word.parse().expect("each word should be a number")})
        .collect::<Vec<usize>>()
        .chunks(2)
        .filter(|chunk| chunk.len() == 2)
        .map(|chunk| Range{start: chunk[0], count: chunk[1]})
        .collect();
    return ranges;
}

fn run_first_part(lines: &Vec<String>) {
    let mut seeds: Vec<usize> = vec!();
    let mut current_map = "none";
    let mut current_start_line : usize = 0;
    let mut maps: HashMap<&str, Mapping> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        if line.contains("seeds") {
            seeds = parse_seeds(&line);
            continue;
        }

        if line.contains("map:") {
            if current_map != "none" {
                let map_lines: &[String] = &lines[current_start_line..(index-1)];
                let mapping: Mapping = create_mapping(map_lines);
                maps.insert(current_map, mapping);
            }
            current_map = line.strip_suffix("map:").unwrap().trim();
            current_start_line = index + 1;
        }
    }

    // Catch final input
    if current_map != "none" {
        let map_lines: &[String] = &lines[current_start_line..];
        let mapping: Mapping = create_mapping(map_lines);
        maps.insert(current_map, mapping);
    }

    println!("{:?}", seeds);

    let map_names = ["seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location"];

    let mut values = seeds;

    for map_name in map_names {
        println!("applying {}", map_name);
        let map = maps.get(map_name).expect("map missing");
        values = values.iter().map(|v| {
            map.map_value(*v)
        }).collect();
        println!("new values {:?}", values);
    }

    println!("{}", values.iter().min().unwrap());
}

fn run_second_part(lines: &Vec<String>) {
    let mut seeds: Vec<Range> = vec!();
    let mut current_map = "none";
    let mut current_start_line : usize = 0;
    let mut maps: HashMap<&str, Mapping> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        if line.contains("seeds") {
            seeds = parse_seeds_as_range(&line);
            continue;
        }

        if line.contains("map:") {
            if current_map != "none" {
                let map_lines: &[String] = &lines[current_start_line..(index-1)];
                let mapping: Mapping = create_mapping(map_lines);
                maps.insert(current_map, mapping);
            }
            current_map = line.strip_suffix("map:").unwrap().trim();
            current_start_line = index + 1;
        }
    }

    // Catch final input
    if current_map != "none" {
        let map_lines: &[String] = &lines[current_start_line..];
        let mapping: Mapping = create_mapping(map_lines);
        maps.insert(current_map, mapping);
    }

    println!("{:?}", seeds);
    println!("{:?}", maps);

    let map_names = ["seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location"];

    let mut ranges = seeds;

    for map_name in map_names {
        println!("applying {}", map_name);
        let map = maps.get(map_name).expect("map missing");
        ranges = ranges.iter().map(|range| {
            map.map_range(*range)
        }).flatten().collect();
        println!("new values {:?}", ranges);
    }

    println!("{:?}", ranges);
    let mut min_num = ranges[0].start;
    for r in ranges {
        min_num = min(min_num, r.start);
    }
    println!("{}", min_num);
}

fn main() {
    let path = Path::new("big_input.txt");

    let file: File = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    run_second_part(&lines);



}
