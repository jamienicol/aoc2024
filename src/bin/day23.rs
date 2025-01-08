use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, char, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    all_consuming(terminated(
        separated_list1(newline, separated_pair(alpha1, char('-'), alpha1)),
        opt(newline),
    ))(input)
}
fn main() -> Result<()> {
    let input = include_str!("../../res/input23.txt");
    let connections = parse_input(input)?.1;
    let connections = connections.into_iter().fold(
        HashMap::default(),
        |mut acc: HashMap<&str, HashSet<&str>>, (a, b)| {
            acc.entry(a).or_default().insert(b);
            acc.entry(b).or_default().insert(a);
            acc
        },
    );

    let part_a = connections
        .iter()
        .filter(|(a, _)| a.starts_with('t'))
        .fold(
            HashSet::default(),
            |mut acc: HashSet<[&str; 3]>, (a, others)| {
                for (b, c) in others.iter().tuple_combinations() {
                    if connections.get(b).unwrap().contains(c) {
                        let mut group = [*a, *b, *c];
                        group.sort();
                        acc.insert(group);
                    }
                }
                acc
            },
        )
        .len();
    println!("Day 23, part A: {part_a}");
    assert_eq!(part_a, 1368);

    let mut groups = vec![vec![]];
    for a in connections.keys() {
        let others = connections.get(a).unwrap();
        let old_groups = groups.clone();
        groups.extend(old_groups.into_iter().filter_map(|mut group| {
            if group.iter().all(|b| others.contains(b)) {
                group.push(*a);
                group.sort();
                Some(group)
            } else {
                None
            }
        }));
    }

    let part_b = groups
        .iter()
        .max_by_key(|group| group.len())
        .context("No groups found")?
        .join(",");
    println!("Day 23, part B: {part_b}");
    assert_eq!(part_b, "dd,ig,il,im,kb,kr,pe,ti,tv,vr,we,xu,zi");

    Ok(())
}
