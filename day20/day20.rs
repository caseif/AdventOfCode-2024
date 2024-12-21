use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

const PART_1_DIST: i32 = 2;
const PART_2_DIST: i32 = 20;

fn main() {
    let maze = parse_input();
    println!("Part 1: {}", solve(&maze, PART_1_DIST));
    println!("Part 2: {}", solve(&maze, PART_2_DIST));
}

fn solve(maze: &MazeDef, cheat_dist: i32) -> u64 {
    let base_path = find_base_path(maze).into_iter().enumerate().map(|(i, p)| (p, i)).collect::<HashMap<_, _>>();

    let mut cheats = Vec::new();

    let deltas = (-cheat_dist..=cheat_dist)
        .map(|x| (-cheat_dist..=cheat_dist).map(|y| (x, y)).collect::<Vec<_>>())
        .flatten()
        .filter(|(x, y)| !(*x == 0 && *y == 0) && x.abs() + y.abs() <= cheat_dist)
        .collect::<Vec<_>>();

    for (src_pos, src_index) in base_path.iter() {
        for (off_x, off_y) in deltas.iter() {
            let dest_pos_signed = (src_pos.0 as i32 + off_x, src_pos.1 as i32 + off_y);
            if dest_pos_signed.0 < 0 ||
                dest_pos_signed.1 < 0 ||
                dest_pos_signed.0 >= maze.width as i32 ||
                dest_pos_signed.1 >= maze.height as i32 {
                continue;
            }
            let dest_pos = (dest_pos_signed.0 as u32, dest_pos_signed.1 as u32);
            let Some(dest_index) = base_path.get(&dest_pos) else { continue; };
            if dest_index < src_index {
                continue;
            }
            let timesave = (dest_index - src_index) as i32 - (off_x.abs() + off_y.abs());
            if timesave > 0 {
                cheats.push((src_pos, dest_pos, timesave, dest_index - src_index, off_x + off_y));
            }
        }
    }

    let counts = cheats.iter().counts_by(|(_, _, timesave, _, _)| *timesave);

    counts.into_iter()
        .filter(|(timesave, _)| *timesave >= 100)
        .map(|(_, count)| count as u64)
        .sum()
}

fn find_base_path(maze: &MazeDef) -> Vec<(u32, u32)> {
    let mut path = vec![maze.start];
    let mut cur_pos = maze.start;
    let mut last_pos = None;
    while cur_pos != maze.end {
        for neighbor in get_neighbors(maze, cur_pos) {
            if last_pos.map(|p| p == neighbor).unwrap_or(false) {
                continue;
            }
            if !maze.barriers.contains(&neighbor) {
                last_pos = Some(cur_pos);
                cur_pos = neighbor;
                path.push(neighbor);
                break;
            }
        }
    }
    path
}

fn parse_input() -> MazeDef {
    let lines = read_input_lines(20);
    let height = lines.len() as u32;
    let width = lines[0].len() as u32;
    let populated_tiles = lines.into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(col, c)| (col as u32, row as u32, c))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let start = populated_tiles.iter()
        .filter_map(|&(x, y, c)| if c == 'S' { Some((x, y)) } else { None })
        .take(1)
        .next()
        .unwrap();
    let end = populated_tiles.iter()
        .filter_map(|&(x, y, c)| if c == 'E' { Some((x, y)) } else { None })
        .take(1)
        .next()
        .unwrap();
    let barriers = populated_tiles.into_iter()
        .filter_map(|(x, y, c)| if c == '#' { Some((x, y)) } else { None })
        .collect();
    MazeDef {
        width,
        height,
        start,
        end,
        barriers,
    }
}

fn get_neighbors(maze: &MazeDef, pos: (u32, u32)) -> Vec<(u32, u32)> {
    let mut neighbors = Vec::with_capacity(4);
    if pos.0 > 0 {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if pos.0 < maze.width - 1 {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 < maze.height - 1 {
        neighbors.push((pos.0, pos.1 + 1));
    }
    neighbors
}

struct MazeDef {
    width: u32,
    height: u32,
    start: (u32, u32),
    end: (u32, u32),
    barriers: HashSet<(u32, u32)>,
}
