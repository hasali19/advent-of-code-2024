use std::env;
use std::path::PathBuf;

use clap::{Args, Parser};
use duct::cmd;
use eyre::{Context, ContextCompat};
use jiff::Zoned;

#[derive(Parser)]
enum Command {
    Init(InitArgs),
}

#[derive(Args)]
struct InitArgs {
    day: Option<u32>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let command = Command::parse();
    match command {
        Command::Init(InitArgs { day }) => init(day),
    }
}

fn init(day: Option<u32>) -> eyre::Result<()> {
    let day = day
        .or_else(|| {
            let now = Zoned::now();
            if now.year() == 2024 && now.month() == 12 {
                Some(now.day() as u32)
            } else {
                None
            }
        })
        .wrap_err("cannot determine day, please specify as argument")?;

    let input_url = format!("https://adventofcode.com/2023/day/{day}/input");
    let solution_path = PathBuf::from(format!("day_{day:02}"));
    let aoc_session = env::var("AOC_SESSION").wrap_err("could not read `AOC_SESSION` cookie")?;

    if !solution_path.exists() {
        cmd!("cargo", "new", &solution_path).run()?;

        std::fs::write(
            solution_path.join("Cargo.toml"),
            include_str!("template/Cargo.toml.template").replace("<DAY>", &format!("{day:02}")),
        )?;

        std::fs::write(
            solution_path.join("src/main.rs"),
            include_str!("template/main.rs.template").replace("<DAY>", &format!("{day}")),
        )?;
    }

    let input = ureq::get(&input_url)
        .set("Cookie", &format!("session={aoc_session}"))
        .call()?
        .into_string()?;

    std::fs::write(format!("day_{day:02}/input.txt"), input)?;

    Ok(())
}
