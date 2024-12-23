use std::collections::{HashMap, HashSet};

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(23, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let edges = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect_vec();

    let mut graph = HashMap::new();
    for (a, b) in edges {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let count = graph
        .keys()
        .tuple_combinations()
        .filter(|&(&a, &b, &c)| {
            graph[a].contains(b)
                && graph[a].contains(c)
                && graph[b].contains(c)
                && [a, b, c].iter().any(|it| it.starts_with('t'))
        })
        .count();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let edges = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect::<HashSet<_>>();

    let mut graph = HashMap::new();
    for &(a, b) in &edges {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut max_network = graph
        .keys()
        .map(|&a| {
            let mut network = vec![a];

            for &b in graph.keys() {
                if network
                    .iter()
                    .all(|&c| edges.contains(&(b, c)) || edges.contains(&(c, b)))
                {
                    network.push(b);
                }
            }

            network
        })
        .max_by_key(|n| n.len())
        .unwrap();

    max_network.sort();

    let password = max_network.join(",");

    println!("{password}");

    Ok(())
}
