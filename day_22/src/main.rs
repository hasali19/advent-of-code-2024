#![feature(gen_blocks)]

use std::collections::{HashMap, HashSet};
use std::iter;

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(22, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut sum = 0;

    for n in input.lines() {
        sum += secret_numbers(n.parse()?).take(2000).last().unwrap();
    }

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut counts = HashMap::new();

    for n in input.lines() {
        let n = n.parse()?;

        let prices = iter::once(n)
            .chain(secret_numbers(n).take(2000))
            .map(|n| n % 10)
            .collect_vec();

        let mut sequences = HashSet::new();
        for (i, (a, b, c, d)) in price_changes(&prices).tuple_windows().enumerate() {
            if sequences.insert((a, b, c, d)) {
                *counts.entry((a, b, c, d)).or_insert(0) += prices[i + 4];
            }
        }
    }

    let max = counts.values().copied().max();

    println!("{:?}", max);

    Ok(())
}

fn secret_numbers(mut n: usize) -> impl Iterator<Item = usize> {
    gen move {
        loop {
            n = next(n);
            yield n;
        }
    }
}

fn price_changes(prices: &[usize]) -> impl Iterator<Item = isize> {
    prices
        .iter()
        .copied()
        .tuple_windows()
        .map(|(a, b)| b as isize - a as isize)
}

fn next(n: usize) -> usize {
    let n = mix_and_prune(n, n * 64);
    let n = mix_and_prune(n, n / 32);

    mix_and_prune(n, n * 2048)
}

fn mix_and_prune(s: usize, n: usize) -> usize {
    (s ^ n) % 16777216
}
