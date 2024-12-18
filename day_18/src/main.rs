#![feature(gen_blocks, let_chains)]

use std::collections::HashSet;

use aoc2024::aoc_solution;
use eyre::OptionExt;
use itertools::Itertools;
use pathfinding::prelude::{dfs_reach, dijkstra};

fn main() -> eyre::Result<()> {
    aoc_solution(18, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

fn part_1(input: &str) -> eyre::Result<()> {
    let bytes = parse_input(input)?;

    fn successors(
        x: usize,
        y: usize,
        fallen_bytes: &HashSet<(usize, usize)>,
    ) -> impl Iterator<Item = ((usize, usize), u32)> {
        gen move {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    && x < WIDTH
                    && y < HEIGHT
                    && !fallen_bytes.contains(&(x, y))
                {
                    yield ((x, y), 1);
                }
            }
        }
    }

    let kilobyte = bytes[..1024].iter().copied().collect();
    let res = dijkstra(
        &(0usize, 0usize),
        |&(x, y)| successors(x, y, &kilobyte),
        |&(x, y)| (x, y) == (WIDTH - 1, HEIGHT - 1),
    );

    let (_, cost) = res.unwrap();

    println!("{cost}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let bytes = parse_input(input)?;

    fn successors(
        x: usize,
        y: usize,
        fallen_bytes: &HashSet<(usize, usize)>,
    ) -> impl Iterator<Item = (usize, usize)> {
        gen move {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    && x < WIDTH
                    && y < HEIGHT
                    && !fallen_bytes.contains(&(x, y))
                {
                    yield (x, y);
                }
            }
        }
    }

    let mut first = None;
    let mut fallen_bytes = HashSet::new();
    for byte in bytes {
        fallen_bytes.insert(byte);

        let mut reachable = dfs_reach((0, 0), |&(x, y)| successors(x, y, &fallen_bytes));
        if !reachable.any(|(x, y)| (x, y) == (70, 70)) {
            first = Some(byte);
            break;
        }
    }

    println!("{first:?}");

    Ok(())
}

fn parse_input(input: &str) -> eyre::Result<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').ok_or_eyre("invalid line")?;
            Ok((x.parse()?, y.parse()?))
        })
        .try_collect()
}
