use itertools::Itertools;
use regex::Regex;
use aoc2024_common::file::read_input_lines;

const PART_2_OFFSET: u64 = 10000000000000;

fn main() {
    let machines = parse_machines();
    println!("Part 1: {}", solve(&machines, 0));
    println!("Part 2: {}", solve(&machines, PART_2_OFFSET));
}

fn solve(machines: &Vec<MachineDef>, prize_offset: u64) -> u64 {
    let mut total_tokens = 0u64;
    for (_i, machine) in machines.iter().enumerate() {
        if let Some((a_presses, b_presses)) = solve_machine(machine, prize_offset) {
            total_tokens += a_presses * 3 + b_presses;
        }
    }

    total_tokens
}

fn parse_machines() -> Vec<MachineDef> {
    let pattern = Regex::new(r".+: X[+=](\d+), Y[+=](\d+)").unwrap();
    read_input_lines(13)
        .chunks(3)
        .map(|chunk| {
            let tuples: Vec<(i64, i64)> = chunk.into_iter().take(3)
                .map(|line| {
                    pattern.captures(line).unwrap().iter()
                        .skip(1)
                        .map(|n| n.unwrap().as_str().parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect();
            MachineDef {
                a_delta: tuples[0],
                b_delta: tuples[1],
                prize: tuples[2],
            }
        })
        .collect()
}

fn solve_machine(machine: &MachineDef, prize_offset: u64) -> Option<(u64, u64)> {
    let (a, c) = machine.a_delta;
    let (b, d) = machine.b_delta;
    let e = machine.prize.0 + prize_offset as i64;
    let f = machine.prize.1 + prize_offset as i64;

    let det = a * d - b * c;
    if det == 0 {
        panic!("No unique solution!");
    }

    let inv_a = d as f64 / det as f64;
    let inv_b = -b as f64 / det as f64;
    let inv_c = -c as f64 / det as f64;
    let inv_d = a as f64 / det as f64;

    let x = inv_a * e as f64 + inv_b * f as f64;
    let y = inv_c * e as f64 + inv_d * f as f64;

    if x > 0.0 && y > 0.0 &&
        (x.fract() < 0.001 || x.fract() > 0.999) &&
        (y.fract() < 0.001 || y.fract() > 0.999) {
        Some((x.round() as u64, y.round() as u64))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct MachineDef {
    a_delta: (i64, i64),
    b_delta: (i64, i64),
    prize: (i64, i64),
}
