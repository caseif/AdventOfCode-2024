use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input_file() -> Vec<String> {
     let file = File::open("./input.txt").unwrap();
     BufReader::new(file).lines()
          .filter(Result::is_ok)
          .map(Result::unwrap)
          .filter(|s| !s.is_empty())
          .collect()
}
