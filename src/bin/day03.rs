use anyhow::Result;
use aoc2024::util::parse_unsigned;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::char,
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone)]
enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    alt((
        map(
            delimited(
                tag("mul("),
                separated_pair(parse_unsigned, char(','), parse_unsigned),
                char(')'),
            ),
            |(a, b)| Instr::Mul(a, b),
        ),
        value(Instr::Do, tag("do()")),
        value(Instr::Dont, tag("don't()")),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instr>> {
    fold_many0(
        alt((map(parse_instr, Some), value(None, take(1usize)))),
        Vec::new,
        |mut acc, item| {
            if let Some(instr) = item {
                acc.push(instr);
            }
            acc
        },
    )(input)
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input03.txt");
    let instructions = parse_input(input)?.1;

    let part_a = instructions
        .iter()
        .map(|instr| match instr {
            Instr::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<usize>();
    println!("Day 03, part A: {part_a}");
    assert_eq!(part_a, 188116424);

    let part_b = instructions
        .iter()
        .fold((0, true), |(mut sum, mut enabled), instr| {
            match instr {
                Instr::Mul(a, b) => {
                    if enabled {
                        sum += a * b;
                    }
                }
                Instr::Do => enabled = true,
                Instr::Dont => enabled = false,
            }
            (sum, enabled)
        })
        .0;
    println!("Day 03, part B: {part_b}");
    assert_eq!(part_b, 104245808);

    Ok(())
}
