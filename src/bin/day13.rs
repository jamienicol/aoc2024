use anyhow::Result;
use aoc2024::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map, opt},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

struct Game {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    all_consuming(terminated(
        separated_list1(
            newline,
            map(
                tuple((
                    delimited(
                        tag("Button A: X+"),
                        separated_pair(parse_unsigned, tag(", Y+"), parse_unsigned),
                        opt(newline),
                    ),
                    delimited(
                        tag("Button B: X+"),
                        separated_pair(parse_unsigned, tag(", Y+"), parse_unsigned),
                        opt(newline),
                    ),
                    delimited(
                        tag("Prize: X="),
                        separated_pair(parse_unsigned, tag(", Y="), parse_unsigned),
                        opt(newline),
                    ),
                )),
                |(button_a, button_b, prize)| Game {
                    button_a,
                    button_b,
                    prize,
                },
            ),
        ),
        opt(newline),
    ))(input)
}

fn determinant(mat: [usize; 4]) -> isize {
    mat[0] as isize * mat[3] as isize - mat[1] as isize * mat[2] as isize
}

fn solve(game: &Game) -> Option<usize> {
    let det = determinant([
        game.button_a.0,
        game.button_b.0,
        game.button_a.1,
        game.button_b.1,
    ]);
    if det == 0 {
        return None;
    }
    let det_a = determinant([game.prize.0, game.prize.1, game.button_b.0, game.button_b.1]);
    let det_b = determinant([game.button_a.0, game.button_a.1, game.prize.0, game.prize.1]);
    if det_a % det != 0 || det_b % det != 0 {
        return None;
    }
    Some((3 * det_a / det + det_b / det) as usize)
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input13.txt");
    let games = parse_input(input)?.1;

    let part_a = games.iter().filter_map(solve).sum::<usize>();
    println!("Day 13, part A: {part_a}");
    assert_eq!(part_a, 33427);

    let games = games
        .into_iter()
        .map(|game| Game {
            button_a: game.button_a,
            button_b: game.button_b,
            prize: (game.prize.0 + 10000000000000, game.prize.1 + 10000000000000),
        })
        .collect_vec();

    let part_b = games.iter().filter_map(solve).sum::<usize>();
    println!("Day 13, part B: {part_b}");
    assert_eq!(part_b, 91649162972270);

    Ok(())
}
