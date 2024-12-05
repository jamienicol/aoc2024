use anyhow::Result;
use aoc2024::util::parse_unsigned;
use nom::{
    character::complete::{char, newline},
    combinator::{all_consuming, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
// use std::collections::{HashMap, HashSet};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Rules = HashMap<usize, HashSet<usize>>;
type Updates = Vec<Vec<usize>>;

fn parse_input(input: &str) -> IResult<&str, (Rules, Updates)> {
    all_consuming(terminated(
        separated_pair(
            fold_many1(
                terminated(
                    separated_pair(parse_unsigned, char('|'), parse_unsigned),
                    newline,
                ),
                HashMap::default,
                |mut acc: Rules, (less, greater)| {
                    acc.entry(less).or_default().insert(greater);
                    acc
                },
            ),
            many1(newline),
            separated_list1(newline, separated_list1(char(','), parse_unsigned)),
        ),
        opt(newline),
    ))(input)
}

fn fix_updates(updates: &mut Updates, rules: &Rules) {
    for update in updates {
        'outer: loop {
            let mut seen = HashMap::default();
            for i in 0..update.len() {
                seen.insert(update[i], i);
                if let Some(dest) = rules
                    .get(&update[i])
                    .into_iter()
                    .flatten()
                    .filter_map(|other| seen.get(other))
                    .min()
                {
                    let val = update.remove(i);
                    update.insert(*dest, val);
                    continue 'outer;
                }
            }
            break;
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input05.txt");
    let (rules, updates) = parse_input(input)?.1;

    let (good_updates, mut bad_updates): (Updates, Updates) =
        updates.into_iter().partition(|update| {
            let mut seen = HashSet::default();
            update.iter().all(|page| {
                seen.insert(page);
                rules
                    .get(page)
                    .into_iter()
                    .flatten()
                    .all(|other| !seen.contains(other))
            })
        });

    let part_a = good_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<usize>();
    println!("Day 05, part A: {part_a}");
    assert_eq!(part_a, 4766);

    fix_updates(&mut bad_updates, &rules);
    let part_b = bad_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<usize>();
    println!("Day 05, part B: {part_b}");
    assert_eq!(part_b, 6257);

    Ok(())
}
