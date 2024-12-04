use regex::Regex;
use aoc2024_common::file::read_input_string;

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn solve_p1() -> i32 {
    let input = read_input_string();
    process_muls(input.as_str())
}

fn solve_p2() -> i32 {
    let do_dont_re = Regex::new(r"(?:^|do\(\))(.*?)(?:$|don't\(\))").unwrap();

    let input = read_input_string();

    let mut total = 0;
        for (_, [inner]) in do_dont_re.captures_iter(&input).map(|c| c.extract()) {
            total += process_muls(inner)
        }

    total
}

fn process_muls(line: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for (_, [a, b]) in mul_re.captures_iter(&line).map(|c| c.extract()) {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }
    total
}
