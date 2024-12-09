use anyhow::{Ok, Result};
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    bytes::complete::take,
    character::complete::newline,
    combinator::{all_consuming, map_parser, opt},
    multi::many0,
    sequence::terminated,
    IResult,
};
use std::collections::HashSet;

fn parse_input(input: &str) -> IResult<&str, Vec<usize>> {
    all_consuming(terminated(
        many0(map_parser(take(1usize), parse_unsigned)),
        opt(newline),
    ))(input)
}

type Disk = Vec<Option<usize>>;

fn defrag_a(mut disk: Disk) -> Disk {
    let mut start = 0;
    let mut end = disk.len() - 1;
    while start < end {
        match (disk[start], disk[end]) {
            (_, None) => end -= 1,
            (Some(_), _) => start += 1,
            (None, Some(file)) => {
                disk[start] = Some(file);
                disk[end] = None;
                start += 1;
                end -= 1;
            }
        }
    }
    disk
}

fn defrag_b(mut disk: Disk) -> Disk {
    let mut end = disk.len() - 1;
    let mut seen: HashSet<usize> = HashSet::default();
    while end > 0 {
        match disk[end] {
            None => end -= 1,
            Some(id) if seen.contains(&id) => end -= 1,
            Some(id) => {
                seen.insert(id);

                let mut file_start = end;
                while file_start > 0 && disk[file_start - 1] == disk[end] {
                    file_start -= 1;
                }
                let file_size = end - file_start + 1;
                let free_start = disk[0..file_start]
                    .windows(file_size)
                    .position(|window| window.iter().all(|block| block.is_none()));
                if let Some(free_start) = free_start {
                    disk.copy_within(file_start..=end, free_start);
                    disk[file_start..(file_start + file_size)].fill(None);
                }
            }
        }
    }

    disk
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, file)| file.map(|file| i * file))
        .sum::<usize>()
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input09.txt");
    let disk_map = parse_input(input)?.1;

    let disk = disk_map
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chunk)| {
            let file_size = chunk[0];
            let free_size = *chunk.get(1).unwrap_or(&0);
            std::iter::repeat(Some(id))
                .take(file_size)
                .chain(std::iter::repeat(None).take(free_size))
        })
        .collect_vec();

    let part_a = checksum(&defrag_a(disk.clone()));
    println!("Day 09, part A: {part_a}");
    assert_eq!(part_a, 6201130364722);

    let part_b = checksum(&defrag_b(disk));
    println!("Day 09, part B: {part_b}");
    assert_eq!(part_b, 6221662795602);

    Ok(())
}
