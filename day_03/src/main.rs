use aoc2024::aoc_solution;
use regex::{Captures, Regex};

fn main() -> eyre::Result<()> {
    aoc_solution(3, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let total: i32 = regex
        .captures_iter(input)
        .map(|captures| get_capture_int(&captures, 1) * get_capture_int(&captures, 2))
        .sum();

    println!("{total}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut is_enabled = true;
    let mut total = 0;

    for captures in regex.captures_iter(input) {
        let instruction = captures.get(0).unwrap().as_str();
        match instruction {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => {
                if is_enabled {
                    total += get_capture_int(&captures, 1) * get_capture_int(&captures, 2)
                }
            }
        }
    }

    println!("{total}");

    Ok(())
}

fn get_capture_int(captures: &Captures<'_>, i: usize) -> i32 {
    captures.get(i).unwrap().as_str().parse::<i32>().unwrap()
}
