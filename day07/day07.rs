use std::collections::VecDeque;
use aoc2024_common::file::read_input_lines;

fn main() {
    let lines = read_input_lines();
    println!("Part 1: {}", solve_p1(&lines));
    println!("Part 2: {}", solve_p2(&lines));
}

fn solve_p1(lines: &Vec<String>) -> i64 {
    solve(lines, false)
}

fn solve_p2(lines: &Vec<String>) -> i64 {
    solve(lines, true)
}

fn solve(lines: &Vec<String>, allow_concat: bool) -> i64 {
    let mut total: i64 = 0;

    for line in lines {
        let (test_val_str, nums_str) = line.split_once(':').unwrap();
        let test_val = test_val_str.parse::<i64>().unwrap();
        let nums = nums_str.trim().split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
        if check_if_valid(test_val, nums, allow_concat) {
            total += test_val;
        }
    }

    total
}

fn check_if_valid(test_val: i64, nums: Vec<i64>, allow_concat: bool) -> bool {
    let mut action_stack = VecDeque::from([
        (nums[0], 1, Operation::Add),
        (nums[0], 1, Operation::Multiply)
    ]);
    if allow_concat {
        action_stack.push_back((nums[0], 1, Operation::Concat));
    }
    while let Some((cur_val, next_index, next_op)) = action_stack.pop_front() {
        assert!(next_index < nums.len());
        let new_val = match next_op {
            Operation::Add => cur_val + nums[next_index],
            Operation::Multiply => cur_val * nums[next_index],
            Operation::Concat => format!("{}{}", cur_val, nums[next_index]).parse::<i64>().unwrap(),
        };

        if new_val > test_val {
            continue;
        }

        if next_index == nums.len() - 1 {
            if new_val == test_val {
                return true;
            } else {
                continue;
            }
        }

        action_stack.push_back((new_val, next_index + 1, Operation::Add));
        action_stack.push_back((new_val, next_index + 1, Operation::Multiply));
        if allow_concat {
            action_stack.push_back((new_val, next_index + 1, Operation::Concat));
        }
    }

    false
}

enum Operation {
    Add,
    Multiply,
    Concat,
}
