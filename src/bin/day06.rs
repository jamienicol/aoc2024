use anyhow::{bail, ensure, Context, Ok, Result};
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

#[derive(Clone)]
struct Map {
    width: isize,
    height: isize,
    obstacles: HashSet<(isize, isize)>,
}

#[derive(Clone)]
struct Guard {
    pos: (isize, isize),
    facing: (isize, isize),
}

enum Route {
    Finite(HashSet<(isize, isize)>),
    Loop,
}

fn parse_input(input: &str) -> Result<(Map, Guard)> {
    let width = input.lines().next().map_or(0, |line| line.len() as isize);
    let height = input.lines().count() as isize;
    let mut obstacles = HashSet::default();
    let mut guard = None;
    for (y, line) in input.trim_end().lines().enumerate() {
        ensure!(line.chars().count() as isize == width);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((x as isize, y as isize));
                }
                '^' => {
                    guard = Some(Guard {
                        pos: (x as isize, y as isize),
                        facing: (0, -1),
                    });
                }
                _ => {}
            }
        }
    }

    let map = Map {
        width,
        height,
        obstacles,
    };
    let guard = guard.context("Couldn't find guard in input")?;
    Ok((map, guard))
}

fn move_guard(map: &Map, guard: &mut Guard) {
    let new_pos = (guard.pos.0 + guard.facing.0, guard.pos.1 + guard.facing.1);
    if map.obstacles.contains(&new_pos) {
        guard.facing = (-guard.facing.1, guard.facing.0);
    } else {
        guard.pos = new_pos;
    }
}

fn simulate_route(map: &Map, mut guard: Guard) -> Route {
    let mut visited = HashSet::default();
    let mut visited_facing = HashSet::default();

    loop {
        if visited_facing.contains(&(guard.pos, guard.facing)) {
            return Route::Loop;
        }
        if !(0..map.width).contains(&guard.pos.0) || !(0..map.height).contains(&guard.pos.1) {
            return Route::Finite(visited);
        }
        visited.insert(guard.pos);
        visited_facing.insert((guard.pos, guard.facing));
        move_guard(map, &mut guard);
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input06.txt");
    let (map, guard) = parse_input(input)?;

    let visited = match simulate_route(&map, guard.clone()) {
        Route::Finite(visited) => visited.into_iter().collect_vec(),
        _ => bail!("Loop incorrectly detected in part A"),
    };
    let part_a = visited.len();
    println!("Day 06, part A: {part_a}");
    assert_eq!(part_a, 5199);

    let part_b = visited
        .into_par_iter()
        .filter(|pos| {
            let mut map = map.clone();
            map.obstacles.insert(*pos);

            matches!(simulate_route(&map, guard.clone()), Route::Loop)
        })
        .count();
    println!("Day 06, part B: {part_b}");
    assert_eq!(part_b, 1915);

    Ok(())
}
