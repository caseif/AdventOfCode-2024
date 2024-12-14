use std::{fs, thread};
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Mul, Rem};
use std::path::PathBuf;
use std::sync::mpsc;
use aoc2024_common::file::read_input_lines;
use itertools::Itertools;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const PART_1_ITERATIONS: i64 = 100;
const PART_2_ITERATIONS: i64 = 9999; // takes ~45s on my Ryzen 5800X
const PART_2_ITERATIONS_SKIP: i64 = 0;

fn main() {
    let robots = parse_robots();
    println!("Part 1: {}", solve_p1(&robots));
    solve_p2(&robots);
    println!("Part 2: See file explorer :^)");
}

fn solve_p1(robots: &Vec<RobotDef>) -> u64 {
    let mut robot_positions: Vec<Vector2l> = Vec::new();
    for robot in robots {
        let new_pos = (((robot.pos + robot.vel * PART_1_ITERATIONS) % (WIDTH, HEIGHT)) + (WIDTH, HEIGHT)) % (WIDTH, HEIGHT);
        robot_positions.push(new_pos);
    }

    let quads: Vec<_> = robot_positions
        .into_iter()
        .map(|pos| {
            (pos, get_quadrant(WIDTH, HEIGHT, pos))
        })
        .collect();
    let quad_counts: Vec<_> = quads.into_iter()
        .map(|(_, quad)| quad)
        .counts()
        .into_iter()
        .filter(|(quad, _)| *quad != Quadrant::Center)
        .collect();
    quad_counts.into_iter().map(|(_, count)| count as u64).product()
}

fn solve_p2(robots: &Vec<RobotDef>) -> u64 {
    let images_dir_path = PathBuf::from("./part2_images");
    if !images_dir_path.exists() {
        fs::create_dir(&images_dir_path).expect("Failed to create images directory");
    }

    let worker_count = thread::available_parallelism().unwrap();
    let pool = Pool::<ThunkWorker<()>>::new(worker_count.get());
    let (tx, rx) = mpsc::channel();
    for i in PART_2_ITERATIONS_SKIP..=PART_2_ITERATIONS {
        let robots_owned = robots.clone();
        let bmp_file_path = images_dir_path.join(format!("{:06}.bmp", i));
        pool.execute_to(tx.clone(), Thunk::of(move || {
            let mut bmp_file = File::create(&bmp_file_path).expect("Failed to create BMP file");

            let file_size = 32 + WIDTH * HEIGHT;
            let bmp_header_bytes: [u8; 14] = [
                0x42, 0x4D, // magic
                (file_size % 0xFF) as u8,
                ((file_size >> 8) % 0xFF) as u8,
                ((file_size >> 16) % 0xFF) as u8,
                ((file_size >> 24) % 0xFF) as u8,
                0x00, 0x00, // reserved
                0x00, 0x00, // reserved
                0x1A, 0x00, 0x00, 0x00 // pixel array offset
            ];
            let dib_header_bytes: [u8; 12] = [
                0x0C, 0x00, 0x00, 0x00, // DIB header length
                (WIDTH % 0xFF) as u8, ((WIDTH >> 8) % 0xFF) as u8,
                ((-HEIGHT as i16) % 0xFF) as u8, (((-HEIGHT as i16) >> 8) % 0xFF) as u8,
                0x01, 0x00, // color planes count
                0x01, 0x00, // BPP
            ];
            let color_table_bytes: [u8; 6] = [
                0x00, 0x00, 0x00,
                0xFF, 0xFF, 0xFF,
            ];
            bmp_file.write(&bmp_header_bytes).unwrap();
            bmp_file.write(&dib_header_bytes).unwrap();
            bmp_file.write(&color_table_bytes).unwrap();

            const PIXEL_ROW_LEN: usize = ((WIDTH + (32 - (WIDTH % 32))) / 8) as usize;
            const PIXEL_DATA_LEN: usize = PIXEL_ROW_LEN * HEIGHT as usize;
            let mut pixel_data = [0u8; PIXEL_DATA_LEN];

            let mut robot_positions: Vec<Vector2l> = Vec::new();
            for robot in robots_owned {
                let new_pos = (((robot.pos + robot.vel * i) % (WIDTH, HEIGHT)) + (WIDTH, HEIGHT)) % (WIDTH, HEIGHT);
                robot_positions.push(new_pos);
            }

            for i in 0..HEIGHT as usize {
                for j in 0..WIDTH as usize {
                    let byte_offset = i * PIXEL_ROW_LEN + j / 8;
                    let bit_offset = 7 - j as u8 % 8;
                    let has_robot = robot_positions.iter()
                        .any(|&pos| pos == Vector2l { x: j as i64, y: i as i64 });
                    pixel_data[byte_offset] |= (if has_robot { 1 } else { 0 } as u8) << bit_offset;
                }
            }

            bmp_file.write(&pixel_data).unwrap();
            bmp_file.flush().unwrap();
            if i > 0 && i % 100 == 0 {
                println!("Rendered {}/{}", i, PART_2_ITERATIONS - PART_2_ITERATIONS_SKIP + 1);
            }
        }));
    }

    _ = rx.iter().take((PART_2_ITERATIONS - PART_2_ITERATIONS_SKIP + 1) as usize).collect::<Vec<_>>();

    0
}

fn parse_robots() -> Vec<RobotDef> {
    read_input_lines(14).iter()
        .map(|line| {
            line.split(" ")
                .map(|s| {
                    s.split("=")
                        .skip(1)
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|s2| s2.parse::<i64>().unwrap())
                        .collect_tuple::<(i64, i64)>()
                        .map(|(x, y)| Vector2l { x, y })
                        .unwrap()
                })
                .collect_tuple()
                .map(|(pos, vel)| RobotDef { pos, vel })
                .unwrap()
        })
        .collect()
}

fn get_quadrant(width: i64, height: i64, pos: Vector2l) -> Quadrant {
    if pos.x == width / 2 || pos.y == height / 2 {
        return Quadrant::Center;
    }
    let is_left = pos.x < width / 2;
    let is_top = pos.y < height / 2;
    if is_left && is_top {
        Quadrant::TopLeft
    } else if !is_left && is_top {
        Quadrant::TopRight
    } else if is_left && !is_top {
        Quadrant::BottomLeft
    } else {
        Quadrant::BottomRight
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector2l {
    x: i64,
    y: i64,
}

impl Add<Self> for Vector2l {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<(i64, i64)> for Vector2l {
    type Output = Self;
    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Self { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl Mul<i64> for Vector2l {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Rem<(i64, i64)> for Vector2l {
    type Output = Self;
    fn rem(self, rhs: (i64, i64)) -> Self {
        Vector2l { x: self.x % rhs.0, y: self.y % rhs.1 }
    }
}

#[derive(Clone, Copy, Debug)]
struct RobotDef {
    pos: Vector2l,
    vel: Vector2l,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}
