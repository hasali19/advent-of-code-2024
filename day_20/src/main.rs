#![feature(gen_blocks, let_chains)]

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use aoc2024::aoc_solution;
use eyre::bail;
use itertools::{Itertools, iproduct};
use pathfinding::prelude::dfs;

fn main() -> eyre::Result<()> {
    aoc_solution(20, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let start = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'S')
        .unwrap();

    let end = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'E')
        .unwrap();

    fn successors(
        grid: &Vec<Vec<char>>,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let width = grid[0].len();
        let height = grid.len();
        gen move {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    && x < width
                    && y < height
                    && grid[y][x] != '#'
                {
                    yield (x, y);
                }
            }
        }
    }

    let Some(path) = dfs(start, |&(x, y)| successors(&grid, x, y), |&n| n == end) else {
        bail!("failed to find a path from {start:?} to {end:?}");
    };

    let dists = path
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, path.len() - i - 1))
        .collect::<HashMap<_, _>>();

    fn cheats(grid: &Vec<Vec<char>>, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let width = grid[0].len();
        let height = grid.len();
        gen move {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    && x < width
                    && y < height
                    && grid[y][x] == '#'
                    && let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    && x < width
                    && y < height
                    && grid[y][x] != '#'
                {
                    yield (x, y);
                }
            }
        }
    }

    let cheats = path
        .iter()
        .copied()
        .flat_map(|(ax, ay)| cheats(&grid, ax, ay).map(move |b| ((ax, ay), b)))
        .unique()
        .filter_map(|(a, b)| {
            let a_dist = dists.get(&a).unwrap();
            let b_dist = dists.get(&b).unwrap();
            if *a_dist > *b_dist + 2 {
                Some((a, b, a_dist - (b_dist + 2)))
            } else {
                None
            }
        })
        .collect_vec();

    let count = cheats.iter().filter(|(_, _, d)| *d >= 100).count();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    Ok(())
}
