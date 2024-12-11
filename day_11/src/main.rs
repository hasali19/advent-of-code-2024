use std::collections::HashMap;

use aoc2024::aoc_solution;

fn main() -> eyre::Result<()> {
    aoc_solution(11, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = HashMap::new();

    let count: usize = stones
        .iter()
        .map(|stone| stones_after_blinks(*stone, 25, &mut cache))
        .sum();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = HashMap::new();

    let count: usize = stones
        .iter()
        .map(|stone| stones_after_blinks(*stone, 75, &mut cache))
        .sum();

    println!("{count}");

    Ok(())
}

fn stones_after_blinks(stone: u64, blinks: u32, cache: &mut HashMap<(u32, u64), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(n) = cache.get(&(blinks, stone)) {
        return *n;
    }

    let n = if stone == 0 {
        stones_after_blinks(1, blinks - 1, cache)
    } else if let Some((l, r)) = split_digits(stone) {
        stones_after_blinks(l, blinks - 1, cache) + stones_after_blinks(r, blinks - 1, cache)
    } else {
        stones_after_blinks(stone * 2024, blinks - 1, cache)
    };

    cache.insert((blinks, stone), n);

    n
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    let mut digits = 1;
    let mut power = 1;
    while n / power >= 10 {
        digits += 1;
        power *= 10;
    }

    if digits % 2 == 0 {
        let mut l = n;
        let mut r = 0;
        for i in 0..digits / 2 {
            r += (l % 10) * u64::pow(10, i);
            l /= 10;
        }
        Some((l, r))
    } else {
        None
    }
}
