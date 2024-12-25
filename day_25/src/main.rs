use std::iter;

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};

fn main() -> eyre::Result<()> {
    aoc_solution(25, |input| {
        part_1(input)?;
        Ok(())
    })
}

const WIDTH: usize = 5;
const HEIGHT: usize = 7;

fn part_1(input: &str) -> eyre::Result<()> {
    let mut keys = vec![];
    let mut locks = vec![];

    for schematic in input.split("\n\n") {
        let grid = schematic
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        if grid[0].iter().all(|&c| c == '#') {
            locks.push(
                (0..WIDTH)
                    .map(|i| (1..HEIGHT).filter(|&j| grid[j][i] == '#').count())
                    .collect_vec(),
            );
        } else {
            keys.push(
                (0..WIDTH)
                    .map(|i| (0..HEIGHT - 1).filter(|&j| grid[j][i] == '#').count())
                    .collect_vec(),
            );
        }
    }

    let count = iproduct!(locks, keys)
        .filter(|(lock, key)| iter::zip(lock, key).all(|(&l, &k)| k < HEIGHT - l - 1))
        .count();

    println!("{count}");

    Ok(())
}
