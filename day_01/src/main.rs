use std::collections::HashMap;

use aoc2024::aoc_solution;

fn main() -> eyre::Result<()> {
    aoc_solution(1, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("  ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    let total: u32 = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| i32::abs_diff(a, b))
        .sum();

    println!("{total}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("  ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for v in right {
        *right_counts.entry(v).or_default() += 1;
    }

    let total = left
        .into_iter()
        .map(|a| a * right_counts.get(&a).unwrap_or(&0))
        .sum::<i32>();

    println!("{total}");

    Ok(())
}
