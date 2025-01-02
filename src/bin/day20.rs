use anyhow::{bail, ensure, Context, Result};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

type Pos = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Track,
    Wall,
}

struct Map {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
}

impl Map {
    fn get(&self, pos: Pos) -> Option<Tile> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            Some(self.tiles[(pos.1 * self.width + pos.0) as usize])
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Result<(Map, Pos, Pos)> {
    let width = input.lines().next().context("Empty input")?.len() as isize;
    let height = input.lines().count() as isize;
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
                        tiles.push(Tile::Track);
                        start = Some(pos);
                    }
                    'E' => {
                        ensure!(end.is_none(), "Multiple ends found");
                        tiles.push(Tile::Track);
                        end = Some(pos);
                    }
                    '.' => {
                        tiles.push(Tile::Track);
                    }
                    _ => bail!("Invalid character in map: {}", c),
                }
                Ok((start, end, tiles))
            },
        )?;
    Ok((
        Map {
            width,
            height,
            tiles,
        },
        start.context("No start found")?,
        end.context("No end found")?,
    ))
}

fn find_path(
    map: &Map,
    start: Pos,
    end: Pos,
    cache: &mut HashMap<(Pos, Pos), usize>,
) -> Option<usize> {
    let mut open = BinaryHeap::new();
    open.push(Reverse((0, vec![start])));
    let mut closed: HashSet<Pos> = HashSet::default();

    while let Some(Reverse((cost, path))) = open.pop() {
        let pos = *path.last().unwrap();
        if closed.contains(&pos) {
            continue;
        }
        closed.insert(pos);

        if pos == end {
            for (i, pos) in path.iter().rev().enumerate() {
                cache.insert((*pos, end), i);
            }
            return Some(path.len() - 1);
        }

        if let Some(remainder) = cache.get(&(pos, end)).copied() {
            for (i, pos) in path.iter().rev().enumerate() {
                cache.insert((*pos, end), i + remainder);
            }
            return Some(path.len() - 1 + remainder);
        }

        open.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|step| {
                    let pos = (pos.0 + step.0, pos.1 + step.1);
                    match map.get(pos) {
                        Some(Tile::Track) => {
                            let mut path = path.clone();
                            path.push(pos);
                            Some(Reverse((cost + 1, path)))
                        }
                        _ => None,
                    }
                }),
        );
    }
    None
}

fn find_cheat_starts(map: &Map, start: Pos) -> Vec<(Pos, usize)> {
    let mut open = VecDeque::new();
    open.push_back((0, start));
    let mut closed: HashSet<Pos> = HashSet::default();

    let mut cheat_starts = Vec::new();

    while let Some((cost, pos)) = open.pop_front() {
        if closed.contains(&pos) {
            continue;
        }
        closed.insert(pos);
        cheat_starts.push((pos, cost));

        open.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|step| {
                    let pos = (pos.0 + step.0, pos.1 + step.1);
                    match map.get(pos) {
                        Some(Tile::Track) => Some((cost + 1, pos)),
                        _ => None,
                    }
                }),
        );
    }

    cheat_starts
}

fn find_cheat_ends(map: &Map, start: Pos, cheat_len: usize) -> Vec<(Pos, usize)> {
    let mut open = VecDeque::new();
    open.push_back((0, start, cheat_len));
    let mut closed: HashSet<Pos> = HashSet::default();

    let mut cheat_ends = Vec::new();

    while let Some((cost, pos, remaining_len)) = open.pop_front() {
        if closed.contains(&pos) {
            continue;
        }
        closed.insert(pos);
        if map.get(pos) == Some(Tile::Track) {
            cheat_ends.push((pos, cost));
        }

        open.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|step| {
                    let pos = (pos.0 + step.0, pos.1 + step.1);
                    (remaining_len > 0).then_some((cost + 1, pos, remaining_len - 1))
                }),
        );
    }

    cheat_ends
}

fn find_cheat_paths(
    cheat_starts: &[(Pos, usize)],
    end: Pos,
    cheat_len: usize,
    max_len: usize,
    map: &Map,
    cache: &mut HashMap<(Pos, Pos), usize>,
) -> usize {
    cheat_starts
        .iter()
        .flat_map(|(cheat_start, start_cost)| {
            find_cheat_ends(map, *cheat_start, cheat_len)
                .into_iter()
                .map(move |(cheat_end, cheat_cost)| {
                    (cheat_start, cheat_end, start_cost + cheat_cost)
                })
        })
        .filter_map(|(_, cheat_end, cost)| {
            let cost = cost + find_path(map, cheat_end, end, cache)?;
            (cost <= max_len).then_some(cost)
        })
        .count()
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input20.txt");
    let (map, start, end) = parse_input(input)?;
    let mut cache = HashMap::default();

    let baseline = find_path(&map, start, end, &mut cache).context("Couldn't find path")?;
    let cheat_starts = find_cheat_starts(&map, start);

    let part_a = find_cheat_paths(&cheat_starts, end, 2, baseline - 100, &map, &mut cache);
    println!("Day 20, part A: {part_a}");
    assert_eq!(part_a, 1375);

    let part_b = find_cheat_paths(&cheat_starts, end, 20, baseline - 100, &map, &mut cache);
    println!("Day 20, part B: {part_b}");
    assert_eq!(part_b, 983054);

    Ok(())
}
