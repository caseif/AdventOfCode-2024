use std::cmp::max;
use std::collections::HashMap;
use aoc2024_common::file::read_input_lines;

const NUMPAD_COORDS: &[(i32, i32)] = &[
    (1, 3), // 0
    (0, 2), // ...
    (1, 2),
    (2, 2),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 0),
    (1, 0), // ...
    (2, 0), // 9
    (2, 3), // A
];

const NUMPAD_BLANK_POS: (i32, i32) = (0, 3);
const DIRPAD_BLANK_POS: (i32, i32) = (0, 0);

const PART_1_INDIRECTION: u32 = 2;
const PART_2_INDIRECTION: u32 = 25;

fn main() {
    let codes = read_input_lines(21).into_iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == 'A' { 10 } else { c as usize - '0' as usize })
                .collect::<Vec<_>>()
        })
        .collect();
    println!("Part 1: {}", solve(&codes, PART_1_INDIRECTION));
    println!("Part 2: {}", solve(&codes, PART_2_INDIRECTION));
}

fn solve(codes: &Vec<Vec<usize>>, indirection: u32) -> u64 {
    let mut ans = 0;

    let mut cur_numpad_pos = NUMPAD_COORDS[10];
    let mut dirpad_positions = vec![Movement::Press.get_coord(); indirection as usize];
    let mut cache = HashMap::new();

    for code in codes {
        let mut total = 0;
        for num in code {
            let key_pos = NUMPAD_COORDS[*num];
            let initial_sequence = get_movement_sequence(cur_numpad_pos, key_pos, NUMPAD_BLANK_POS);
            total += get_final_sequence_len(&initial_sequence, &mut dirpad_positions, indirection, &mut cache);
            cur_numpad_pos = key_pos;
        }

        let code_value = (code[0] * 100 + code[1] * 10 + code[2]) as u64;
        ans += total * code_value;
    }

    ans
}

fn get_final_sequence_len(
    sequence: &Vec<Movement>,
    dirpad_positions: &mut [(i32, i32)],
    remaining_levels: u32,
    cache: &mut HashMap<((i32, i32), (i32, i32), u32), u64>
) -> u64 {
    if remaining_levels == 0 {
        return sequence.len() as u64;
    }

    let mut total = 0;
    let mut cur_pos = dirpad_positions[0];
    for dir in sequence {
        let key_pos = dir.get_coord();
        let seq_len = if let Some(&seq_len) = cache.get(&(cur_pos, key_pos, remaining_levels)) {
            seq_len
        } else {
            let next_sequence = get_movement_sequence(cur_pos, key_pos, DIRPAD_BLANK_POS);
            let seq_len = get_final_sequence_len(
                &next_sequence,
                &mut dirpad_positions[1..],
                remaining_levels - 1,
                cache,
            );
            cache.insert((cur_pos, key_pos, remaining_levels), seq_len);
            seq_len
        };
        total += seq_len;
        cur_pos = key_pos;
    }
    dirpad_positions[0] = cur_pos;
    total
}

fn get_movement_sequence(cur_pos: (i32, i32), key_pos: (i32, i32), blank_pos: (i32, i32)) -> Vec<Movement> {
    let left_moves = max(cur_pos.0 - key_pos.0, 0);
    let right_moves = max(key_pos.0 - cur_pos.0, 0);
    let up_moves = max(cur_pos.1 - key_pos.1, 0);
    let down_moves = max(key_pos.1 - cur_pos.1, 0);

    let mut moves = Vec::new();
    // order is important here for the dirpad - ^ and > are closest to A
    for _ in 0..left_moves {
        moves.push(Movement::Left);
    }
    for _ in 0..down_moves {
        moves.push(Movement::Down);
    }
    for _ in 0..up_moves {
        moves.push(Movement::Up);
    }
    for _ in 0..right_moves {
        moves.push(Movement::Right);
    }

    let mut sim_pos = cur_pos;
    let mut must_reverse = false;
    for movement in &moves {
        sim_pos = movement.do_movement(sim_pos);
        if sim_pos == blank_pos {
            must_reverse = true;
            break;
        }
    }
    if must_reverse {
        moves.reverse();
    }

    moves.push(Movement::Press);

    moves
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl Movement {
    fn get_coord(&self) -> (i32, i32) {
        match self {
            Movement::Up => (1, 0),
            Movement::Down => (1, 1),
            Movement::Left => (0, 1),
            Movement::Right => (2, 1),
            Movement::Press => (2, 0),
        }
    }

    fn do_movement(&self, start: (i32, i32)) -> (i32, i32) {
        let delta = match self {
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
            Movement::Press => (0, 0),
        };
        (start.0 + delta.0, start.1 + delta.1)
    }
}
