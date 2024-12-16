use std::collections::HashSet;
use aoc2024_common::file::read_input_lines_preserve_blank;

fn main() {
    let (tiles, directions) = parse_input();
    println!("Part 1: {}", solve_p1(&tiles, &directions));
    println!("Part 2: {}", solve_p2(&tiles, &directions));
}

fn solve_p1(tiles: &TileInfo, directions: &Vec<Direction>) -> u64 {
    let mut new_tiles = tiles.clone();
    //print_tiles(&new_tiles);
    for i in 0..directions.len() {
        let dir = directions[i];
        try_move(&mut new_tiles, dir);
    }

    new_tiles.boxes_l.iter().map(|(x, y)| (y * 100 + x) as u64).sum()
}

fn solve_p2(tiles: &TileInfo, directions: &Vec<Direction>) -> u64 {
    let mut transformed_tiles = TileInfo {
        width: tiles.width * 2,
        height: tiles.height,
        cur_pos: (tiles.cur_pos.0 * 2, tiles.cur_pos.1),
        walls: tiles.walls.iter().map(|&(x, y)| vec![(x * 2, y), (x * 2 + 1, y)]).flatten().collect(),
        boxes_l: tiles.boxes_l.iter().map(|&(x, y)| (x * 2, y)).collect(),
        boxes_r: tiles.boxes_l.iter().map(|&(x, y)| (x * 2 + 1, y)).collect(),
    };
    for i in 0..directions.len() {
        let dir = directions[i];
        try_move(&mut transformed_tiles, dir);
    }

    transformed_tiles.boxes_l.iter().map(|(x, y)| (y * 100 + x) as u64).sum()
}

fn try_move(tiles: &mut TileInfo, dir: Direction) {
    let mut box_moves_l = Vec::new();
    let mut box_moves_r = Vec::new();

    let delta = get_direction_delta(dir);

    let mut check_queue = vec![(tiles.cur_pos.0 + delta.0, tiles.cur_pos.1 + delta.1)];
    let mut seen_coords = HashSet::new();

    while let Some((cur_x, cur_y)) = check_queue.pop() {
        if !seen_coords.insert((cur_x, cur_y)) {
            continue;
        }
        if tiles.walls.contains(&(cur_x, cur_y)) {
            // hit wall before any empty spaces
            return;
        } else if tiles.boxes_l.contains(&(cur_x, cur_y)) {
            // stage box to be pushed if we find an empty space
            box_moves_l.push(((cur_x, cur_y), (cur_x + delta.0, cur_y + delta.1)));
            if !tiles.boxes_r.is_empty() {
                check_queue.push((cur_x + 1, cur_y));
            }
        } else if tiles.boxes_r.contains(&(cur_x, cur_y)) {
            // stage box to be pushed if we find an empty space
            box_moves_r.push(((cur_x, cur_y), (cur_x + delta.0, cur_y + delta.1)));
            check_queue.push((cur_x - 1, cur_y));
        } else {
            continue;
        }
        check_queue.push((cur_x + delta.0, cur_y + delta.1));
    }

    // commit moves
    for (old_box, _) in box_moves_l.iter().rev() {
        tiles.boxes_l.remove(old_box);
    }
    for (_, new_box) in box_moves_l.iter().rev() {
        tiles.boxes_l.insert(*new_box);
    }
    for (old_box, _) in box_moves_r.iter().rev() {
        tiles.boxes_r.remove(old_box);
    }
    for (_, new_box) in box_moves_r.iter().rev() {
        tiles.boxes_r.insert(*new_box);
    }

    tiles.cur_pos = (tiles.cur_pos.0 + delta.0, tiles.cur_pos.1 + delta.1);
}

fn parse_input() -> (TileInfo, Vec<Direction>) {
    let lines = read_input_lines_preserve_blank(15);
    let mut it = lines.iter();

    let mut tiles = TileInfo::default();
    tiles.height = lines.iter().take_while(|line| !line.is_empty()).collect::<Vec<_>>().len();
    tiles.width = lines[0].len();

    it.by_ref().take_while(|line| !line.is_empty()).enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            match c {
                '#' => { tiles.walls.insert((col as i64, row as i64)); },
                'O' => { tiles.boxes_l.insert((col as i64, row as i64)); },
                '@' => { tiles.cur_pos = (col as i64, row as i64); },
                _ => {}
            }
        })
    });
    let dirs = it.take_while(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| {
            match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Bad direction char")
            }
        }))
        .flatten()
        .collect();

    (tiles, dirs)
}

#[allow(unused)]
fn print_tiles(tiles: &TileInfo) {
    let box_l_repr = if tiles.boxes_r.is_empty() { "O" } else { "[" };
    for i in 0..tiles.height {
        for j in 0..tiles.width {
            if tiles.boxes_l.contains(&(j as i64, i as i64)) {
                print!("{}", box_l_repr);
            } else if tiles.boxes_r.contains(&(j as i64, i as i64)) {
                print!("]");
            } else if tiles.walls.contains(&(j as i64, i as i64)) {
                print!("#");
            } else if tiles.cur_pos == (j as i64, i as i64) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[derive(Clone, Debug, Default)]
struct TileInfo {
    width: usize,
    height: usize,
    cur_pos: (i64, i64),
    walls: HashSet<(i64, i64)>,
    boxes_l: HashSet<(i64, i64)>,
    boxes_r: HashSet<(i64, i64)>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction_delta(dir: Direction) -> (i64, i64) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}
