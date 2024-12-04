use anyhow::{bail, Result};
use itertools::Itertools;

struct Grid {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, pos: (isize, isize)) -> Option<char> {
        if pos.0 < 0 || pos.0 as usize >= self.width || pos.1 < 0 || pos.1 as usize >= self.height {
            return None;
        }
        let idx = pos.1 as usize * self.width + pos.0 as usize;
        self.cells.get(idx).copied()
    }

    fn count_xmas_lines_at(&self, pos: (isize, isize)) -> usize {
        let step_iter = (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(x, y)| *x != 0 || *y != 0);
        step_iter
            .filter(|step| {
                "XMAS".chars().enumerate().all(|(i, c)| {
                    let x = pos.0 + i as isize * step.0;
                    let y = pos.1 + i as isize * step.1;
                    self.get((x, y)) == Some(c)
                })
            })
            .count()
    }

    fn has_xmas_x_at(&self, pos: (isize, isize)) -> bool {
        self.get(pos) == Some('A')
            && ((self.get((pos.0 - 1, pos.1 - 1)) == Some('M')
                && self.get((pos.0 + 1, pos.1 + 1)) == Some('S'))
                || (self.get((pos.0 - 1, pos.1 - 1)) == Some('S')
                    && self.get((pos.0 + 1, pos.1 + 1)) == Some('M')))
            && ((self.get((pos.0 - 1, pos.1 + 1)) == Some('M')
                && self.get((pos.0 + 1, pos.1 - 1)) == Some('S'))
                || (self.get((pos.0 - 1, pos.1 + 1)) == Some('S')
                    && self.get((pos.0 + 1, pos.1 - 1)) == Some('M')))
    }
}

fn parse_input(input: &str) -> Result<Grid> {
    let cells = input.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
    let width = input.lines().next().map_or(0, |line| line.len());
    let height = input.lines().count();
    if cells.len() != width * height {
        bail!("All lines must have the same length");
    }

    Ok(Grid {
        cells,
        width,
        height,
    })
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input04.txt");
    let grid = parse_input(input)?;

    let cell_iter = (0..grid.width as isize).cartesian_product(0..grid.height as isize);

    let part_a = cell_iter
        .clone()
        .map(|pos| grid.count_xmas_lines_at(pos))
        .sum::<usize>();
    println!("Day 04, part A: {part_a}");
    assert_eq!(part_a, 2521);

    let part_b = cell_iter.filter(|pos| grid.has_xmas_x_at(*pos)).count();
    println!("Day 04, part B: {part_b}");
    assert_eq!(part_b, 1912);

    Ok(())
}
