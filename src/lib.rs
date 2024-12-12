use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::Parser;
use eyre::{Context, eyre};

#[derive(Parser)]
struct Args {
    input: Option<PathBuf>,
}

pub fn aoc_solution(day: u32, runner: impl Fn(&str) -> eyre::Result<()>) -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let input_path = match args.input {
        Some(path) => path,
        None => {
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("day_{day:02}"))
                .join("input.txt");

            if !path.exists() {
                download_input(day, &path)?;
            }

            path
        }
    };

    let input = fs::read_to_string(&input_path)
        .wrap_err_with(|| eyre!("failed to read path: {input_path:?}"))?;

    runner(&input)
}

pub fn download_input(day: u32, out_path: impl AsRef<Path>) -> eyre::Result<()> {
    let input_url = format!("https://adventofcode.com/2024/day/{day}/input");
    let aoc_session = env::var("AOC_SESSION").wrap_err("could not read `AOC_SESSION` cookie")?;

    println!("> downloading input from {input_url}");

    let input = ureq::get(&input_url)
        .set("Cookie", &format!("session={aoc_session}"))
        .call()?
        .into_string()?;

    fs::write(out_path, input)?;

    Ok(())
}
