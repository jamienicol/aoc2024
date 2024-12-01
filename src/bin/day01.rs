use anyhow::Result;
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, opt},
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    all_consuming(fold_many1(
        terminated(
            separated_pair(parse_unsigned, tag("   "), parse_unsigned),
            opt(newline),
        ),
        || (Vec::new(), Vec::new()),
        |(mut list_a, mut list_b), (a, b)| {
            list_a.push(a);
            list_b.push(b);
            (list_a, list_b)
        },
    ))(input)
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input01.txt");
    let (mut list_a, mut list_b) = parse_input(input)?.1;

    list_a.sort_unstable();
    list_b.sort_unstable();

    let part_a = list_a
        .iter()
        .zip(&list_b)
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>();
    println!("Day 01, part A: {part_a}");
    assert_eq!(part_a, 1319616);

    let counts = list_b.into_iter().counts();
    let part_b = list_a
        .into_iter()
        .map(|a| a * counts.get(&a).cloned().unwrap_or(0))
        .sum::<usize>();
    println!("Day 01, part B: {part_b}");
    assert_eq!(part_b, 27267728);

    Ok(())
}
