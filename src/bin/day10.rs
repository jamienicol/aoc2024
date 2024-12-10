use anyhow::{Context, Result};
use itertools::Itertools;

struct Map {
    width: isize,
    height: isize,
    elevations: Vec<u32>,
}

impl Map {
    fn get(&self, pos: (isize, isize)) -> Option<u32> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            Some(self.elevations[(pos.1 * self.width + pos.0) as usize])
        } else {
            None
        }
    }

    fn trailheads(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.elevations
            .iter()
            .enumerate()
            .filter_map(move |(i, &elevation)| {
                if elevation == 0 {
                    Some((i as isize % self.width, i as isize / self.width))
                } else {
                    None
                }
            })
    }

    fn find_trails(&self, head: (isize, isize)) -> Vec<Vec<(isize, isize)>> {
        assert!(self.get(head) == Some(0));
        let mut trails: Vec<Vec<(isize, isize)>> = Vec::new();
        let mut open = Vec::new();
        open.push(vec![head]);
        while let Some(current_trail) = open.pop() {
            let current_pos = *current_trail.last().unwrap();

            if self.get(current_pos) == Some(9) {
                trails.push(current_trail);
            } else {
                open.extend(
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .filter_map(|dir| {
                            let next_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
                            if self.get(next_pos) == Some(self.get(current_pos).unwrap() + 1) {
                                let mut next_trail = current_trail.clone();
                                next_trail.push(next_pos);
                                Some(next_trail)
                            } else {
                                None
                            }
                        }),
                );
            }
        }
        trails
    }
}

fn parse_input(input: &str) -> Result<Map> {
    let width = input.lines().next().context("Empty input")?.len() as isize;
    let height = input.lines().count() as isize;
    let elevations = input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| {
            c.to_digit(10)
                .with_context(|| format!("Invalid input: {}", c))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Map {
        width,
        height,
        elevations,
    })
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input10.txt");
    let map = parse_input(input)?;

    let trailheads = map
        .trailheads()
        .map(|trailhead| map.find_trails(trailhead))
        .collect_vec();

    let part_a = trailheads
        .iter()
        .map(|trails| {
            trails
                .iter()
                .filter_map(|trail| trail.last())
                .unique()
                .count()
        })
        .sum::<usize>();
    println!("Day 10, part A: {part_a}");
    assert_eq!(part_a, 510);

    let part_b = trailheads.iter().map(|trails| trails.len()).sum::<usize>();
    println!("Day 10, part B: {part_b}");
    assert_eq!(part_b, 1058);

    Ok(())
}
