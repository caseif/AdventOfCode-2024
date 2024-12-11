use std::collections::{HashMap, HashSet};
use aoc2024_common::file::read_input_lines;

const ADJACENT_STEPS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let (ans_a, ans_b) = solve();
    println!("Part 1: {}", ans_a);
    println!("Part 2: {}", ans_b);
}

fn solve() -> (u32, u32) {
    let lines = read_input_lines(10);

    let topo_map = lines.iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let trailheads = topo_map.iter().enumerate()
        .map(|(row, line)| {
            line.iter().enumerate()
                .filter(|(_, &height)| height == 0)
                .map(move |(col, _)| (col as i32, row as i32))
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut connections: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut trails: HashMap<(i32, i32), HashSet<Vec<(i32, i32)>>> = HashMap::new();
    let mut action_queue = Vec::new();
    action_queue.extend(trailheads.iter().map(|&th| vec![th]));
    while !action_queue.is_empty() {
        let path = action_queue.pop().unwrap();
        let (cur_x, cur_y) = path[path.len() - 1];
        let cur_height = path.len() as u32 - 1;

        if cur_height == 9 {
            connections.entry(path[0]).or_insert_with(|| HashSet::default()).insert((cur_x, cur_y));
            trails.entry(path[0]).or_insert_with(|| HashSet::default()).insert(path.clone());
            continue;
        }

        for (step_x, step_y) in ADJACENT_STEPS {
            let adj_x = cur_x + step_x;
            let adj_y = cur_y + step_y;
            if adj_y < 0 || adj_y > topo_map.len() as i32 - 1 ||
                adj_x < 0 || adj_x > topo_map[0].len() as i32 - 1 {
                continue;
            }
            let adj_height = topo_map[adj_y as usize][adj_x as usize];
            if adj_height == cur_height + 1 {
                action_queue.push(path.clone().into_iter().chain(vec![(adj_x, adj_y)]).collect())
            }
        }
    }

    (
        connections.into_iter().map(|(_, dests)| dests.len() as u32).sum(),
        trails.into_iter().map(|(_, dests)| dests.len() as u32).sum(),
    )
}

fn solve_p2() -> u32 {
    0
}
