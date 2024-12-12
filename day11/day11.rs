use std::collections::{HashMap, VecDeque};
use aoc2024_common::file::read_input_string;

fn main() {
    println!("Part 1: {}", solve(25));
    println!("Part 2: {}", solve(75));
}

fn solve(iterations: u32) -> u64 {
    let orig_vals = read_input_string(11).split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut all_nodes = orig_vals.iter()
        .map(|&val| {
            (val, TreeNode::new(val))
        })
        .collect::<HashMap<u64, TreeNode>>();

    let mut cur_leaves: VecDeque<_> = all_nodes.values()
        .map(|node| (0, node.value))
        .collect();
    while let Some((level, leaf_val)) = cur_leaves.pop_front() {
        if level == iterations {
            continue;
        }

        let (left, right_opt) = if leaf_val == 0 {
            (1, None)
        } else {
            let str_repr = leaf_val.to_string();
            if str_repr.len() % 2 == 0 {
                (
                    str_repr[0..(str_repr.len() / 2)].parse::<u64>().unwrap(),
                    Some(str_repr[(str_repr.len() / 2)..str_repr.len()].parse::<u64>().unwrap()),
                )
            } else {
                (leaf_val * 2024, None)
            }
        };

        if !all_nodes.contains_key(&left) {
            all_nodes.insert(left, TreeNode::new(left));
            all_nodes.get_mut(&leaf_val).unwrap().left = Some(left);
            cur_leaves.push_back((level + 1, left));
        } else {
            all_nodes.get_mut(&leaf_val).unwrap().left = Some(left);
        }

        if let Some(right) = right_opt {
            if !all_nodes.contains_key(&right) {
                all_nodes.insert(right, TreeNode::new(right));
                all_nodes.get_mut(&leaf_val).unwrap().right = Some(right);
                cur_leaves.push_back((level + 1, right));
            } else {
                all_nodes.get_mut(&leaf_val).unwrap().right = Some(right);
            }
        }
    }

    orig_vals.iter().map(|val| all_nodes[val].count_leaves(&all_nodes, iterations)).sum()
}

struct TreeNode {
    value: u64,
    left: Option<u64>,
    right: Option<u64>,
}

impl TreeNode {
    fn new(value: u64) -> Self {
        Self { value, left: None, right: None }
    }

    fn count_leaves(&self, all_nodes: &HashMap<u64, TreeNode>, iterations: u32) -> u64 {
        self.count_leaves_impl(all_nodes, iterations, 0, &mut HashMap::new())
    }

    fn count_leaves_impl(
        &self,
        all_nodes: &HashMap<u64, TreeNode>,
        level_limit: u32,
        cur_level: u32,
        cache: &mut HashMap<(u64, u32), u64>
    ) -> u64 {
        if cur_level == level_limit {
            return 1;
        }

        if let Some(val) = cache.get(&(self.value, cur_level)) {
            return *val;
        }

        match self.left {
            Some(left_val) => {
                let right_count = match self.right {
                    Some(right_val) => {
                        let right_node = &all_nodes[&right_val];
                        right_node.count_leaves_impl(all_nodes, level_limit, cur_level + 1, cache)
                    },
                    None => 0,
                };

                let left_node = &all_nodes[&left_val];
                let left_count = left_node.count_leaves_impl(all_nodes, level_limit, cur_level + 1, cache);

                let total = left_count + right_count;
                cache.insert((self.value, cur_level), total);
                total
            }
            None => 1,
        }
    }
}
