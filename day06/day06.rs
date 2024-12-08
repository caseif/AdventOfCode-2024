use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};
use aoc2024_common::file::read_input_lines;

const OBSTACLE_CHAR: char = '#';
const INITIAL_POS_CHAR: char = '^';

fn main() {
    let params = get_world_parameters();
    println!("Part 1: {}", solve_p1(&params));
    println!("Part 2: {}", solve_p2(&params));
}

fn solve_p1(params: &WorldParameters) -> i32 {
    do_simulation(params).expect("Got stuck in loop for part 1??")
}

fn solve_p2(params: &WorldParameters) -> i32 {
    let worker_count = thread::available_parallelism().unwrap();
    let pool = Pool::<ThunkWorker<i32>>::new(worker_count.get());
    let (tx, rx) = mpsc::channel();

    for y in 0..params.height {
        let params_owned = params.clone();
        pool.execute_to(tx.clone(), Thunk::of(move || {
            let mut count = 0;
            for x in 0..params_owned.width {
                if params_owned.obstacles.contains(&(x, y)) {
                    continue;
                }

                let modified_params = WorldParameters {
                    width: params_owned.width,
                    height: params_owned.height,
                    initial_pos: params_owned.initial_pos,
                    obstacles: params_owned.obstacles.iter().chain([&(x, y)]).copied().collect(),
                };

                if do_simulation(&modified_params).is_err() {
                    count += 1;
                }
            }

            count
        }));
    }

    rx.iter().take(params.width as usize).sum()
}

#[derive(Clone, Debug)]
struct WorldParameters {
    width: i32,
    height: i32,
    initial_pos: (i32, i32),
    obstacles: HashSet<(i32, i32)>,
}

fn get_world_parameters() -> WorldParameters {
    let lines = read_input_lines();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut initial_pos = (0, 0);
    let mut obstacles = HashSet::new();
    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == OBSTACLE_CHAR {
                obstacles.insert((col as i32, row as i32));
            } else if c == INITIAL_POS_CHAR {
                initial_pos = (col as i32, row as i32);
            }
        }
    }
    let initial_pos = initial_pos;
    let obstacles = obstacles;

    WorldParameters {
        width,
        height,
        initial_pos,
        obstacles,
    }
}

fn do_simulation(params: &WorldParameters) -> Result<i32, ()> {
    let mut visited = HashSet::from([params.initial_pos]);
    let mut visited_dirs = HashMap::from([(params.initial_pos, Direction::Up)]);
    let mut cur_pos = params.initial_pos;
    let mut cur_dir = Direction::Up;
    while cur_pos.0 >= 0 && cur_pos.0 < params.width &&
        cur_pos.1 >= 0 && cur_pos.1 < params.height {
        (cur_pos, cur_dir) = do_step(params, cur_pos, cur_dir);
        //println!("Moved to {:?}", cur_pos);
        if visited_dirs.get(&cur_pos).map(|&d| d == cur_dir).unwrap_or(false) {
            return Err(())
        }
        visited.insert(cur_pos.clone());
        visited_dirs.insert(cur_pos, cur_dir);
    }

    // subtract 1 for last position since it's out of bounds
    Ok(visited.len() as i32 - 1)
}

fn do_step(params: &WorldParameters, cur_pos: (i32, i32), last_dir: Direction) -> ((i32, i32), Direction) {
    let mut cur_dir = last_dir;
    loop {
        let new_pos = get_next_position(cur_pos, cur_dir);

        if !params.obstacles.contains(&new_pos) {
            return (new_pos, cur_dir);
        }

        cur_dir = get_next_direction(cur_dir);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_next_direction(cur_direction: Direction) -> Direction {
    match cur_direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_next_position(cur_position: (i32, i32), cur_direction: Direction) -> (i32, i32) {
    let delta = match cur_direction {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    };
    (cur_position.0 + delta.0, cur_position.1 + delta.1)
}
