use std::ops::{Add, Mul};

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(7, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let strategies = [Add::add, Mul::mul];

    let total: i64 = parse_input(input)
        .filter(|(test_value, values)| can_produce(*test_value, values, 0, &strategies))
        .map(|(v, _)| v)
        .sum();

    println!("{total}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let strategies = [Add::add, Mul::mul, concat];

    let total: i64 = parse_input(input)
        .filter(|(test_value, values)| can_produce(*test_value, values, 0, &strategies))
        .map(|(v, _)| v)
        .sum();

    println!("{total}");

    Ok(())
}

fn parse_input(input: &str) -> impl Iterator<Item = (i64, Vec<i64>)> {
    input.lines().map(|line| {
        let (test_value, values) = line.split_once(':').unwrap();

        let test_value = test_value.trim().parse::<i64>().unwrap();
        let values: Vec<i64> = values
            .split_whitespace()
            .map(|v| v.parse::<i64>())
            .try_collect()
            .unwrap();

        (test_value, values)
    })
}

fn can_produce(
    test_value: i64,
    values: &[i64],
    acc: i64,
    strategies: &[fn(i64, i64) -> i64],
) -> bool {
    let Some((&head, tail)) = values.split_first() else {
        return test_value == acc;
    };

    strategies
        .iter()
        .any(|s| can_produce(test_value, tail, s(acc, head), strategies))
}

fn concat(a: i64, b: i64) -> i64 {
    a * i64::pow(10, count_digits(b)) + b
}

fn count_digits(a: i64) -> u32 {
    let mut digits = 1;
    let mut power = 1;
    while a / power >= 10 {
        digits += 1;
        power *= 10;
    }
    digits
}
