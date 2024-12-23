#![feature(gen_blocks, let_chains)]

use std::collections::HashMap;

use aoc2024::aoc_solution;
use eyre::bail;
use itertools::{Itertools, iproduct};
use pathfinding::prelude::{bfs_reach, dfs};

fn main() -> eyre::Result<()> {
    aoc_solution(20, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

type Grid = Vec<Vec<char>>;

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let cheats = cheats(&grid, 2)?;

    let count = cheats.filter(|(_, _, d)| *d >= 100).count();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let cheats = cheats(&grid, 20)?;

    let count = cheats.filter(|(_, _, d)| *d >= 100).count();

    println!("{count}");

    Ok(())
}

type Cheat = ((usize, usize), (usize, usize), usize);

/// Finds all cheats in the grid with a given max length.
fn cheats(grid: &Grid, max_len: usize) -> eyre::Result<impl Iterator<Item = Cheat>> {
    let path = find_path(grid)?;

    // Build a map of the distances to the end from each node along the path.
    let dists = path
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    // For each node along the path, find all cheats starting at that node and calculate how much
    // time each cheat will save.
    let cheats = path
        .into_iter()
        .flat_map(move |(ax, ay)| cheats_from(grid, ax, ay, max_len).map(move |b| ((ax, ay), b)))
        .filter_map(move |(a, (b, d))| {
            let a_dist = dists.get(&a).unwrap();
            let b_dist = dists.get(&b).unwrap();
            if *a_dist > *b_dist + d {
                Some((a, b, a_dist - (b_dist + d)))
            } else {
                None
            }
        });

    Ok(cheats)
}

/// Computes the path from the start 'S' to the end 'E' in `grid`.
fn find_path(grid: &Grid) -> eyre::Result<Vec<(usize, usize)>> {
    let width = grid[0].len();
    let height = grid.len();

    let start = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'S')
        .unwrap();

    let end = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'E')
        .unwrap();

    fn successors(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
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

    let Some(path) = dfs(start, |&(x, y)| successors(grid, x, y), |&n| n == end) else {
        bail!("failed to find a path from {start:?} to {end:?}");
    };

    Ok(path)
}

/// Find all cheats with the given `max_len` starting at `(x, y)`.
fn cheats_from(
    grid: &Grid,
    x: usize,
    y: usize,
    max_len: usize,
) -> impl Iterator<Item = ((usize, usize), usize)> {
    let width = grid[0].len();
    let height = grid.len();

    let successors = |&((x, y), d): &((usize, usize), usize)| gen move {
        if d == max_len {
            return;
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                && x < width
                && y < height
            {
                yield ((x, y), d + 1);
            }
        }
    };

    // BFS to find all reachable nodes from (x, y), up to a maximum distance `max_len`.
    bfs_reach(((x, y), 0), successors)
        .filter(|&((x, y), _)| grid[y][x] != '#')
        .into_grouping_map()
        .min() // when there are multiple paths to a node we take the shortest
        .into_iter()
}
