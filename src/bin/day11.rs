use anyhow::Result;
use aoc2024::util::parse_unsigned;
use nom::{
    character::complete::{char, multispace0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::collections::HashMap;

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    all_consuming(terminated(
        separated_list1(char(' '), parse_unsigned),
        multispace0,
    ))(input)
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    stones
        .into_iter()
        .fold(HashMap::default(), |mut acc, (stone, count)| {
            if stone == 0 {
                *acc.entry(1).or_default() += count;
            } else {
                let num_digits = stone.ilog10() + 1;
                if num_digits % 2 == 0 {
                    *acc.entry(stone / 10u64.pow(num_digits / 2)).or_default() += count;
                    *acc.entry(stone % 10u64.pow(num_digits / 2)).or_default() += count;
                } else {
                    *acc.entry(stone * 2024).or_default() += count;
                }
            }
            acc
        })
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input11.txt");
    let stones = parse_input(input)?.1;
    let stones: HashMap<u64, u64> = stones.into_iter().map(|stone| (stone, 1)).collect();

    let part_a = (0..25)
        .fold(stones.clone(), |stones, _| blink(stones))
        .values()
        .sum::<u64>();
    println!("Day 11, part A: {}", part_a);
    assert_eq!(part_a, 203953);

    let part_b = (0..75)
        .fold(stones, |stones, _| blink(stones))
        .values()
        .sum::<u64>();
    println!("Day 11, part B: {}", part_b);
    assert_eq!(part_b, 242090118578155);

    Ok(())
}
