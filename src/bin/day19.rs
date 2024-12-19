use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{pair, separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    all_consuming(terminated(
        separated_pair(
            separated_list1(tag(", "), alpha1),
            pair(newline, newline),
            separated_list1(newline, alpha1),
        ),
        opt(newline),
    ))(input)
}

fn permutations<'a>(design: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(count) = cache.get(design) {
        return *count;
    }

    let count = towels
        .iter()
        .map(|towel| {
            if design == *towel {
                1
            } else if let Some(remainder) = design.strip_prefix(towel) {
                permutations(remainder, towels, cache)
            } else {
                0
            }
        })
        .sum::<u64>();
    cache.insert(design, count);
    count
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input19.txt");
    let (towels, designs) = parse_input(input)?.1;

    let mut cache = HashMap::default();
    let perms = designs
        .iter()
        .map(|design| permutations(design, &towels, &mut cache))
        .collect_vec();

    let part_a = perms.iter().filter(|ways| **ways > 0).count();
    println!("Day 19, part A: {part_a}");
    assert_eq!(part_a, 258);

    let part_b = perms.into_iter().sum::<u64>();
    println!("Day 19, part B: {part_b}");
    assert_eq!(part_b, 632423618484345);

    Ok(())
}
