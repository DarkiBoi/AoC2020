use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn get_input_lines() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").unwrap();
    return BufReader::new(file).lines().map(|line| line.unwrap());
}