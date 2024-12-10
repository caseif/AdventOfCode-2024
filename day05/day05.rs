use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

const DAY: u32 = 5;

fn main() {
    let lines = read_input_lines(DAY);

    let (rule_lines, seq_lines): (Vec<_>, Vec<_>) = lines.into_iter().partition(|line| line.contains("|"));

    let rules = get_rules(rule_lines);
    let (valid, invalid) = get_valid_invalid_seqs(&rules, seq_lines);

    println!("Part 1: {}", solve_p1(&valid));
    println!("Part 2: {}", solve_p2(&rules, &invalid));
}

fn solve_p1(valid_seqs: &Vec<Vec<i32>>) -> i32 {
    sum_middle_elements(valid_seqs)
}

fn solve_p2(rules: &HashMap<i32, Vec<i32>>, invalid_seqs: &Vec<Vec<i32>>) -> i32 {
    sum_middle_elements(
        &invalid_seqs.iter()
            .map(|seq| seq.iter()
                .copied()
                .sorted_by(|&a, &b| { compare(rules, a, b) })
                .collect::<Vec<_>>()
            )
            .collect()
    )
}

fn get_rules(lines: Vec<String>) -> HashMap<i32, Vec<i32>> {
    let mut rules = HashMap::new();
    for line in lines {
        let (a, b) = line.split_once("|").unwrap();
        rules.entry(a.parse::<i32>().unwrap()).or_insert(Vec::new()).push(b.parse::<i32>().unwrap());
    }
    rules
}

fn get_valid_invalid_seqs(rules: &HashMap<i32, Vec<i32>>, lines: Vec<String>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    lines.into_iter()
        .map(|line| line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .partition(|seq| seq.is_sorted_by(|&a, &b| compare(rules, a, b) != Ordering::Greater))
}

fn compare(rules: &HashMap<i32, Vec<i32>>, a: i32, b: i32) -> Ordering {
    if rules.get(&a).is_some_and(|a_rules| a_rules.contains(&b)) {
        Ordering::Less
    } else if rules.get(&b).is_some_and(|b_rules| b_rules.contains(&a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn sum_middle_elements(seq: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for seq in seq {
        total += seq[seq.len() / 2];
    }

    total
}
