#![feature(let_chains)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(5, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (rules, updates) = parse_input(input);

    let mut sum = 0;

    for update in &updates {
        if !is_ordered(update, &rules) {
            continue;
        }

        let mid = update.len() / 2;
        let mid = update[mid];

        sum += mid;
    }

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (rules, mut updates) = parse_input(input);

    let mut sum = 0;

    for update in &mut updates {
        if is_ordered(update, &rules) {
            continue;
        }

        update.sort_by(|a, b| {
            if let Some(rules) = rules.get(a)
                && rules.contains(b)
            {
                return Ordering::Less;
            }

            if let Some(rules) = rules.get(b)
                && rules.contains(a)
            {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        let mid = update.len() / 2;
        let mid = update[mid];

        sum += mid;
    }

    println!("{sum}");

    Ok(())
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (rules_str, updates) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::<i32, HashSet<i32>>::new();
    for rule in rules_str.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        rules.entry(a).or_default().insert(b);
    }

    let updates = updates
        .lines()
        .map(|update| {
            update
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    (rules, updates)
}

fn is_ordered(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for (a, b) in update.iter().tuple_combinations() {
        if let Some(rules) = rules.get(b)
            && rules.contains(a)
        {
            return false;
        }
    }
    true
}
