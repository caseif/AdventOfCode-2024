use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

fn main() {
    let input_blocks = read_input_lines(25)
        .into_iter()
        .chunks(7)
        .into_iter()
        .map(|c| c.collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in input_blocks {
        if block[0] == "#####" {
            // lock
            let mut cols = Vec::new();
            for i in 0..5 {
                cols.push(block.iter().find_position(|line| line.chars().nth(i).unwrap() == '.').unwrap().0);
            }
            locks.push(cols);

        } else {
            // key
            let mut cols = Vec::new();
            for i in 0..5 {
                cols.push(7 - block.iter().find_position(|line| line.chars().nth(i).unwrap() == '#').unwrap().0);
            }
            keys.push(cols);
        }
    }

    println!("Solution: {}", solve(&locks, &keys));
}

fn solve(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) -> u64 {
    let mut total = 0;
    for i in 0..locks.len() {
        'outer: for j in 0..keys.len() {
            for k in 0..5 {
                if locks[i][k] + keys[j][k] > 7 {
                    continue 'outer;
                }
            }
            total += 1;
        }
    }
    total
}
