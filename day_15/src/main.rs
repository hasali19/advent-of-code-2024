#![feature(gen_blocks, strict_overflow_ops)]

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};
use linked_hash_set::LinkedHashSet;

fn main() -> eyre::Result<()> {
    aoc_solution(15, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let mut grid = map
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let (mut x, mut y) = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == '@')
        .unwrap();

    for movement in movements.chars().filter(|c| !c.is_whitespace()) {
        let (dx, dy) = direction(movement);
        (x, y) = try_move(&mut grid, x, y, dx, dy);
    }

    let sum = iproduct!(0..width, 0..height)
        .map(|(x, y)| if grid[y][x] == 'O' { 100 * y + x } else { 0 })
        .sum::<usize>();

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = map
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let (mut x, mut y) = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == '@')
        .unwrap();

    for movement in movements.chars().filter(|c| !c.is_whitespace()) {
        let (dx, dy) = direction(movement);
        (x, y) = try_move(&mut grid, x, y, dx, dy);
    }

    let sum = iproduct!(0..width, 0..height)
        .map(|(x, y)| if grid[y][x] == '[' { 100 * y + x } else { 0 })
        .sum::<usize>();

    println!("{sum}");

    Ok(())
}

fn direction(movement: char) -> (isize, isize) {
    match movement {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => unreachable!(),
    }
}

fn try_move(grid: &mut Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize) {
    let mut obstacles = LinkedHashSet::new();
    if can_move(grid, x, y, dx, dy, &mut obstacles) {
        for (x, y) in obstacles.into_iter().chain([(x, y)]) {
            let (tx, ty) = (x.strict_add_signed(dx), y.strict_add_signed(dy));
            grid[ty][tx] = grid[y][x];
            grid[y][x] = '.';
        }
        (x.strict_add_signed(dx), y.strict_add_signed(dy))
    } else {
        (x, y)
    }
}

/// Returns true iff an object at (x, y) can be moved to (x + dx, y + dy), collecting any obstacles
/// in the path into `obstacles`.
fn can_move(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    obstacles: &mut LinkedHashSet<(usize, usize)>,
) -> bool {
    let (Some(tx), Some(ty)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) else {
        return false;
    };

    if ty >= grid.len() || tx >= grid[0].len() {
        return false;
    }

    let target = grid[ty][tx];

    if target == '#' {
        return false;
    }

    if target == '.' {
        return true;
    }

    if target == '[' && dx == 0 {
        return if can_move(grid, tx, ty, dx, dy, obstacles)
            && can_move(grid, tx + 1, ty, dx, dy, obstacles)
        {
            obstacles.insert_if_absent((tx, ty));
            obstacles.insert_if_absent((tx + 1, ty));
            true
        } else {
            false
        };
    }

    if target == ']' && dx == 0 {
        return if can_move(grid, tx, ty, dx, dy, obstacles)
            && can_move(grid, tx - 1, ty, dx, dy, obstacles)
        {
            obstacles.insert_if_absent((tx, ty));
            obstacles.insert_if_absent((tx - 1, ty));
            true
        } else {
            false
        };
    }

    if can_move(grid, tx, ty, dx, dy, obstacles) {
        obstacles.insert_if_absent((tx, ty));
        return true;
    }

    false
}
