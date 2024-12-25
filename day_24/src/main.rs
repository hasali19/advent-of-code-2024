use std::cmp::Reverse;
use std::collections::HashMap;

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(24, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (initial_values, gates) = input.split_once("\n\n").unwrap();

    let mut wires = initial_values
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            (name, value.parse::<u8>().unwrap() == 1)
        })
        .collect::<HashMap<_, _>>();

    let gates = gates
        .lines()
        .map(|line| {
            let (formula, output) = line.split_once(" -> ").unwrap();
            let (a, op, b) = formula.split_whitespace().collect_tuple().unwrap();
            (output, (a, b, op))
        })
        .collect::<HashMap<_, _>>();

    let res = gates
        .keys()
        .filter(|name| name.starts_with('z'))
        .sorted_by_key(|&v| Reverse(v))
        .map(|name| read_output(&mut wires, &gates, name))
        .fold(0usize, |acc, v| acc << 1 | if v { 1 } else { 0 });

    println!("{res}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    Ok(())
}

fn read_output<'a>(
    wires: &mut HashMap<&'a str, bool>,
    gates: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    name: &'a str,
) -> bool {
    if let Some(value) = wires.get(name) {
        return *value;
    }

    let (a, b, op) = gates[name];

    let a = read_output(wires, gates, a);
    let b = read_output(wires, gates, b);

    let output = match op {
        "AND" => a && b,
        "OR" => a || b,
        "XOR" => a ^ b,
        _ => unreachable!(),
    };

    wires.insert(name, output);

    output
}
