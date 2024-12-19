#![feature(let_chains)]

use std::collections::HashMap;

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(19, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (patterns, designs) = parse_input(input);

    let mut cache = HashMap::new();

    let count = designs
        .iter()
        .filter(|design| count_design_possibilities(design, &patterns, &mut cache) > 0)
        .count();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (patterns, designs) = parse_input(input);

    let mut cache = HashMap::new();

    let count = designs
        .iter()
        .map(|design| count_design_possibilities(design, &patterns, &mut cache))
        .sum::<usize>();

    println!("{count}");

    Ok(())
}

fn count_design_possibilities<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&count) = cache.get(design) {
        return count;
    }

    if design.is_empty() {
        return 1;
    }

    let mut count = 0;

    for pattern in patterns {
        if let Some(tail) = design.strip_prefix(pattern) {
            count += count_design_possibilities(tail, patterns, cache);
        }
    }

    cache.insert(design, count);

    count
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect_vec();
    let designs = designs.lines().collect_vec();
    (patterns, designs)
}
