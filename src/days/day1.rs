use std::{
    fs,
    vec::Vec
};

use regex::Regex;

struct Elf {
    food: Vec<i32>,
}

impl Elf {
    fn calories(&self) -> i32 {
        self.food.iter().sum()
    }
}

fn parse_data_file() -> Vec<Elf> {
    let chunk_delim = Regex::new(r"\n{2,}").unwrap();

    let raw_data = fs::read_to_string("data/day1.in").expect("file i/o error");
    let chunks = chunk_delim.split(&raw_data).collect::<Vec<&str>>();
    chunks.iter().map(|c|
        Elf {
            food: c.split("\n").map(|s| s.parse::<i32>().unwrap()).collect(),
        }
    ).collect()
}

pub fn part_one() -> i32 {
    parse_data_file().iter().map(|e| e.calories()).max().unwrap()
}

pub fn part_two() -> i32 {
    let mut all_loads = parse_data_file().iter().map(|e| e.calories()).collect::<Vec<i32>>();
    all_loads.sort();
    all_loads.reverse();

    all_loads[0..3].iter().sum()
}
