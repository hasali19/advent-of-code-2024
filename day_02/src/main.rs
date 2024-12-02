use aoc2024::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(2, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let count = input
        .lines()
        .filter(|line| {
            let levels = line
                .split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec();

            is_report_safe(levels.iter().copied())
        })
        .count();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let count = input
        .lines()
        .filter(|line| {
            let levels = line
                .split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec();

            if is_report_safe(levels.iter().copied()) {
                return true;
            }

            for i in 0..levels.len() {
                if is_report_safe(levels.iter().copied().skip_nth(i)) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("{count}");

    Ok(())
}

fn is_report_safe(levels: impl Iterator<Item = i32>) -> bool {
    let mut diffs = levels.tuple_windows().map(|(a, b)| a - b);
    let Some(diff) = diffs.next() else {
        return true;
    };

    let is_safe_diff = |d: i32| d.abs() >= 1 && d.abs() <= 3;

    is_safe_diff(diff) && diffs.all(|d| is_safe_diff(d) && d.signum() == diff.signum())
}

trait IteratorExt: Iterator + Sized {
    /// Returns all elements of the iterator except the nth.
    fn skip_nth(self, n: usize) -> impl Iterator<Item = Self::Item> {
        self.enumerate()
            .filter(move |&(i, _)| n != i)
            .map(|(_, l)| l)
    }
}

impl<T: Iterator + Sized> IteratorExt for T {}
