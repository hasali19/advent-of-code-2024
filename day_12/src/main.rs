#![feature(gen_blocks)]

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};
use pathfinding::prelude::strongly_connected_components;

fn main() -> eyre::Result<()> {
    aoc_solution(12, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let mut cost = 0;

    let nodes = iproduct!(0..grid.width(), 0..grid.height()).collect_vec();
    let regions = strongly_connected_components(&nodes, |&(x, y)| successors(&grid, x, y));

    for region in regions {
        let area = region.len();
        let perimeter = perimeter(&grid, &region);
        cost += area * perimeter;
    }

    println!("{cost}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let mut cost = 0;

    let nodes = iproduct!(0..grid.width(), 0..grid.height()).collect_vec();
    let regions = strongly_connected_components(&nodes, |&(x, y)| successors(&grid, x, y));

    for region in regions {
        let area = region.len();
        let sides = sides(&grid, &region);
        println!("{region:?}");
        println!("{area}");
        println!("{sides}");
        cost += area * sides;
    }

    println!("{cost}");

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

    fn get(&self, x: usize, y: usize) -> char {
        self.0[y][x]
    }

    fn try_get(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize {
            Some(self.get(x as usize, y as usize))
        } else {
            None
        }
    }
}

fn successors(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    gen move {
        let c = grid.get(x, y);

        if x > 0 && grid.get(x - 1, y) == c {
            yield (x - 1, y);
        }

        if x < grid.width() - 1 && grid.get(x + 1, y) == c {
            yield (x + 1, y);
        }

        if y > 0 && grid.get(x, y - 1) == c {
            yield (x, y - 1);
        }

        if y < grid.height() - 1 && grid.get(x, y + 1) == c {
            yield (x, y + 1);
        }
    }
}

fn perimeter(grid: &Grid, plots: &[(usize, usize)]) -> usize {
    let mut perimeter = 0;

    for &(x, y) in plots {
        perimeter += 4 - successors(grid, x, y).count();
    }

    perimeter
}

fn sides(grid: &Grid, plots: &[(usize, usize)]) -> usize {
    let mut count = 0;

    for &(x, y) in plots {
        count += corners(grid, x, y);
    }

    count
}

fn corners(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;

    let corners = [
        ((-1, 0), (0, -1)),
        ((1, 0), (0, -1)),
        ((-1, 0), (0, 1)),
        ((1, 0), (0, 1)),
    ];

    for ((ax, ay), (bx, by)) in corners {
        let c = grid.get(x, y);

        let (x, y) = (x as isize, y as isize);

        let c1 = grid.try_get(x + ax, y + ay);
        let c2 = grid.try_get(x + bx, y + by);
        let c3 = grid.try_get(x + ax, y + by);

        if c1 != Some(c) && c2 != Some(c) || c1 == Some(c) && c2 == Some(c) && c3 != Some(c) {
            count += 1;
        }
    }

    count
}
