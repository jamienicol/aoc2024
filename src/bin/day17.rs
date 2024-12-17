use anyhow::{bail, Context, Result};
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

type Regs = (u64, u64, u64);

fn parse_input(input: &str) -> IResult<&str, (Regs, Vec<u8>)> {
    all_consuming(separated_pair(
        tuple((
            delimited(tag("Register A: "), parse_unsigned, newline),
            delimited(tag("Register B: "), parse_unsigned, newline),
            delimited(tag("Register C: "), parse_unsigned, newline),
        )),
        newline,
        delimited(
            tag("Program: "),
            separated_list1(char(','), parse_unsigned),
            opt(newline),
        ),
    ))(input)
}

fn combo(operand: u8, regs: &Regs) -> Result<u64> {
    match operand {
        0..=3 => Ok(operand as u64),
        4 => Ok(regs.0),
        5 => Ok(regs.1),
        6 => Ok(regs.2),
        _ => bail!("Unexpected operand {}", operand),
    }
}

fn run_program(mut regs: Regs, instructions: &[u8]) -> Result<Vec<u8>> {
    let mut ip = 0;
    let mut out = Vec::new();
    while ip < instructions.len() - 1 {
        let (instr, operand) = (instructions[ip], instructions[ip + 1]);
        match instr {
            0 => {
                // adv
                regs.0 /= 2u64.pow(combo(operand, &regs)? as u32);
                ip += 2;
            }
            1 => {
                // bxl
                regs.1 ^= operand as u64;
                ip += 2;
            }
            2 => {
                // bst
                regs.1 = combo(operand, &regs)? & 0b111;
                ip += 2;
            }
            3 => {
                // jnz
                if regs.0 == 0 {
                    ip += 2;
                } else {
                    ip = operand as usize;
                }
            }
            4 => {
                // bxc
                regs.1 ^= regs.2;
                ip += 2;
            }
            5 => {
                // out
                out.push((combo(operand, &regs)? & 0b111) as u8);
                ip += 2;
            }
            6 => {
                // bdv
                regs.1 = regs.0 / 2u64.pow(combo(operand, &regs)? as u32);
                ip += 2;
            }
            7 => {
                // cdv
                regs.2 = regs.0 / 2u64.pow(combo(operand, &regs)? as u32);
                ip += 2;
            }
            _ => bail!("Unexpected instruction {}", instr),
        }
    }

    Ok(out)
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input17.txt");
    let (regs, instructions) = parse_input(input)?.1;

    let part_a = run_program(regs, &instructions)?
        .iter()
        .map(|x| x.to_string())
        .join(",");
    println!("Day 17, part A: {part_a}");
    assert_eq!(part_a, "6,0,6,3,0,2,3,1,6");

    let part_b = (0..instructions.len())
        .fold(vec![0], |acc, _| {
            acc.into_iter()
                .flat_map(|acc| (0..8).map(move |a| (acc << 3) | a))
                .filter(|a| {
                    run_program((*a, regs.1, regs.2), &instructions).map_or(false, |out| {
                        out.iter()
                            .rev()
                            .zip(instructions.iter().rev())
                            .all(|(a, b)| a == b)
                    })
                })
                .collect_vec()
        })
        .into_iter()
        .min()
        .context("No solution found")?;
    println!("Day 17, part B: {part_b:?}");
    assert_eq!(part_b, 236539226447469);

    Ok(())
}
