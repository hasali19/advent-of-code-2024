#![feature(let_chains)]

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(9, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut disk_map = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();

    let mut pos = 0;
    let mut checksum = 0usize;
    let mut current_block = (0, 0);

    let mut i = 0;
    while i < disk_map.len() {
        let mut size = disk_map[i];
        if i % 2 == 0 {
            let id = i as u32 / 2;
            for j in 0..size {
                checksum += ((pos + j) * id) as usize;
            }
            pos += size;
        } else {
            while size > 0 {
                if current_block.1 == 0 {
                    let last_block_id = (disk_map.len() as u32 - 1) / 2;
                    let last_block_size = disk_map.pop().unwrap();
                    disk_map.pop();
                    current_block = (last_block_id, last_block_size);
                }

                let copy_size = u32::min(size, current_block.1);
                size -= copy_size;
                for j in 0..copy_size {
                    checksum += ((pos + j) * current_block.0) as usize;
                }
                current_block.1 -= copy_size;
                pos += copy_size;
            }
        }
        i += 1;
    }

    for i in 0..current_block.1 {
        checksum += ((pos + i) * current_block.0) as usize;
    }

    println!("{checksum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut disk_map = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let n = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                Segment::File(i / 2, n)
            } else {
                Segment::Free(n)
            }
        })
        .collect_vec();

    for i in (0..disk_map.len()).rev() {
        for j in 0..i {
            if let Segment::Free(free_size) = disk_map[j]
                && let Segment::File(id, file_size) = disk_map[i]
                && file_size <= free_size
            {
                disk_map[i] = Segment::Free(file_size);
                disk_map[j] = Segment::Free(free_size - file_size);
                disk_map.insert(j, Segment::File(id, file_size));
                break;
            }
        }
    }

    let checksum = checksum(&disk_map);

    println!("{checksum}");

    Ok(())
}

enum Segment {
    Free(usize),
    File(usize, usize),
}

impl Segment {
    fn size(&self) -> usize {
        match self {
            Segment::Free(size) => *size,
            Segment::File(_, size) => *size,
        }
    }
}

fn checksum(disk_map: &[Segment]) -> usize {
    let mut checksum = 0usize;
    let mut pos = 0;
    for segment in disk_map {
        if let &Segment::File(id, size) = segment {
            for i in 0..size {
                checksum += id * (pos + i);
            }
        }
        pos += segment.size();
    }
    checksum
}
