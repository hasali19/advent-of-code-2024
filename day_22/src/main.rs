use aoc2024::aoc_solution;

fn main() -> eyre::Result<()> {
    aoc_solution(22, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut sum = 0;

    for n in input.lines() {
        let mut n = n.parse::<usize>()?;
        for _ in 0..2000 {
            n = next(n);
        }
        sum += n;
    }

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    Ok(())
}

fn next(n: usize) -> usize {
    let n = mix_and_prune(n, n * 64);
    let n = mix_and_prune(n, n / 32);

    mix_and_prune(n, n * 2048)
}

fn mix_and_prune(s: usize, n: usize) -> usize {
    (s ^ n) % 16777216
}
