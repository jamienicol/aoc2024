use anyhow::Result;
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use rayon::prelude::*;

fn parse_input(input: &str) -> IResult<&str, Vec<(usize, Vec<usize>)>> {
    all_consuming(terminated(
        separated_list1(
            newline,
            separated_pair(
                parse_unsigned,
                tag(": "),
                separated_list1(char(' '), parse_unsigned),
            ),
        ),
        opt(newline),
    ))(input)
}

fn possible_results(operands: &[usize], allow_cat: bool) -> Vec<usize> {
    let mut operands = operands.iter();
    let first = match operands.next() {
        Some(first) => vec![*first],
        None => Vec::new(),
    };
    operands.fold(first, |acc, rhs| {
        acc.iter()
            .map(|lhs| lhs + rhs)
            .chain(acc.iter().map(|lhs| lhs * rhs))
            .chain(
                allow_cat
                    .then_some(
                        acc.iter()
                            .map(|lhs| lhs * 10usize.pow(rhs.ilog10() + 1) + rhs),
                    )
                    .into_iter()
                    .flatten(),
            )
            .collect_vec()
    })
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input07.txt");
    let equations = parse_input(input)?.1;

    let part_a = equations
        .par_iter()
        .filter_map(|(result, operands)| {
            possible_results(operands, false)
                .into_iter()
                .find(|possibility| possibility == result)
        })
        .sum::<usize>();
    println!("Day 07, part A: {part_a}");
    assert_eq!(part_a, 303876485655);

    let part_b = equations
        .par_iter()
        .filter_map(|(result, operands)| {
            possible_results(operands, true)
                .into_iter()
                .find(|possibility| possibility == result)
        })
        .sum::<usize>();
    println!("Day 07, part B: {part_b}");
    assert_eq!(part_b, 146111650210682);

    Ok(())
}
