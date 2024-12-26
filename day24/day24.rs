use std::collections::HashMap;
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

fn main() {
    let (initial_states, gates) = parse_input();

    println!("Part 1: {}", solve_p1(&initial_states, &gates));
    println!("Part 2: {}", solve_p2(&gates));
}

fn parse_input() -> (HashMap<String, u8>, HashMap<String, LogicGate>) {
    let (initial_lines, logic_lines): (Vec<_>, Vec<_>) = read_input_lines(24).into_iter()
        .partition(|line| line.contains(":"));
    let initial_states = initial_lines.into_iter()
        .map(|line| {
            let spl = line.split_once(':').unwrap();
            (spl.0.to_string(), spl.1.trim().parse::<u8>().unwrap())
        })
        .collect();
    let gates = logic_lines.into_iter()
        .map(|line| {
            let mut spl = line.split_whitespace();
            let in_1 = spl.next().unwrap().to_string();
            let op = match spl.next().unwrap() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("Unknown operation"),
            };
            let in_2 = spl.next().unwrap().to_string();
            let out = spl.skip(1).next().unwrap().to_string();
            (
                out.clone(),
                LogicGate {
                    in_1,
                    in_2,
                    op,
                    out,
                },
            )
        })
        .collect();
    (initial_states, gates)
}

fn solve_p1(initial_states: &HashMap<String, u8>, gates: &HashMap<String, LogicGate>) -> u64 {
    resolve_z_num(initial_states, gates).0
}

fn solve_p2(gates: &HashMap<String, LogicGate>) -> String {
    // honestly f*** this problem, on christmas eve no less...

    let bits = gates.iter().filter(|(k, _)| k.starts_with("z")).collect::<Vec<_>>().len();

    let mut candidates = Vec::new();

    for (name, gate) in gates {
        let are_inputs_xy = (gate.in_1.starts_with("x") && gate.in_2.starts_with("y")) ||
            (gate.in_1.starts_with("y") && gate.in_2.starts_with("x"));
        let are_inputs_lsb = gate.in_1 == "x00" || gate.in_1 == "y00";
        let is_output_z = gate.out.starts_with("z");
        let is_output_msb = gate.out[1..] != (bits - 1).to_string();

        if is_output_z &&
            is_output_msb &&
            gate.op != Operation::Xor {
            candidates.push(name.clone());
        } else if !is_output_z &&
            !are_inputs_xy &&
            gate.op == Operation::Xor {
            candidates.push(name.clone());
        } else if gate.op == Operation::Xor &&
            are_inputs_xy &&
            !are_inputs_lsb &&
            !gates.iter().any(|(_, gate2)| {
                gate2.op == Operation::Xor && (gate2.in_1 == gate.out || gate2.in_2 == gate.out)
            }) {
            candidates.push(name.clone());
        } else if gate.op == Operation::And &&
            !are_inputs_lsb &&
            !gates.iter().any(|(_, gate2)| {
                gate2.op == Operation::Or && (gate2.in_1 == gate.out || gate2.in_2 == gate.out)
            }) {
            candidates.push(name.clone());
        }
    }

    candidates.into_iter().sorted().join(",")
}

fn resolve_z_num(initial_states: &HashMap<String, u8>, gates: &HashMap<String, LogicGate>) -> (u64, u8) {
    let mut i = 0;
    let mut z_num = 0;
    let mut cache = initial_states.clone();
    while let Some(z_gate) = gates.get(&format!("z{:02}", i)) {
        let res = resolve(z_gate, &gates, &mut cache);
        z_num |= (res as u64) << i;
        i += 1;
    }
    (z_num, i)
}

fn resolve(
    gate: &LogicGate,
    all_gates: &HashMap<String, LogicGate>,
    cached_states: &mut HashMap<String, u8>,
) -> u8 {
    if let Some(cached) = cached_states.get(&gate.out) {
        *cached
    } else {
        let in_1 = match cached_states.get(&gate.in_1) {
            Some(val) => *val,
            None => resolve(&all_gates[&gate.in_1], all_gates, cached_states),
        };
        let in_2 = match cached_states.get(&gate.in_2) {
            Some(val) => *val,
            None => resolve(&all_gates[&gate.in_2], all_gates, cached_states),
        };
        let res = match gate.op {
            Operation::And => in_1 & in_2,
            Operation::Or => in_1 | in_2,
            Operation::Xor => in_1 ^ in_2,
        };
        cached_states.insert(gate.out.clone(), res);
        res
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
struct LogicGate {
    in_1: String,
    in_2: String,
    out: String,
    op: Operation,
}


