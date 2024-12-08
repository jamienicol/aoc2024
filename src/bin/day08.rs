use anyhow::Result;
use itertools::{Either, Itertools};
use std::collections::{HashMap, HashSet};

struct Map {
    width: isize,
    height: isize,
    antennae: HashMap<char, HashSet<(isize, isize)>>,
}

fn parse_input(input: &str) -> Map {
    let width = input.lines().next().map_or(0, |line| line.len() as isize);
    let height = input.lines().count() as isize;
    let mut antennae: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z') {
                antennae
                    .entry(c)
                    .or_default()
                    .insert((x as isize, y as isize));
            }
        }
    }

    Map {
        width,
        height,
        antennae,
    }
}

fn get_antinodes(
    antennae: &HashSet<(isize, isize)>,
    width: isize,
    height: isize,
    harmonics: bool,
) -> impl Iterator<Item = (isize, isize)> + '_ {
    antennae
        .iter()
        .tuple_combinations()
        .flat_map(move |(a, b)| {
            let step = (b.0 - a.0, b.1 - a.1);
            match harmonics {
                false => {
                    let antinode_a = (a.0 - step.0, a.1 - step.1);
                    let antinode_b = (b.0 + step.0, b.1 + step.1);
                    Either::Left(
                        [antinode_a, antinode_b]
                            .into_iter()
                            .filter(move |(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height),
                    )
                }
                true => {
                    let gen_harmonics = |start: (isize, isize), step: (isize, isize)| {
                        (0..)
                            .map(move |n| (start.0 + step.0 * n, start.1 + step.1 * n))
                            .take_while(move |(x, y)| {
                                *x >= 0 && *x < width && *y >= 0 && *y < height
                            })
                    };
                    Either::Right(
                        gen_harmonics(*a, (-step.0, -step.1)).chain(gen_harmonics(*b, step)),
                    )
                }
            }
        })
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input08.txt");
    let map = parse_input(input);

    let part_a = map
        .antennae
        .values()
        .flat_map(|antennae| get_antinodes(antennae, map.width, map.height, false))
        .unique()
        .count();
    println!("Day 08, part A: {}", part_a);
    assert_eq!(part_a, 222);

    let part_b = map
        .antennae
        .values()
        .flat_map(|antennae| get_antinodes(antennae, map.width, map.height, true))
        .unique()
        .count();
    println!("Day 08, part B: {}", part_b);
    assert_eq!(part_b, 884);

    Ok(())
}
