use itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use aoc2024_common::file::read_input_lines;

fn main() {
    println!("Part 1: {}", solve(false));
    println!("Part 2: {}", solve(true));
}

fn solve(combine_sides: bool) -> u64 {
    let grid: Vec<Vec<char>> = read_input_lines(12).iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut cell_area_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut area_perims: HashMap<usize, usize> = HashMap::new();
    let mut area_char_map: HashMap<usize, char> = HashMap::new();
    let mut next_area_id = 0;

    for (base_y, row) in grid.iter().enumerate() {
        for (base_x, &needle) in row.iter().enumerate() {
            if cell_area_map.contains_key(&(base_x, base_y)) {
                continue;
            }

            let cur_area_id = next_area_id;
            next_area_id += 1;

            area_char_map.insert(cur_area_id, needle);

            let mut area_perim = 0;
            let mut visited: HashSet<(usize, usize)> = HashSet::from([(base_x, base_y)]);
            let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(base_x, base_y)]);
            while let Some((x, y)) = queue.pop_front() {
                cell_area_map.insert((x, y), cur_area_id);

                for (off_x, off_y) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                    if (x == 0 && off_x == -1) ||
                        (y == 0 && off_y == -1) ||
                        (x == grid[0].len() - 1 && off_x == 1) ||
                        (y == grid.len() - 1 && off_y == 1) {
                        area_perim += 1;
                        continue;
                    }

                    let adj_x = (x as isize + off_x) as usize;
                    let adj_y = (y as isize + off_y) as usize;

                    if grid[adj_y][adj_x] == needle {
                        if visited.insert((adj_x, adj_y)) {
                            queue.push_back((adj_x, adj_y));
                        }
                    } else {
                        area_perim += 1;
                    }
                }
            }

            if !combine_sides {
                area_perims.insert(cur_area_id, area_perim);
            }
        }
    }

    let areas = cell_area_map.into_iter()
        .map(|(pos, area)| (area, pos))
        .into_grouping_map()
        .collect::<HashSet<_>>();

    if combine_sides {
        for (&id, area) in &areas {
            let mut vertex_count = 0;
            for &(x, y) in area {
                let mut neighbors: HashMap<Direction, bool> = HashMap::new();
                for dir in DIRECTIONS {
                    let (off_x, off_y) = get_direction_offset(dir);
                    if (x == 0 && off_x == -1) ||
                        (y == 0 && off_y == -1) ||
                        (x == grid[0].len() - 1 && off_x == 1) ||
                        (y == grid.len() - 1 && off_y == 1) {
                        neighbors.insert(dir, false);
                    }

                    neighbors.insert(
                        dir,
                        area.contains(&((x as isize + off_x) as usize, (y as isize + off_y) as usize))
                    );
                }

                if neighbors[&Direction::Up] && neighbors[&Direction::Left] && !neighbors[&Direction::UpLeft] {
                    vertex_count += 1;
                } else if !neighbors[&Direction::Up] && !neighbors[&Direction::Left] {
                    vertex_count += 1;
                }
                if neighbors[&Direction::Up] && neighbors[&Direction::Right] && !neighbors[&Direction::UpRight] {
                    vertex_count += 1;
                } else if !neighbors[&Direction::Up] && !neighbors[&Direction::Right] {
                    vertex_count += 1;
                }
                if neighbors[&Direction::Down] && neighbors[&Direction::Left] && !neighbors[&Direction::DownLeft] {
                    vertex_count += 1;
                } else if !neighbors[&Direction::Down] && !neighbors[&Direction::Left] {
                    vertex_count += 1;
                }
                if neighbors[&Direction::Down] && neighbors[&Direction::Right] && !neighbors[&Direction::DownRight] {
                    vertex_count += 1;
                } else if !neighbors[&Direction::Down] && !neighbors[&Direction::Right] {
                    vertex_count += 1;
                }
            }
            area_perims.insert(id, vertex_count);
        }
    }

    let mut total = 0;
    for (id, area) in areas.into_iter().sorted_by_key(|(k, _)| *k) {
        total += area.len() * area_perims[&id];
    }

    total as u64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight
];

fn get_direction_offset(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::UpLeft => (-1, -1),
        Direction::UpRight => (1, -1),
        Direction::DownLeft => (-1, 1),
        Direction::DownRight => (1, 1),
    }
}
