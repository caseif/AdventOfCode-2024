use std::iter;
use aoc2024_common::file::read_input_lines;

const P1_NEEDLE: &str = "XMAS";
const P1_CTX_RADIUS: i32 = P1_NEEDLE.len() as i32 - 1;
const P2_CTX_RADIUS: i32 = 1;
const PAD_CHAR: char = '.';

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn solve_p1() -> i32 {
    let lines = read_input_lines();

    let mut count = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == P1_NEEDLE.chars().nth(0).unwrap() {
                let context = get_context(&lines, P1_CTX_RADIUS, row as i32, col as i32);
                count += find_word(&context);
            }
        }
    }

    count
}

fn solve_p2() -> i32 {
    let lines = read_input_lines();

    let mut count = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'A' {
                let context = get_context(&lines, P2_CTX_RADIUS, row as i32, col as i32);
                if contains_cross(&context) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn get_context(lines: &Vec<String>, radius: i32, row: i32, col: i32) -> Vec<String> {
    let start_row = i32::max(row - radius, 0);
    let end_row = i32::min(row + radius, lines.len() as i32 - 1);
    let start_col = i32::max(col - radius, 0);
    let end_col = i32::min(col + radius, lines[0].len() as i32 - 1);

    let pad_top = i32::abs_diff(row - radius, start_row) as usize;
    let pad_bottom = i32::abs_diff(row + radius, end_row) as usize;
    let pad_left = i32::abs_diff(col - radius, start_col) as usize;
    let pad_right = i32::abs_diff(col + radius, end_col) as usize;

    let base_context: Vec<_> = lines[start_row as usize..=end_row as usize].iter()
        .map(|line| pad(&line[start_col as usize..=end_col as usize], pad_left, pad_right))
        .collect();

    let pad_line = pad("", radius as usize * 2 + 1, 0);
    vec![
        iter::repeat_n(pad_line.clone(), pad_top).collect(),
        base_context,
        iter::repeat_n(pad_line.clone(), pad_bottom).collect(),
    ]
        .into_iter()
        .flatten()
        .collect()
}

fn pad(s: &str, left: usize, right: usize) -> String {
    iter::repeat_n(PAD_CHAR, left)
        .chain(s.to_string().chars())
        .chain(iter::repeat_n(PAD_CHAR, right).take(right))
        .collect()
}

fn find_word(context: &Vec<String>) -> i32 {
    return
        find_word_in_dir(&context, -1,  0) + // up
        find_word_in_dir(&context, -1,  1) + // up-right
        find_word_in_dir(&context,  0,  1) + // right
        find_word_in_dir(&context,  1,  1) + // down-right
        find_word_in_dir(&context,  1,  0) + // down
        find_word_in_dir(&context,  1, -1) + // down-left
        find_word_in_dir(&context,  0, -1) + // left
        find_word_in_dir(&context, -1, -1);  // up-left
}

fn find_word_in_dir(context: &Vec<String>, step_v: i32, step_h: i32) -> i32 {
    for i in 1..P1_NEEDLE.len() as i32 {
        let needle_char = P1_NEEDLE.chars().nth(i as usize).unwrap();
        let cur_row = (P1_NEEDLE.len() as i32 - 1) + (step_v * i);
        let cur_col = (P1_NEEDLE.len() as i32 - 1) + (step_h * i);
        let actual_char = context[cur_row as usize].chars().nth(cur_col as usize).unwrap();
        if needle_char != actual_char {
            return 0;
        }
    }
    1
}

fn contains_cross(context: &Vec<String>) -> bool {
    let top_left = context[0].chars().nth(0).unwrap();
    let top_right = context[0].chars().nth(2).unwrap();
    let bottom_left = context[2].chars().nth(0).unwrap();
    let bottom_right = context[2].chars().nth(2).unwrap();

    let s1 = [top_left, 'A', bottom_right];
    let s2 = [top_right, 'A', bottom_left];

    (s1 == ['M', 'A', 'S'] || s1 == ['S', 'A', 'M']) && (s2 == ['M', 'A', 'S'] || s2 == ['S', 'A', 'M'])
}
