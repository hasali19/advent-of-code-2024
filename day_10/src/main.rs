#![feature(gen_blocks)]

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};
use pathfinding::prelude::dfs_reach;

fn main() -> eyre::Result<()> {
    aoc_solution(10, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let count: u32 = iproduct!(0..grid.width(), 0..grid.height())
        .filter(|&(x, y)| grid.get(x, y) == 0)
        .map(|(x, y)| count_reachable_ends(&grid, x, y))
        .sum();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let count: u32 = iproduct!(0..grid.width(), 0..grid.height())
        .filter(|&(x, y)| grid.get(x, y) == 0)
        .map(|(x, y)| count_trails(&grid, x, y))
        .sum();

    println!("{count}");

    Ok(())
}

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn parse(input: &str) -> Grid {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();

        Grid(grid)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.0[y][x]
    }
}

fn successors(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    gen move {
        let n = grid.get(x, y);

        if x > 0 && grid.get(x - 1, y) == n + 1 {
            yield (x - 1, y);
        }

        if x < grid.width() - 1 && grid.get(x + 1, y) == n + 1 {
            yield (x + 1, y);
        }

        if y > 0 && grid.get(x, y - 1) == n + 1 {
            yield (x, y - 1);
        }

        if y < grid.height() - 1 && grid.get(x, y + 1) == n + 1 {
            yield (x, y + 1);
        }
    }
}

fn count_reachable_ends(grid: &Grid, x: usize, y: usize) -> u32 {
    let reachable = dfs_reach((x, y), |&(x, y)| successors(grid, x, y));

    let mut count = 0;
    for (x, y) in reachable {
        if grid.get(x, y) == 9 {
            count += 1;
        }
    }

    count
}

fn count_trails(grid: &Grid, x: usize, y: usize) -> u32 {
    if grid.get(x, y) == 9 {
        return 1;
    }

    let mut count = 0;
    for (x, y) in successors(grid, x, y) {
        count += count_trails(grid, x, y);
    }

    count
}
