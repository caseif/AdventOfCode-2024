use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound;
use aoc2024_common::file::read_input_string;
use itertools;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn solve_p1() -> u64 {
    // add phantom free space at end to allow clean division into 2-element steps
    let input = read_input_string(9) + "0";
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
            checksum += (block_index as u64 * *file_id as u64) as u64;
        }
    }

    checksum
}

fn solve_p2() -> u64 {
    // add phantom free space at end to allow clean division into 2-element steps
    let input = read_input_string(9) + "0";
    let mut files = Vec::new();
    let mut free_space = FreeSpace::new();
    let mut cur_block_off = 0;
    for (file_id, (file_run_c, space_run_c)) in input.chars().tuples().enumerate() {
        let file_run = file_run_c as u32 - '0' as u32;
        let space_len = space_run_c as u32 - '0' as u32;
        files.push(FileInfo::new(file_id as u32, cur_block_off, file_run));
        cur_block_off += file_run;
        if space_len > 0 {
            free_space.add(FreeSpaceChunk::new(cur_block_off, space_len));
            cur_block_off += space_len;
        }
    }

    let mut checksum = 0;

    for file in files.into_iter().rev() {
        let final_offset;
        if file.offset < free_space.get_first_free_offset() {
            final_offset = file.offset as u64;
        } else if let Some(new_file_off) = free_space.find_first_free(file.offset, file.len) {
            final_offset = new_file_off as u64;
            free_space.remove(new_file_off, file.len);
        } else {
            final_offset = file.offset as u64;
        }
        for i in 0..file.len {
            checksum += file.id as u64 * (final_offset + i as u64);
        }
    }

    checksum
}

#[derive(Clone, Copy, Debug)]
struct FileInfo {
    id: u32,
    offset: u32,
    len: u32,
}

impl FileInfo {
    fn new(id: u32, offset: u32, len: u32) -> Self {
        Self { id, offset, len }
    }
}

#[derive(Clone, Copy, Debug)]
struct FreeSpaceChunk {
    offset: u32,
    len: u32,
    end_offset: u32,
}

impl PartialEq<Self> for FreeSpaceChunk {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Eq for FreeSpaceChunk {}

impl PartialOrd<Self> for FreeSpaceChunk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for FreeSpaceChunk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl FreeSpaceChunk {
    fn new(offset: u32, len: u32) -> Self {
        Self {
            offset,
            len,
            end_offset: offset + len,
        }
    }
}

#[derive(Debug, Default)]
struct FreeSpace {
    chunks_by_begin_offset: BTreeMap<u32, FreeSpaceChunk>,
    chunks_by_end_offset: HashMap<u32, FreeSpaceChunk>,
    lower_bound_by_len: HashMap<u32, u32>,
    first_free_offset: u32,
}

impl FreeSpace {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, chunk: FreeSpaceChunk) {
        let mut final_offset = chunk.offset;
        let mut final_len = chunk.len;
        if let Some(lead_space) = self.at_end_offset(chunk.offset) {
            final_offset = lead_space.offset;
            final_len += lead_space.len;
        }
        if let Some(&trail_space) = self.at_offset(chunk.offset + chunk.len) {
            final_len += trail_space.len;
        }

        self.chunks_by_begin_offset.insert(final_offset, chunk.clone());
        self.chunks_by_end_offset.insert(final_offset + final_len, chunk.clone());
        self.first_free_offset = *self.chunks_by_begin_offset.iter().nth(0).unwrap().0;
    }

    fn remove(&mut self, offset: u32, len: u32) {
        if let Some(chunk) = self.chunks_by_begin_offset.remove(&offset) {
            assert!(len <= chunk.len);
            if len < chunk.len {
                let new_off = offset + len;
                let new_len = chunk.len - len;
                self.chunks_by_begin_offset.insert(new_off, FreeSpaceChunk::new(new_off, new_len));
                self.chunks_by_end_offset.entry(chunk.end_offset).and_modify(|c| {
                    c.offset = new_off;
                    c.len = new_len;
                });
            } else {
                self.chunks_by_end_offset.remove(&chunk.end_offset);
            }
        }
        self.first_free_offset = *self.chunks_by_begin_offset.iter().nth(0).unwrap().0;
    }

    fn at_offset(&self, offset: u32) -> Option<&FreeSpaceChunk> {
        self.chunks_by_begin_offset.get(&offset)
    }

    fn at_end_offset(&self, end_offset: u32) -> Option<&FreeSpaceChunk> {
        self.chunks_by_end_offset.get(&end_offset)
    }

    fn find_first_free(&mut self, before_offset: u32, len: u32) -> Option<u32> {
        let search_from_key = *self.lower_bound_by_len.get(&len).unwrap_or(&0);
        match self.chunks_by_begin_offset
            .range((Bound::Included(search_from_key), Bound::Unbounded))
            .find_position(|(_, chunk)| chunk.len >= len && chunk.offset < before_offset) {
            Some((chunk_index, (_, chunk))) => {
                if chunk_index > 6 {
                    for _i in len..=chunk.len {
                        *self.lower_bound_by_len.entry(_i).or_default() = chunk.offset;
                    }
                }
                Some(chunk.offset)
            }
            None => None
        }
    }

    fn get_first_free_offset(&self) -> u32 {
        self.first_free_offset
    }
}
