use anyhow::{Context, Ok, Result};
use aoc2024::util::parse_unsigned;
use nom::{
    character::complete::{char, newline},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use rustc_hash::FxHashMap as HashMap;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Safe,
    Corrupted,
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::Safe; (width * height) as usize],
        }
    }

    fn get(&self, pos: (isize, isize)) -> Option<&Tile> {
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            return None;
        }
        self.tiles.get((pos.1 * self.width + pos.0) as usize)
    }

    fn get_mut(&mut self, pos: (isize, isize)) -> Option<&mut Tile> {
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            return None;
        }
        self.tiles.get_mut((pos.1 * self.width + pos.0) as usize)
    }
}
fn parse_input(input: &str) -> IResult<&str, Vec<(isize, isize)>> {
    all_consuming(terminated(
        separated_list1(
            newline,
            separated_pair(parse_unsigned, char(','), parse_unsigned),
        ),
        opt(newline),
    ))(input)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: (isize, isize),
    g: u32,
    h: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g + other.h).cmp(&(self.g + self.h))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_dist(a: (isize, isize), b: (isize, isize)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn find_path(start: (isize, isize), end: (isize, isize), map: &Map) -> Option<u32> {
    let mut open = BinaryHeap::new();
    let mut costs = HashMap::default();
    open.push(State {
        pos: start,
        g: 0,
        h: manhattan_dist(start, end),
    });

    while let Some(state) = open.pop() {
        if state.pos == end {
            return Some(state.g);
        }

        if costs.get(&state.pos).map_or(false, |cost| state.g >= *cost) {
            continue;
        }
        costs.insert(state.pos, state.g);

        open.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|dir| {
                    let new_pos = (state.pos.0 + dir.0, state.pos.1 + dir.1);
                    (map.get(new_pos) == Some(&Tile::Safe)).then(|| State {
                        pos: new_pos,
                        g: state.g + 1,
                        h: manhattan_dist(new_pos, end),
                    })
                }),
        );
    }
    None
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input18.txt");
    let corruptions = parse_input(input)?.1;

    let mut map = corruptions
        .iter()
        .take(1024)
        .fold(Map::new(71, 71), |mut map, pos| {
            if let Some(tile) = map.get_mut(*pos) {
                *tile = Tile::Corrupted;
            }
            map
        });

    let part_a = find_path((0, 0), (70, 70), &map).context("No path found")?;
    println!("Day 18, Part A: {part_a}");
    assert_eq!(part_a, 356);

    let part_b = corruptions
        .into_iter()
        .skip(1024)
        .find(|pos| {
            if let Some(tile) = map.get_mut(*pos) {
                *tile = Tile::Corrupted;
            }
            find_path((0, 0), (70, 70), &map).is_none()
        })
        .context("Path found after all corruptions")
        .map(|(x, y)| format!("{x},{y}"))?;
    println!("Day 18, Part B: {part_b}");
    assert_eq!(part_b, "22,33");

    Ok(())
}
