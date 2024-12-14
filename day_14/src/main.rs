use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};
use regex::Regex;

fn main() -> eyre::Result<()> {
    aoc_solution(14, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let robots = parse_input(input);

    let width = 101;
    let height = 103;

    let mut grid = vec![vec![0; width]; height];

    for ((mut x, mut y), (dx, dy)) in robots {
        for _ in 0..100 {
            x += dx;
            y += dy;

            x = x.rem_euclid(width as i64);
            y = y.rem_euclid(height as i64);
        }
        grid[y as usize][x as usize] += 1;
    }

    let quadrants = [
        (0, 0),
        (width / 2 + 1, 0),
        (0, height / 2 + 1),
        (width / 2 + 1, height / 2 + 1),
    ];

    let mut safety_factor = 1;

    for (qx, qy) in quadrants {
        safety_factor *= iproduct!(qx..qx + width / 2, qy..qy + height / 2)
            .map(|(x, y)| grid[y][x])
            .sum::<i64>();
    }

    println!("{safety_factor}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut robots = parse_input(input);

    let width = 101;
    let height = 103;

    let mut max_clustering = 0;

    for i in 1.. {
        for ((x, y), (dx, dy)) in &mut robots {
            *x += *dx;
            *y += *dy;

            *x = x.rem_euclid(width);
            *y = y.rem_euclid(height);
        }

        let counts = robots
            .iter()
            .map(|&((x, y), _)| ((x, y), 1))
            .into_grouping_map()
            .sum();

        // Calculate a "clustering" score by counting robots that are within one square of another robot
        let clustering = robots
            .iter()
            .map(|((x, y), _)| {
                iproduct!(-1..=1, -1..=1)
                    .filter(|(dx, dy)| counts.contains_key(&(*x + *dx, *y + *dy)))
                    .count()
            })
            .sum::<usize>();

        if clustering > max_clustering {
            for y in 0..height {
                for x in 0..width {
                    if let Some(n) = counts.get(&(x, y)) {
                        print!("{n}");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }

            println!("{i}");
            println!();

            max_clustering = clustering;
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| -> eyre::Result<_> {
            let re = Regex::new(r"-?\d+")?;
            let (px, py, vx, vy) = re
                .find_iter(line)
                .map(|v| v.as_str().parse())
                .collect_tuple()
                .unwrap();
            Ok(((px?, py?), (vx?, vy?)))
        })
        .try_collect()
        .unwrap()
}
