use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input_lines() -> Vec<String> {
     let file = File::open("./input.txt").expect("Failed to open input file");
     BufReader::new(file).lines()
          .filter(Result::is_ok)
          .map(Result::unwrap)
          .filter(|s| !s.is_empty())
          .collect()
}

pub fn read_input_string() -> String {
     read_input_lines().join("")
}
