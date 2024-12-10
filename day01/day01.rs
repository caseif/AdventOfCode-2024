use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

const DAY: u32 = 1;

fn main() {
    let (list_a, list_b) = parse_lists();
    println!("Part 1: {}", solve_p1(&list_a, &list_b));
    println!("Part 2: {}", solve_p2(&list_a, &list_b));
}

fn parse_lists() -> (Vec<i32>, Vec<i32>) {
    let lines = read_input_lines(DAY);
    let (list_a_unsorted, list_b_unsorted): (Vec<_>, Vec<_>) = lines.iter()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();
    (list_a_unsorted.into_iter().sorted().collect(), list_b_unsorted.into_iter().sorted().collect())
}

fn solve_p1(list_a: &Vec<i32>, list_b: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..list_a.len() {
        sum += i32::abs(list_a[i] - list_b[i]);
    }

    sum
}

fn solve_p2(list_a: &Vec<i32>, list_b: &Vec<i32>) -> i32 {
    let mut score = 0;
    for i in 0..list_a.len() {
        score += list_a[i] * (list_b.iter().filter(|&b| *b == list_a[i]).count() as i32);
    }

    score
}
