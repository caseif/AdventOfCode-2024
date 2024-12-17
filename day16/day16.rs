use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

const MOVE_POINTS: u64 = 1;
const TURN_POINTS: u64 = 1000;

fn main() {
    let maze = parse_input();
    let min_score = solve_p1(&maze);
    println!("Part 1: {}", min_score);
    println!("Part 2: {}", solve_p2(&maze, min_score));
}

fn solve_p1(maze: &MazeDef) -> u64 {
    fn h(cur: ((i32, i32), Direction), goal: (i32, i32)) -> i32 {
        let taxi_distance = (i32::abs_diff(cur.0.0, goal.0) + i32::abs_diff(cur.0.1, goal.1)) as i32;
        let turn_penalty =
            if cur.0.0 < goal.0 && cur.1 == Direction::East
                || cur.0.1 > goal.0 && cur.1 == Direction::South {
            TURN_POINTS as i32
        } else {
            0
        };
        taxi_distance + turn_penalty
    }

    let start = (maze.start, Direction::East);
    let goal = maze.goal;

    let mut came_from = HashMap::new();
    let mut g_score = HashMap::from([(start, 0)]);
    let mut f_score = HashMap::from([(start, h(start, goal))]);
    let mut open_set = BinaryHeap::from([AStarNode { pos: start.0, direction: start.1, f_score: f_score[&start] }]);

    while let Some(cur) = open_set.pop() {
        if cur.pos == goal {
            let mut total_path = HashMap::new();
            let mut total_points = 0u64;
            let mut cur_backtrack_node = (cur.pos, cur.direction);
            while came_from.contains_key(&cur_backtrack_node) {
                let prev_backtrack_node: ((i32, i32), Direction) = came_from[&cur_backtrack_node];
                total_path.insert(prev_backtrack_node.0, prev_backtrack_node.1);
                total_points += if prev_backtrack_node.1 == cur_backtrack_node.1 {
                    MOVE_POINTS
                } else {
                    TURN_POINTS
                };
                cur_backtrack_node = prev_backtrack_node;
            }

            return total_points;
        }
        let turn_dirs = if cur.direction == Direction::North || cur.direction == Direction::South {
            [Direction::East, Direction::West]
        } else {
            [Direction::North, Direction::South]
        };
        let move_delta = match cur.direction {
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
        };
        let neighbors = [
            (((cur.pos.0 + move_delta.0), cur.pos.1 + move_delta.1), cur.direction),
            (cur.pos, turn_dirs[0]),
            (cur.pos, turn_dirs[1]),
        ];

        for neighbor in neighbors {
            if maze.walls.contains(&neighbor.0) {
                continue;
            }

            let weight = if neighbor.1 == cur.direction {
                MOVE_POINTS
            } else {
                TURN_POINTS
            };
            let tent_g_score = g_score[&(cur.pos, cur.direction)] + weight as i32;
            if tent_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, (cur.pos, cur.direction));
                g_score.insert(neighbor, tent_g_score);
                let neighbor_f_score = tent_g_score + h(neighbor, goal);
                f_score.insert(neighbor, neighbor_f_score);
                let neighbor_node = AStarNode { pos: neighbor.0, direction: neighbor.1, f_score: neighbor_f_score };
                if !open_set.iter().contains(&neighbor_node) {
                    open_set.push(neighbor_node);
                }
            }
        }
    }

    panic!("No valid path found");
}

fn solve_p2(maze: &MazeDef, min_score: u64) -> u64 {
    let start_node = (maze.start, Direction::East);
    let mut open_set = VecDeque::from([(start_node, false, 0u64, HashSet::new())]);
    let mut valid_paths = Vec::new();
    let mut best_scores = HashMap::new();

    while let Some(((cur_pos, cur_dir), did_turn, cur_score, prev_nodes)) = open_set.pop_back() {
        if !did_turn && prev_nodes.contains(&cur_pos) {
            continue;
        }
        if cur_score > *best_scores.get(&(cur_pos, cur_dir)).unwrap_or(&u64::MAX) {
            continue;
        }
        best_scores.insert((cur_pos, cur_dir), cur_score);
        best_scores.insert((cur_pos, cur_dir.opposite()), cur_score);

        let mut new_prev_nodes = prev_nodes.clone();
        new_prev_nodes.insert(cur_pos.clone());

        if cur_pos == maze.goal {
            if cur_score == min_score {
                valid_paths.push((new_prev_nodes, cur_score));
            }
            continue;
        }

        let turn_dirs = if cur_dir == Direction::North || cur_dir == Direction::South {
            [Direction::East, Direction::West]
        } else {
            [Direction::North, Direction::South]
        };
        let move_delta = match cur_dir {
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
        };
        let mut neighbors = vec![(((cur_pos.0 + move_delta.0, cur_pos.1 + move_delta.1), cur_dir), MOVE_POINTS)];
        if !did_turn {
            neighbors.push(((cur_pos, turn_dirs[0]), TURN_POINTS));
            neighbors.push(((cur_pos, turn_dirs[1]), TURN_POINTS));
        }

        for (neighbor, neighbor_score) in neighbors {
            if maze.walls.contains(&neighbor.0) {
                continue;
            }
            if cur_score + neighbor_score > min_score {
                continue;
            }
            open_set.push_back((neighbor, neighbor.1 != cur_dir, cur_score + neighbor_score, new_prev_nodes.clone()));
        }
    }

    let best_path_tiles = valid_paths.into_iter()
        .map(|(path, _)| path)
        .flatten()
        .dedup()
        .collect::<HashSet<_>>();

    best_path_tiles.len() as u64
}

fn parse_input() -> MazeDef {
    let lines = read_input_lines(16);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut walls = HashSet::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert((col as i32, row as i32));
            } else if c == 'S' {
                start = (col as i32, row as i32)
            } else if c == 'E' {
                goal = (col as i32, row as i32)
            }
        }
    }
    MazeDef { width, height, walls, start, goal }
}

struct MazeDef {
    #[allow(unused)]
    width: i32,
    #[allow(unused)]
    height: i32,
    walls: HashSet<(i32, i32)>,
    start: (i32, i32),
    goal: (i32, i32),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct AStarNode {
    pos: (i32, i32),
    direction: Direction,
    f_score: i32,
}

impl PartialEq<Self> for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.direction == other.direction
    }
}

impl Eq for AStarNode {}

impl Hash for AStarNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.pos.0);
        state.write_i32(self.pos.1);
        state.write_i32(self.direction as i32);
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
