use aoc2024_common::file::read_input_lines;

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn solve_p1() -> i32 {
    get_safe_levels_count(false)
}

fn solve_p2() -> i32 {
    get_safe_levels_count(true)
}

fn get_safe_levels_count(allow_dampen: bool) -> i32 {
    let lines = read_input_lines();
    let mut count = 0;
    'outer: for (_line_num, line) in lines.iter().enumerate() {
        let nums: Vec<_> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let diffs = compute_diffs(&nums);

        if are_diffs_safe(&diffs) {
            count += 1;
        } else if allow_dampen {
            for i in 0..nums.len() {
                let mut nums_modified = nums.clone();
                nums_modified.remove(i);
                let nums_modified = nums_modified;
                let new_diffs = compute_diffs(&nums_modified);
                if are_diffs_safe(&new_diffs) {
                    count += 1;
                    continue 'outer;
                }
            }
        }
    }
    count
}

fn compute_diffs(nums: &Vec<i32>) -> Vec<i32> {
    nums.iter().zip(nums.iter().skip(1)).map(|(a, b)| b - a).collect()
}

fn are_diffs_safe(diffs: &Vec<i32>) -> bool {
    let all_inside_range = diffs.iter().all(|&n| n >= -3 && n <= 3 && n != 0);
    let all_asc = diffs.iter().all(|n| *n > 0);
    let all_desc = diffs.iter().all(|n| *n < 0);
    all_inside_range && (all_asc || all_desc)
}
