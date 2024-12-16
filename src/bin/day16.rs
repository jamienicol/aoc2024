use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;
use std::{cmp::Ordering, collections::BinaryHeap};

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Map {
    width: isize,
    tiles: Vec<Tile>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn get(&self, pos: Pos) -> Tile {
        self.tiles[(pos.1 * self.width + pos.0) as usize]
    }
}

fn parse_input(input: &str) -> Result<Map> {
    let width = input.lines().next().context("Empty input")?.len() as isize;
    let (start, end, tiles) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .try_fold(
            (None, None, Vec::new()),
            |(mut start, mut end, mut tiles), (pos, c)| {
                match c {
                    '#' => {
                        tiles.push(Tile::Wall);
                    }
                    'S' => {
                        ensure!(start.is_none(), "Multiple starts found");
                        tiles.push(Tile::Empty);
                        start = Some(pos);
                    }
                    'E' => {
                        ensure!(end.is_none(), "Multiple ends found");
                        tiles.push(Tile::Empty);
                        end = Some(pos);
                    }
                    '.' => {
                        tiles.push(Tile::Empty);
                    }
                    _ => bail!("Invalid character in map: {}", c),
                }
                Ok((start, end, tiles))
            },
        )?;
    Ok(Map {
        width,
        tiles,
        start: start.context("No start found")?,
        end: end.context("No end found")?,
    })
}

#[derive(PartialEq, Eq)]
struct State {
    path: Vec<Pos>,
    dir: Pos,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(map: &Map) -> Result<(u32, usize)> {
    let mut finished_paths = Vec::new();
    let mut finished_cost = None;
    let mut costs = HashMap::default();
    let mut open = BinaryHeap::new();
    open.push(State {
        path: vec![map.start],
        dir: (1, 0),
        cost: 0,
    });

    while let Some(state) = open.pop() {
        let pos = *state.path.last().unwrap();

        if costs
            .get(&(pos, state.dir))
            .map_or(false, |&cost| state.cost > cost)
        {
            continue;
        }
        costs.insert((pos, state.dir), state.cost);

        if let Some(cost) = finished_cost {
            if state.cost > cost {
                continue;
            }
        }

        if pos == map.end {
            finished_paths.push(state.path);
            finished_cost = Some(state.cost);
            continue;
        }

        if matches!(
            map.get((pos.0 + state.dir.0, pos.1 + state.dir.1)),
            Tile::Empty
        ) {
            let mut path = state.path.clone();
            path.push((pos.0 + state.dir.0, pos.1 + state.dir.1));
            open.push(State {
                path,
                dir: state.dir,
                cost: state.cost + 1,
            });
        }
        open.push(State {
            path: state.path.clone(),
            dir: (state.dir.1, -state.dir.0),
            cost: state.cost + 1000,
        });
        open.push(State {
            path: state.path,
            dir: (-state.dir.1, state.dir.0),
            cost: state.cost + 1000,
        });
    }
    Ok((
        finished_cost.context("Failed to find path")?,
        finished_paths.into_iter().flatten().unique().count(),
    ))
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input16.txt");
    let map = parse_input(input)?;

    let (part_a, part_b) = solve(&map)?;
    println!("Day 16, part A: {part_a:?}");
    assert_eq!(part_a, 65436);
    println!("Day 16, part B: {part_b:?}");
    assert_eq!(part_b, 489);
    Ok(())
}
