#![feature(gen_blocks)]

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};

fn main() -> eyre::Result<()> {
    aoc_solution(4, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let count = grid
        .iter()
        .map(|(x, y, _)| {
            iproduct!(-1..=1, -1..=1)
                .filter(|&(dx, dy)| grid.is_string(x, y, dx, dy, "XMAS".chars()))
                .count()
        })
        .sum::<usize>();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let count = grid
        .iter()
        .filter(|(x, y, _)| {
            grid.is_string_or_rev(x - 1, y - 1, 1, 1, "MAS")
                && grid.is_string_or_rev(x + 1, y - 1, -1, 1, "MAS")
        })
        .count();

    println!("{count}");

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

    fn iter(&self) -> impl Iterator<Item = (isize, isize, char)> {
        gen {
            for (y, row) in self.0.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    yield (x as isize, y as isize, *c)
                }
            }
        }
    }

    fn is_string(
        &self,
        x: isize,
        y: isize,
        dx: isize,
        dy: isize,
        s: impl Iterator<Item = char>,
    ) -> bool {
        s.enumerate()
            .all(|(i, c)| self.is_char(x + dx * i as isize, y + dy * i as isize, c))
    }

    fn is_string_or_rev(&self, x: isize, y: isize, dx: isize, dy: isize, s: &str) -> bool {
        self.is_string(x, y, dx, dy, s.chars()) || self.is_string(x, y, dx, dy, s.chars().rev())
    }

    fn is_char(&self, x: isize, y: isize, c: char) -> bool {
        if x < 0 || y < 0 {
            return false;
        };

        let Some(row) = self.0.get(y as usize) else {
            return false;
        };

        let Some(char) = row.get(x as usize) else {
            return false;
        };

        *char == c
    }
}
