use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

fn main() {
    let nums = read_input_lines(22).into_iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    let (ans_1, ans_2) = solve(&nums);
    println!("Part 1: {}", ans_1);
    println!("Part 2: {}", ans_2);
}

fn solve(nums: &Vec<u64>) -> (u64, u64) {
    let mut p1_sum = 0;

    let mut sequence_vals: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

    for initial_num in nums {
        let mut cur_num = *initial_num;
        let mut secrets = vec![*initial_num];
        let mut changes = Vec::new();
        for _ in 0..2000 {
            let mut new_num = (cur_num ^ (cur_num * 64)) % 16777216;
            new_num = (new_num ^ (new_num / 32)) % 16777216;
            new_num = (new_num ^ (new_num * 2048)) % 16777216;
            secrets.push(new_num);
            changes.push((new_num as i64 % 10) - (cur_num as i64 % 10));
            cur_num = new_num;
        }

        let cur_sequences = changes.into_iter()
            .tuple_windows::<(i64, i64, i64, i64)>()
            .enumerate()
            .unique_by(|(_, seq)| *seq)
            .map(|(i, s)| (s, secrets[i + 4] % 10))
            .collect::<Vec<_>>();
        for (seq, val) in cur_sequences {
            *sequence_vals.entry(seq).or_insert(0) += val;
        }

        p1_sum += cur_num;
    }

    let seq_total_val_map = sequence_vals.into_iter()
        .into_grouping_map_by(|(seq, _)| *seq)
        .fold(0, |acc, _, (_, val)| acc + val);
    let (_p2_seq, p2_count) = seq_total_val_map.into_iter().max_by_key(|(_, val)| *val).unwrap();

    (p1_sum, p2_count)
}
