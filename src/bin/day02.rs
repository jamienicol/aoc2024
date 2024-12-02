use anyhow::Result;
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    character::complete::{char, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    all_consuming(terminated(
        separated_list1(newline, separated_list1(char(' '), parse_unsigned)),
        opt(newline),
    ))(input)
}

fn is_report_safe(report: impl Iterator<Item = usize>) -> bool {
    let mut increasing = None;
    report.tuple_windows().all(|(a, b)| {
        let safe_diff = (1..=3).contains(&a.abs_diff(b));
        let safe_dir = match increasing {
            Some(true) => b > a,
            Some(false) => a > b,
            None => {
                increasing = Some(b > a);
                true
            }
        };

        safe_diff && safe_dir
    })
}

fn skip_nth<T>(it: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
    it.enumerate()
        .filter_map(move |(i, item)| (i != n).then_some(item))
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input02.txt");
    let reports = parse_input(input)?.1;

    let part_a = reports
        .iter()
        .filter(|report| is_report_safe(report.iter().copied()))
        .count();
    println!("Day 02, part A: {part_a}");
    assert_eq!(part_a, 341);

    let part_b = reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|n| is_report_safe(skip_nth(report.iter().copied(), n)))
        })
        .count();
    println!("Day 02, part B: {part_b}");
    assert_eq!(part_b, 404);

    Ok(())
}
