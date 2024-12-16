use std::{env, fs};
use std::fs::File;
use std::io::{BufReader, Read};

const INPUT_FILE_NAME: &str = "input.txt";

pub fn read_input_string(day: u32) -> String {
     if let Ok(file) = try_open_file(day) {
          let mut content: String = String::new();
          BufReader::new(file).read_to_string(&mut content).expect("Failed to read input file");
          content.trim().to_string()
     } else if let Ok(content) = try_download_input(day) {
          content.trim().to_string()
     } else {
          panic!("Could not open or download input!");
     }
}

pub fn read_input_lines(day: u32) -> Vec<String> {
     read_input_string(day).split("\n").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
}

pub fn read_input_lines_preserve_blank(day: u32) -> Vec<String> {
     read_input_string(day).split("\n").map(|s| s.to_string()).collect()
}

fn try_open_file(day: u32) -> Result<File, ()> {
     if fs::exists(format!("./{}", INPUT_FILE_NAME)).unwrap_or(false) {
          Ok(File::open(format!("./{}", INPUT_FILE_NAME)).expect("Failed to open input file"))
     } else if fs::exists(format!("./day{:02}/{}", day, INPUT_FILE_NAME)).unwrap_or(false) {
          Ok(File::open(format!("./day{:02}/{}", day, INPUT_FILE_NAME)).expect("Failed to open input file"))
     } else {
          Err(())
     }
}

fn try_download_input(day: u32) -> Result<String, ()> {
     env::var("AOC_TOKEN").map(|token| {
         let client = reqwest::blocking::Client::new();
         client
             .get(format!("https://adventofcode.com/2024/day/{day}/input"))
             .header("Cookie", format!("session={}", token))
             .send()
             .expect("Failed to download input")
             .text()
             .expect("Failed to read downloaded input as string")
     })
         .map_err(|_| ())
}
