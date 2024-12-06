use std::collections::HashSet;

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() -> eyre::Result<()> {
    aoc_solution(6, |input| {
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

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let start = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y as usize][x as usize] == '^')
        .unwrap();

    let (mut x, mut y) = start;
    let (mut dx, mut dy) = (0, -1);

    let mut visited = HashSet::new();
    loop {
        visited.insert((x, y));

        let (nx, ny) = (x + dx, y + dy);

        if nx < 0 || nx >= width || ny < 0 || ny >= height {
            break;
        }

        if grid[ny as usize][nx as usize] == '#' {
            (dx, dy) = rotate(dx, dy);
        } else {
            (x, y) = (nx, ny);
        }
    }

    println!("{}", visited.len());

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let start = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y as usize][x as usize] == '^')
        .unwrap();

    let count = iproduct!(0..width, 0..height)
        .par_bridge()
        .filter(|&(bx, by)| {
            let (mut x, mut y) = start;
            let (mut dx, mut dy) = (0, -1);

            let mut visited = HashSet::new();
            loop {
                if visited.contains(&((x, y), (dx, dy))) {
                    break true;
                }

                visited.insert(((x, y), (dx, dy)));

                let (nx, ny) = (x + dx, y + dy);

                if nx < 0 || nx >= width || ny < 0 || ny >= height {
                    break false;
                }

                if grid[ny as usize][nx as usize] == '#' || (nx, ny) == (bx, by) {
                    (dx, dy) = rotate(dx, dy);
                } else {
                    (x, y) = (nx, ny);
                }
            }
        })
        .count();

    println!("{}", count);

    Ok(())
}

fn rotate(dx: isize, dy: isize) -> (isize, isize) {
    match (dx, dy) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}
