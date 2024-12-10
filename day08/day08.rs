use std::collections::{HashMap, HashSet};
use aoc2024_common::file::read_input_lines;
use itertools;
use itertools::Itertools;

fn main() {
    let lines = read_input_lines(8);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    let antennas = get_antennas(&lines);
    println!("Part 1: {}", solve_p1(width, height, &antennas));
    println!("Part 2: {}", solve_p2(width, height, &antennas));
}

fn solve_p1(width: i32, height: i32, antennas: &HashMap<char, Vec<(i32, i32)>>) -> i32 {
    let mut antinodes = HashSet::new();

    for antenna_set in antennas.values() {
        for ((a_x, a_y), (b_x, b_y)) in antenna_set.into_iter().tuple_combinations() {
            let h_sep = b_x - a_x;
            let v_sep = b_y - a_y;

            let n1_x = a_x - h_sep;
            let n1_y = a_y - v_sep;
            let n2_x = b_x + h_sep;
            let n2_y = b_y + v_sep;

            if n1_x >= 0 && n1_x < width && n1_y >= 0 && n1_y < height {
                antinodes.insert((n1_x, n1_y));
            }
            if n2_x >= 0 && n2_x < width && n2_y >= 0 && n2_y < height {
                antinodes.insert((n2_x, n2_y));
            }
        }
    }

    antinodes.len() as i32
}

fn solve_p2(width: i32, height: i32, antennas: &HashMap<char, Vec<(i32, i32)>>) -> i32 {
    let mut antinodes = HashSet::new();

    for antenna_set in antennas.values() {
        for ((a_x, a_y), (b_x, b_y)) in antenna_set.into_iter().tuple_combinations() {
            let h_step = b_x - a_x;
            let v_step = b_y - a_y;

            let mut cur_x = *a_x;
            let mut cur_y = *a_y;
            loop {
                antinodes.insert((cur_x, cur_y));
                cur_x -= h_step;
                cur_y -= v_step;
                if cur_x < 0 || cur_x >= width || cur_y < 0 || cur_y >= height {
                    break;
                }
            }

            let mut cur_x = *b_x;
            let mut cur_y = *b_y;
            loop {
                antinodes.insert((cur_x, cur_y));
                cur_x += h_step;
                cur_y += v_step;
                if cur_x < 0 || cur_x >= width || cur_y < 0 || cur_y >= height {
                    break;
                }
            }
        }
    }

    antinodes.len() as i32
}

fn get_antennas(lines: &Vec<String>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((col as i32, row as i32));
            }
        }
    }

    antennas
}
