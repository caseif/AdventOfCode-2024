use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

const WIDTH: u32 = 71;
const HEIGHT: u32 = 71;

fn main() {
    let barriers: Vec<(u32, u32)> = read_input_lines(18).into_iter()
        .map(|line| {
            let (x_str, y_str) = line.split_once(",").unwrap();
            (x_str.parse::<u32>().unwrap(), y_str.parse::<u32>().unwrap())
        })
        .collect();
    println!("Part 1: {}", solve_p1(&barriers));
    let ans_2 = solve_p2(&barriers);
    println!("Part 2: {},{}", ans_2.0, ans_2.1);
}

fn solve_p1(barriers: &Vec<(u32, u32)>) -> u64 {
    match find_path(&HashSet::from_iter(barriers.iter().take(1024).copied()), (0, 0)) {
        Some(path) => path.len() as u64,
        None => panic!("No path found"),
    }
}

fn solve_p2(barriers: &Vec<(u32, u32)>) -> (u32, u32) {
    let mut cur_barriers = HashSet::from_iter(barriers.iter().take(1024).copied());
    let mut cur_path = find_path(&cur_barriers, (0, 0)).expect("Could not find initial path");
    for i in 1024..barriers.len() {
        cur_barriers.insert(barriers[i]);
        if let Some(trunc_len) = cur_path.iter().position(|p| p == &barriers[i]) {
            cur_path.truncate(trunc_len);

            match find_path(&cur_barriers, *cur_path.last().unwrap()) {
                Some(new_suffix) => {
                    cur_path.extend(new_suffix);
                }
                None => { return barriers[i]; }
            }
        } else {
            continue;
        }
    }
    panic!("No solution found");
}

fn find_path(barriers: &HashSet<(u32, u32)>, start_pos: (u32, u32)) -> Option<Vec<(u32, u32)>> {
    fn h(cur: (u32, u32), goal: (u32, u32)) -> u32 {
        u32::abs_diff(cur.0, goal.0) + u32::abs_diff(cur.1, goal.1)
    }

    let goal = (WIDTH - 1, HEIGHT - 1);

    let mut came_from = HashMap::new();
    let mut g_score = HashMap::from([(start_pos, 0u32)]);
    let mut f_score = HashMap::from([(start_pos, h(start_pos, goal))]);
    let mut open_set = BinaryHeap::from([AStarNode { pos: start_pos, f_score: f_score[&start_pos] }]);

    while let Some(AStarNode { pos: cur_pos, .. }) = open_set.pop() {
        if cur_pos == goal {
            let mut total_path = Vec::new();
            let mut cur_backtrack_node = cur_pos;
            while came_from.contains_key(&cur_backtrack_node) {
                let prev_pos = came_from[&cur_backtrack_node];
                total_path.push(prev_pos);
                cur_backtrack_node = prev_pos;
            }

            /*for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    if barriers.contains(&(j, i)) {
                        print!("#");
                    } else if total_path.contains(&(j, i)) {
                        print!("O");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }*/

            total_path.reverse();
            return Some(total_path);
        }
        for move_delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if move_delta.0 == -1 && cur_pos.0 == 0 ||
                move_delta.0 == 1 && cur_pos.0 == WIDTH - 1 ||
                move_delta.1 == -1 && cur_pos.1 == 0 ||
                move_delta.1 == 1 && cur_pos.1 == WIDTH - 1 {
                continue;
            }

            let neighbor = ((cur_pos.0 as i32 + move_delta.0) as u32, (cur_pos.1 as i32 + move_delta.1) as u32);

            if barriers.contains(&neighbor) {
                continue;
            }

            let tent_g_score = g_score[&cur_pos] + 1;
            if tent_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, cur_pos);
                g_score.insert(neighbor, tent_g_score);
                let neighbor_f_score = tent_g_score + h(neighbor, goal);
                f_score.insert(neighbor, neighbor_f_score);
                let neighbor_node = AStarNode { pos: neighbor, f_score: neighbor_f_score };
                if !open_set.iter().contains(&neighbor_node) {
                    open_set.push(neighbor_node);
                }
            }
        }
    }

    None
}

#[derive(Clone, Copy, Debug)]
struct AStarNode {
    pos: (u32, u32),
    f_score: u32,
}

impl PartialEq<Self> for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for AStarNode {}

impl Hash for AStarNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.pos.0);
        state.write_u32(self.pos.1);
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}
