use std::collections::HashMap;
use std::iter;

use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(21, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let num_pad = build_pad_map(&["789", "456", "123", " 0A"]);
    let dir_pad = build_pad_map(&[" ^A", "<v>"]);

    let codes = input.lines().collect_vec();
    let sequences = codes
        .iter()
        .map(|&code| find_shortest_sequence(&code.chars().collect_vec(), &num_pad))
        .map(|s| find_shortest_sequence(&s, &dir_pad))
        .map(|s| find_shortest_sequence(&s, &dir_pad))
        .collect_vec();

    let complexities = sequences
        .iter()
        .enumerate()
        .map(|(i, s)| s.len() * codes[i][..codes[i].len() - 1].parse::<usize>().unwrap())
        .sum::<usize>();

    println!("{complexities}");

    Ok(())
}

fn build_pad_map(pad: &[&str]) -> HashMap<char, (usize, usize)> {
    let mut map = HashMap::new();
    for (y, &row) in pad.iter().enumerate() {
        for (x, c) in row.char_indices() {
            map.insert(c, (x, y));
        }
    }
    map
}

fn shortest_path(a: char, b: char, pad: &HashMap<char, (usize, usize)>) -> Vec<char> {
    let (ax, ay) = pad[&a];
    let (bx, by) = pad[&b];

    let dx = if bx > ax {
        iter::repeat_n('>', bx - ax)
    } else {
        iter::repeat_n('<', ax - bx)
    };

    let dy = if by > ay {
        iter::repeat_n('v', by - ay)
    } else {
        iter::repeat_n('^', ay - by)
    };

    let gap = pad[&' '];
    let seq = if bx > ax && (ax, by) != gap {
        dy.chain(dx)
    } else if (bx, ay) != gap {
        dx.chain(dy)
    } else {
        dy.chain(dx)
    };

    seq.chain(iter::once('A')).collect_vec()
}

fn find_shortest_sequence(
    target_sequence: &[char],
    pad: &HashMap<char, (usize, usize)>,
) -> Vec<char> {
    let mut sequence = vec![];

    let mut current_key = 'A';
    for &target_key in target_sequence {
        sequence.extend(shortest_path(current_key, target_key, pad));
        current_key = target_key;
    }

    sequence
}

fn part_2(input: &str) -> eyre::Result<()> {
    Ok(())
}
