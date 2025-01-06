use anyhow::Result;
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    character::complete::newline,
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use rustc_hash::FxHashMap as HashMap;

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    all_consuming(terminated(
        separated_list1(newline, parse_unsigned),
        opt(newline),
    ))(input)
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input22.txt");
    let intial_numbers = parse_input(input)?.1;

    let numbers_per_buyer = intial_numbers
        .iter()
        .map(|number| {
            std::iter::successors(Some(*number), |n| {
                let mut n = *n;
                n = (n ^ (n * 64)) % 16777216;
                n = (n ^ (n / 32)) % 16777216;
                n = (n ^ (n * 2048)) % 16777216;
                Some(n)
            })
            .take(2001)
            .collect_vec()
        })
        .collect_vec();

    let part_a = numbers_per_buyer
        .iter()
        .map(|numbers| numbers.last().unwrap())
        .sum::<i64>();
    println!("Day 22, part A: {part_a}");
    assert_eq!(part_a, 14119253575);

    let price_per_buyer = numbers_per_buyer
        .iter()
        .map(|numbers| {
            numbers
                .windows(5)
                .map(|window| {
                    (
                        [
                            window[1] % 10 - window[0] % 10,
                            window[2] % 10 - window[1] % 10,
                            window[3] % 10 - window[2] % 10,
                            window[4] % 10 - window[3] % 10,
                        ],
                        window[4] % 10,
                    )
                })
                .fold(
                    HashMap::default(),
                    |mut acc: HashMap<[i64; 4], i64>, (pattern, price)| {
                        acc.entry(pattern).or_insert(price);
                        acc
                    },
                )
        })
        .collect_vec();

    let total_prices = price_per_buyer.into_iter().fold(
        HashMap::default(),
        |mut acc: HashMap<[i64; 4], i64>, prices| {
            for (pattern, price) in prices {
                *acc.entry(pattern).or_insert(0) += price;
            }
            acc
        },
    );
    let part_b = total_prices.into_values().max().unwrap();
    println!("Day 22, part B: {part_b}");
    assert_eq!(part_b, 1600);

    Ok(())
}
