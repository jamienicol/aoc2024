use anyhow::{Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

struct Map {
    width: isize,
    height: isize,
    plants: Vec<char>,
}

impl Map {
    fn get(&self, pos: (isize, isize)) -> Option<char> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            Some(self.plants[(pos.1 * self.width + pos.0) as usize])
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Result<Map> {
    let width = input.lines().next().context("Empty input")?.len() as isize;
    let height = input.lines().count() as isize;
    let plants = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();
    Ok(Map {
        width,
        height,
        plants,
    })
}

fn find_region(map: &Map, pos: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut region = HashSet::default();
    let mut open = Vec::new();
    open.push(pos);
    while let Some(current_pos) = open.pop() {
        if region.contains(&current_pos) {
            continue;
        }
        region.insert(current_pos);
        let current_plant = map.get(current_pos).unwrap();
        open.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|step| {
                    let next_pos = (current_pos.0 + step.0, current_pos.1 + step.1);
                    if map.get(next_pos) == Some(current_plant) {
                        Some(next_pos)
                    } else {
                        None
                    }
                }),
        );
    }
    region
}

fn find_regions(map: &Map) -> Vec<HashSet<(isize, isize)>> {
    let mut seen: HashSet<(isize, isize)> = HashSet::default();
    (0..map.width)
        .cartesian_product(0..map.height)
        .filter_map(|pos| {
            if seen.contains(&pos) {
                return None;
            }
            let region = find_region(map, pos);
            seen.extend(region.iter());
            Some(region)
        })
        .collect_vec()
}

fn perimeter(region: &HashSet<(isize, isize)>) -> usize {
    region
        .iter()
        .map(|&pos| {
            4 - [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter(|step| region.contains(&(pos.0 + step.0, pos.1 + step.1)))
                .count()
        })
        .sum::<usize>()
}

fn edges(region: &HashSet<(isize, isize)>) -> usize {
    region
        .iter()
        .map(|&pos| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter(|step| {
                    !region.contains(&(pos.0 + step.0, pos.1 + step.1))
                        && (!region.contains(&(pos.0 + step.1, pos.1 + step.0))
                            || region.contains(&(pos.0 + step.0 + step.1, pos.1 + step.0 + step.1)))
                })
                .count()
        })
        .sum::<usize>()
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input12.txt");
    let map = parse_input(input)?;
    let regions = find_regions(&map);

    let part_a = regions
        .iter()
        .map(|region| region.len() * perimeter(region))
        .sum::<usize>();
    println!("Day 12, part A: {part_a}");
    assert_eq!(part_a, 1375574);

    let part_b = regions
        .iter()
        .map(|region| region.len() * edges(region))
        .sum::<usize>();
    println!("Day 12, part B: {part_b}");
    assert_eq!(part_b, 830566);

    Ok(())
}
