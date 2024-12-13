#![feature(let_chains)]

use aoc2024::aoc_solution;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use regex::Regex;

fn main() -> eyre::Result<()> {
    aoc_solution(13, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let machines = parse_input(input);

    let mut cost = 0;

    for ([(ax, ay), (bx, by)], (px, py)) in machines {
        let a = array![[ax, bx], [ay, by]];
        let b = array![px, py];

        let r = a
            .mapv(|v| v as f64)
            .solve_into(b.mapv(|v| v as f64))
            .unwrap()
            .mapv(|v| v.round() as u64);

        if a.dot(&r) == b {
            cost += r[0] * 3 + r[1];
        }
    }

    println!("{cost}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let machines = parse_input(input);

    let mut cost = 0;

    for ([(ax, ay), (bx, by)], (px, py)) in machines {
        let px = px + 10000000000000;
        let py = py + 10000000000000;

        let a = array![[ax, bx], [ay, by]];
        let b = array![px, py];

        let r = a
            .mapv(|v| v as f64)
            .solve_into(b.mapv(|v| v as f64))
            .unwrap()
            .mapv(|v| v.round() as u64);

        if a.dot(&r) == b {
            cost += r[0] * 3 + r[1];
        }
    }

    println!("{cost}");

    Ok(())
}

type Pos = (u64, u64);

fn parse_input(input: &str) -> Vec<([Pos; 2], Pos)> {
    input
        .split("\n\n")
        .map(|m| {
            let re = Regex::new(r"\d+").unwrap();
            let (ax, ay, bx, by, px, py) = re
                .find_iter(m)
                .map(|m| m.as_str().parse().unwrap())
                .take(6)
                .collect_tuple()
                .unwrap();
            ([(ax, ay), (bx, by)], (px, py))
        })
        .collect_vec()
}
