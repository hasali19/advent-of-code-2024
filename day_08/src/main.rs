use std::collections::{HashMap, HashSet};

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};

fn main() -> eyre::Result<()> {
    aoc_solution(8, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);
    let antennae = find_antennae(&grid);

    let mut antinodes = HashSet::new();

    for coords in antennae.values() {
        for (&(ax, ay), &(bx, by)) in coords.iter().tuple_combinations() {
            let dx = bx - ax;
            let dy = by - ay;

            let (x, y) = (bx + dx, by + dy);
            if grid.contains(x, y) {
                antinodes.insert((x, y));
            }

            let (x, y) = (ax - dx, ay - dy);
            if grid.contains(x, y) {
                antinodes.insert((x, y));
            }
        }
    }

    println!("{}", antinodes.len());

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);
    let antennae = find_antennae(&grid);

    let mut antinodes = HashSet::new();

    for &(x, y) in antennae.values().flatten() {
        antinodes.insert((x, y));
    }

    for coords in antennae.values() {
        for (&(ax, ay), &(bx, by)) in coords.iter().tuple_combinations() {
            let dx = bx - ax;
            let dy = by - ay;

            let (mut x, mut y) = (bx + dx, by + dy);
            while grid.contains(x, y) {
                antinodes.insert((x, y));
                (x, y) = (x + dx, y + dy);
            }

            let (mut x, mut y) = (ax - dx, ay - dy);
            while grid.contains(x, y) {
                antinodes.insert((x, y));
                (x, y) = (x - dx, y - dy);
            }
        }
    }

    println!("{}", antinodes.len());

    Ok(())
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse(input: &str) -> Grid {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        Grid(grid)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = (char, isize, isize)> {
        iproduct!(0..self.width(), 0..self.height())
            .map(|(x, y)| (self.0[y][x], x as isize, y as isize))
    }

    fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize
    }
}

fn find_antennae(grid: &Grid) -> HashMap<char, HashSet<(isize, isize)>> {
    let mut antennae = HashMap::<char, HashSet<(isize, isize)>>::new();

    for (c, x, y) in grid.iter() {
        if c != '.' {
            antennae.entry(c).or_default().insert((x, y));
        }
    }

    antennae
}
