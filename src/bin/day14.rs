use anyhow::{Ok, Result};
use aoc2024::util::parse_signed;
use nom::{
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::{all_consuming, map, opt},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};
use raqote::{Color, DrawOptions, DrawTarget};

#[derive(Clone)]
struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    all_consuming(terminated(
        separated_list1(
            newline,
            map(
                pair(
                    preceded(
                        tag("p="),
                        separated_pair(parse_signed, char(','), parse_signed),
                    ),
                    preceded(
                        tag(" v="),
                        separated_pair(parse_signed, char(','), parse_signed),
                    ),
                ),
                |(pos, velocity)| Robot { pos, velocity },
            ),
        ),
        opt(newline),
    ))(input)
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn tick(robots: Vec<Robot>) -> Vec<Robot> {
    robots
        .into_iter()
        .map(|robot| {
            let pos = (
                (robot.pos.0 + robot.velocity.0).rem_euclid(WIDTH),
                (robot.pos.1 + robot.velocity.1).rem_euclid(HEIGHT),
            );
            Robot {
                pos,
                velocity: robot.velocity,
            }
        })
        .collect()
}

fn count_quadrants(robots: &[Robot]) -> [usize; 4] {
    robots
        .iter()
        .fold([0, 0, 0, 0], |[tl, tr, bl, br], robot| match robot.pos {
            (x, y) if x < WIDTH / 2 && y < HEIGHT / 2 => [tl + 1, tr, bl, br],
            (x, y) if x > WIDTH / 2 && y < HEIGHT / 2 => [tl, tr + 1, bl, br],
            (x, y) if x < WIDTH / 2 && y > HEIGHT / 2 => [tl, tr, bl + 1, br],
            (x, y) if x > WIDTH / 2 && y > HEIGHT / 2 => [tl, tr, bl, br + 1],
            _ => [tl, tr, bl, br],
        })
}

#[allow(dead_code)]
fn draw_robots(robots: &[Robot]) -> DrawTarget {
    let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);
    dt.clear(Color::new(255, 0, 0, 0).into());
    let mut grid = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        dt.fill_rect(
            robot.pos.0 as f32,
            robot.pos.1 as f32,
            1.0,
            1.0,
            &Color::new(255, 0, 255, 0).into(),
            &DrawOptions::default(),
        );
        grid[robot.pos.1 as usize][robot.pos.0 as usize] = '#';
    }
    dt
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input14.txt");
    let robots = parse_input(input)?.1;

    let part_a = count_quadrants(&(0..100).fold(robots.clone(), |robots, _| tick(robots)))
        .into_iter()
        .product::<usize>();
    println!("Day 14, Part A: {}", part_a);
    assert_eq!(part_a, 214109808);

    // Save an image of the first 10000 robot arrangements
    // (0..10000).fold(robots, |robots, i| {
    //     draw_robots(&robots)
    //         .write_png(format!("day14_{:05}.png", i))
    //         .unwrap();
    //     tick(robots)
    // });

    // Looking at the images, we can see that usually the robots are randomly spread out.
    // However, every 101 frames they mostly appear in a vertical line, first occuring at
    // frame 12. And every 103 frames they mostly appear in a horizontal line, first
    // occuring at frame 65. The tree will occur when these two patterns overlap.
    let mut vertical = (11..).step_by(101).peekable();
    let mut horizontal = (65..).step_by(103).peekable();
    let part_b = loop {
        let v = vertical.peek().unwrap();
        let h = horizontal.peek().unwrap();
        match v.cmp(h) {
            std::cmp::Ordering::Less => vertical.next(),
            std::cmp::Ordering::Greater => horizontal.next(),
            std::cmp::Ordering::Equal => break *v,
        };
    };
    println!("Day 14, Part B: {:?}", part_b);
    assert_eq!(part_b, 7687);

    Ok(())
}
