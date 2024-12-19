use std::cmp::min;
use std::collections::{HashMap, HashSet};
use aoc2024_common::file::read_input_lines;


fn main() {
    let (patterns, towels) = parse_input();
    println!("Part 1: {}", solve_p1(&patterns, &towels));
    println!("Part 2: {}", solve_p2(&patterns, &towels));
}

fn solve_p1(patterns: &HashSet<String>, towels: &Vec<String>) -> u64 {
    let mut possible_count = 0;
    for towel in towels {
        if is_towel_possible(&patterns, towel.as_str()) {
            possible_count += 1;
        }
    }

    possible_count
}

fn solve_p2(patterns: &HashSet<String>, towels: &Vec<String>) -> u64 {
    let mut possible_count = 0;
    let mut cache = HashMap::new();
    for towel in towels {
        possible_count += get_towel_combinations(&patterns, towel.as_str(), &mut cache);
    }

    possible_count
}

fn parse_input() -> (HashSet<String>, Vec<String>) {
    let lines = read_input_lines(19);
    let patterns = lines[0].split(", ").map(|s| s.to_string()).collect::<HashSet<_>>();
    let towels = lines.iter().skip(1).map(|s| s.to_string()).collect::<Vec<_>>();
    (patterns, towels)
}

fn is_towel_possible(
    patterns: &HashSet<String>,
    towel: &str,
) -> bool {
    let pattern_min_len = patterns.iter().map(|p| p.len()).min().unwrap();
    let pattern_max_len = patterns.iter().map(|p| p.len()).max().unwrap();

    let mut cur_leaves: Vec<(&str, Vec<&str>)> = Vec::from([(towel, Vec::new())]);
    while let Some((suffix, cur_path)) = cur_leaves.pop() {
        if suffix.len() == 0 {
            return true;
        }

        for needle_len in pattern_min_len..=min(pattern_max_len, suffix.len()) {
            let needle = &suffix[0..needle_len];
            if patterns.contains(needle) {
                let mut new_path = cur_path.clone();
                new_path.push(needle);
                cur_leaves.push((&suffix[needle_len..], new_path));
            }
        }
    }

    false
}

fn get_towel_combinations(patterns: &HashSet<String>, towel: &str, cache: &mut HashMap<String, u64>) -> u64 {
    if cache.contains_key(towel) {
        return cache[towel];
    }

    let pattern_min_len = patterns.iter().map(|p| p.len()).min().unwrap();
    let pattern_max_len = patterns.iter().map(|p| p.len()).max().unwrap();

    let mut total = 0;

    for needle_len in pattern_min_len..=min(pattern_max_len, towel.len()) {
        if needle_len > towel.len() {
            continue;
        }

        let needle = &towel[0..needle_len];
        if patterns.contains(needle) {
            if towel.len() == needle.len() {
                total += 1;
            } else {
                total += get_towel_combinations(patterns, &towel[needle_len..], cache);
            }
        }
    }

    cache.insert(towel.to_string(), total);

    total
}
