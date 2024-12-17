#![feature(let_chains)]

use aoc2024::aoc_solution;
use eyre::bail;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(17, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (mut registers, program) = parse_input(input);

    fn load_combo(operand: u8, registers: &[usize]) -> usize {
        match operand {
            0..=3 => operand as usize,
            4..=6 => registers[operand as usize - 4],
            _ => unreachable!(),
        }
    }

    let mut output = vec![];
    let mut pc = 0;
    while pc < program.len() {
        let opcode = program[pc];
        let op = program[pc + 1];
        match opcode {
            // adv
            0 => registers[0] /= 2usize.pow(load_combo(op, &registers) as u32),
            // bxl
            1 => registers[1] ^= op as usize,
            // bst
            2 => registers[1] = load_combo(op, &registers) % 8,
            // jnz
            3 => {
                if registers[0] != 0 {
                    pc = op as usize;
                    continue;
                }
            }
            // bxc
            4 => registers[1] ^= registers[2],
            // out
            5 => output.push((load_combo(op, &registers) % 8) as u8),
            // bdv
            6 => registers[1] = registers[0] / 2usize.pow(load_combo(op, &registers) as u32),
            // cdv
            7 => registers[2] = registers[0] / 2usize.pow(load_combo(op, &registers) as u32),
            _ => bail!("invalid opcode: {opcode}"),
        }
        pc += 2;
    }

    println!("{}", output.iter().join(","));

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (_, program) = parse_input(input);

    fn find_a(program: &[u8], i: usize, n: usize) -> Option<usize> {
        let n = n << 3;
        for k in 0..8 {
            let a = n | k;
            let out = ((((a % 8) ^ 1) ^ (a / 2usize.pow(((a % 8) ^ 1) as u32))) ^ 6) as u8 % 8;
            if out == program[i] {
                if i == 0 {
                    return Some(a);
                }

                if let Some(r) = find_a(program, i - 1, a) {
                    return Some(r);
                }
            }
        }
        None
    }

    let a = find_a(&program, program.len() - 1, 0).unwrap();

    println!("{a}");

    Ok(())
}

fn parse_input(input: &str) -> ([usize; 3], Vec<u8>) {
    let mut lines = input.lines();

    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let c = lines.next().unwrap();

    let p = lines.nth(1).unwrap();

    let (_, a) = a.split_once(": ").unwrap();
    let (_, b) = b.split_once(": ").unwrap();
    let (_, c) = c.split_once(": ").unwrap();
    let (_, p) = p.split_once(": ").unwrap();

    let a = a.parse().unwrap();
    let b = b.parse().unwrap();
    let c = c.parse().unwrap();
    let p = p.split(',').map(|i| i.parse().unwrap()).collect_vec();

    ([a, b, c], p)
}
