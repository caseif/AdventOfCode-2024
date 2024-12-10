use std::collections::BTreeMap;
use aoc2024_common::file::read_input_string;
use itertools;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn solve_p1() -> i64 {
    // add phantom free space at end to allow clean division into 2-element steps
    let input = read_input_string() + "0";
    let mut map = Vec::new();
    for (file_id, (file_run_c, space_run_c)) in input.chars().tuples().enumerate() {
        map.extend(vec![Some(file_id); file_run_c as usize - '0' as usize]);
        map.extend(vec![None; space_run_c as usize - '0' as usize]);
    }

    let mut prev_first_free = 0;
    for cur_block_index in (0..map.len()).rev() {
        let Some(cur_file_id) = map[cur_block_index] else { continue; };
        let (first_free_with_off, _) = map.iter().skip(prev_first_free).find_position(|b| b.is_none()).unwrap();
        let first_free = first_free_with_off + prev_first_free;
        if first_free > cur_block_index {
            break;
        }
        prev_first_free = first_free;
        map[first_free] = Some(cur_file_id);
        map[cur_block_index] = None;
    }

    let mut checksum = 0;
    for (block_index, file_id_opt) in map.iter().enumerate() {
        if let Some(file_id) = file_id_opt {
            checksum += (block_index * file_id) as i64;
        }
    }

    checksum
}

fn solve_p2() -> i64 {
    // add phantom free space at end to allow clean division into 2-element steps
    let input = read_input_string() + "0";
    let mut files = Vec::new();
    let mut free_spaces = BTreeMap::new();
    let mut cur_block_off = 0;
    for (file_id, (file_run_c, space_run_c)) in input.chars().tuples().enumerate() {
        let file_run = file_run_c as usize - '0' as usize;
        let space_run = space_run_c as usize - '0' as usize;
        files.push((file_id, cur_block_off, file_run));
        cur_block_off += file_run;
        if space_run > 0 {
            free_spaces.insert(cur_block_off, space_run);
            cur_block_off += space_run;
        }
    }

    let mut file_final_positions = Vec::new();
    for (file_id, orig_file_off, file_run) in files.into_iter().rev() {
        if orig_file_off < *free_spaces.iter().nth(0).unwrap().0 {
            file_final_positions.push((file_id, orig_file_off, file_run));
            continue;
        }

        if let Some((&new_file_off, &space_run)) = free_spaces.iter()
            .find(|(&off, &run)| off < orig_file_off && run >= file_run) {
            file_final_positions.push((file_id, new_file_off, file_run));
            free_spaces.remove(&new_file_off);
            if space_run > file_run {
                free_spaces.insert(new_file_off + file_run, space_run - file_run);
            }

            let mut new_space_off = orig_file_off;
            let mut new_space_run = file_run;
            if let Some((&lead_space_off, &lead_space_run)) =
                free_spaces.iter().find(|(&off, &run)| off + run == orig_file_off) {
                new_space_off = lead_space_off;
                new_space_run += lead_space_run;
            }
            if let Some((_, &trail_space_run)) =
                free_spaces.iter().find(|(&off, _)| off == orig_file_off + file_run) {
                new_space_run += trail_space_run;
            }
            free_spaces.insert(new_space_off, new_space_run);
        } else {
            file_final_positions.push((file_id, orig_file_off, file_run));
        }
    }

    let mut checksum: i64 = 0;
    for (file_id, file_off, file_run) in file_final_positions {
        for i in 0..file_run {
            checksum += (file_id * (file_off + i)) as i64;
        }
    }

    checksum
}
