use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::collections::HashMap;

trait KeyPad {
    fn pos(&self) -> (i32, i32);
    fn default_key() -> Self;
    fn blank_space() -> (i32, i32);
}

#[derive(Debug, Clone, Copy)]
enum NumPad {
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    Zero,
    A,
}

#[derive(Debug, Clone, Copy)]
enum DirPad {
    Up,
    A,
    Left,
    Down,
    Right,
}

impl KeyPad for DirPad {
    fn pos(&self) -> (i32, i32) {
        match *self {
            DirPad::Up => (1, 0),
            DirPad::A => (2, 0),
            DirPad::Left => (0, 1),
            DirPad::Down => (1, 1),
            DirPad::Right => (2, 1),
        }
    }

    fn default_key() -> Self {
        DirPad::A
    }

    fn blank_space() -> (i32, i32) {
        (0, 0)
    }
}

fn parse_input(input: &str) -> Result<Vec<(usize, Vec<NumPad>)>> {
    input
        .lines()
        .map(|line| {
            let num = line.strip_suffix('A').context("No A")?.parse::<usize>()?;
            let code = line
                .chars()
                .map(|c| match c {
                    '0' => Ok(NumPad::Zero),
                    '1' => Ok(NumPad::One),
                    '2' => Ok(NumPad::Two),
                    '3' => Ok(NumPad::Three),
                    '4' => Ok(NumPad::Four),
                    '5' => Ok(NumPad::Five),
                    '6' => Ok(NumPad::Six),
                    '7' => Ok(NumPad::Seven),
                    '8' => Ok(NumPad::Eight),
                    '9' => Ok(NumPad::Nine),
                    'A' => Ok(NumPad::A),
                    _ => Err(anyhow!("Invalid char: {}", c)),
                })
                .try_collect()?;
            Ok((num, code))
        })
        .try_collect()
}

impl KeyPad for NumPad {
    fn pos(&self) -> (i32, i32) {
        match *self {
            NumPad::Seven => (0, 0),
            NumPad::Eight => (1, 0),
            NumPad::Nine => (2, 0),
            NumPad::Four => (0, 1),
            NumPad::Five => (1, 1),
            NumPad::Six => (2, 1),
            NumPad::One => (0, 2),
            NumPad::Two => (1, 2),
            NumPad::Three => (2, 2),
            NumPad::Zero => (1, 3),
            NumPad::A => (2, 3),
        }
    }

    fn default_key() -> Self {
        NumPad::A
    }

    fn blank_space() -> (i32, i32) {
        (0, 3)
    }
}

fn sequences_for_buttons<T: KeyPad>(a: T, b: T) -> Vec<Vec<DirPad>> {
    let a = a.pos();
    let b = b.pos();
    let mut sequences = if a == b {
        vec![vec![]]
    } else if a.1 == b.1 {
        if a.0 < b.0 {
            vec![vec![DirPad::Right; (b.0 - a.0) as usize]]
        } else {
            vec![vec![DirPad::Left; (a.0 - b.0) as usize]]
        }
    } else if a.0 == b.0 {
        if a.1 < b.1 {
            vec![vec![DirPad::Down; (b.1 - a.1) as usize]]
        } else {
            vec![vec![DirPad::Up; (a.1 - b.1) as usize]]
        }
    } else {
        let h = if a.0 < b.0 {
            vec![DirPad::Right; (b.0 - a.0) as usize]
        } else {
            vec![DirPad::Left; (a.0 - b.0) as usize]
        };
        let v = if a.1 < b.1 {
            vec![DirPad::Down; (b.1 - a.1) as usize]
        } else {
            vec![DirPad::Up; (a.1 - b.1) as usize]
        };

        let mut h_then_v = h.clone();
        h_then_v.extend(&v);
        let mut v_then_h = v;
        v_then_h.extend(h);
        if a.0 == T::blank_space().0 && b.1 == T::blank_space().1 {
            vec![h_then_v]
        } else if a.1 == T::blank_space().1 && b.0 == T::blank_space().0 {
            vec![v_then_h]
        } else {
            vec![h_then_v, v_then_h]
        }
    };

    sequences.iter_mut().for_each(|sequence| {
        sequence.push(DirPad::A);
    });
    sequences
}

type Cache = HashMap<((i32, i32), (i32, i32), usize), usize>;

fn num_dpad_presses_for_buttons<T: KeyPad + Copy>(
    a: T,
    b: T,
    num_robots: usize,
    cache: &mut Cache,
) -> usize {
    if let Some(presses) = cache.get(&(a.pos(), b.pos(), num_robots)) {
        return *presses;
    }

    let sequences = sequences_for_buttons(a, b);
    let presses = match num_robots {
        0 => sequences.into_iter().map(|seq| seq.len()).min().unwrap(),
        _ => sequences
            .into_iter()
            .map(|seq| num_dpad_presses_for_sequence(&seq, num_robots - 1, cache))
            .min()
            .unwrap(),
    };
    cache.insert((a.pos(), b.pos(), num_robots), presses);
    presses
}

fn num_dpad_presses_for_sequence<T: KeyPad + Copy>(
    sequence: &[T],
    num_robots: usize,
    cache: &mut Cache,
) -> usize {
    std::iter::once(&T::default_key())
        .chain(sequence)
        .tuple_windows()
        .map(|(a, b)| num_dpad_presses_for_buttons(*a, *b, num_robots, cache))
        .sum()
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input21.txt");
    let codes = parse_input(input)?;

    let mut cache = HashMap::default();
    let part_a = codes
        .clone()
        .into_iter()
        .map(|(num, code)| num * num_dpad_presses_for_sequence(&code, 2, &mut cache))
        .sum::<usize>();
    println!("Day 21, part A: {part_a}");
    assert_eq!(part_a, 188398);

    cache.clear();
    let part_b = codes
        .into_iter()
        .map(|(num, code)| num * num_dpad_presses_for_sequence(&code, 25, &mut cache))
        .sum::<usize>();
    println!("Day 21, part A: {part_b}");
    assert_eq!(part_b, 230049027535970);

    Ok(())
}
