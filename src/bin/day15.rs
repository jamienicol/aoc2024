use anyhow::{anyhow, bail, ensure, Context, Result};
use itertools::Itertools;

type Pos = (isize, isize);

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    SmallBox,
    BigBoxLeft,
    BigBoxRight,
}

#[derive(Clone)]
struct Map {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
}

impl Map {
    fn get(&self, pos: Pos) -> &Tile {
        &self.tiles[(pos.1 * self.width + pos.0) as usize]
    }

    fn get_mut(&mut self, pos: Pos) -> &mut Tile {
        &mut self.tiles[(pos.1 * self.width + pos.0) as usize]
    }

    #[allow(dead_code)]
    fn print(&self, robot: Pos) {
        for y in 0..self.height {
            for x in 0..self.width {
                match (robot == (x, y), self.get((x, y))) {
                    (true, _) => print!("@"),
                    (_, Tile::Empty) => print!("."),
                    (_, Tile::Wall) => print!("#"),
                    (_, Tile::SmallBox) => print!("O"),
                    (_, Tile::BigBoxLeft) => print!("["),
                    (_, Tile::BigBoxRight) => print!("]"),
                }
            }
            println!();
        }
    }

    fn score(&self) -> isize {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| matches!(tile, Tile::SmallBox | Tile::BigBoxLeft))
            .map(|(i, _)| 100 * (i as isize / self.width) + i as isize % self.width)
            .sum()
    }
}

fn parse_input(input: &str) -> Result<(Pos, Map, Vec<Pos>)> {
    let (map, directions) = input
        .split_once("\n\n")
        .context("No separate map and directions sections")?;

    let width = map.lines().next().context("Empty input")?.len() as isize;
    let height = map.lines().count() as isize;
    let (robot, tiles) = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .try_fold((None, Vec::new()), |(mut robot, mut tiles), (pos, c)| {
            match c {
                '#' => {
                    tiles.push(Tile::Wall);
                }
                'O' => {
                    tiles.push(Tile::SmallBox);
                }
                '@' => {
                    ensure!(robot.is_none(), "Multiple robots found");
                    tiles.push(Tile::Empty);
                    robot = Some(pos);
                }
                '.' => {
                    tiles.push(Tile::Empty);
                }
                _ => bail!("Invalid character in map: {}", c),
            }
            Ok((robot, tiles))
        })?;
    let robot = robot.context("Robot position not found")?;

    let directions: Vec<Pos> = directions
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '<' => Ok((-1, 0)),
            '>' => Ok((1, 0)),
            '^' => Ok((0, -1)),
            'v' => Ok((0, 1)),
            _ => Err(anyhow!("Invalid direction: {}", c)),
        })
        .try_collect()?;

    Ok((
        robot,
        Map {
            width,
            height,
            tiles,
        },
        directions,
    ))
}

fn resize_map(robot: Pos, mut map: Map) -> (Pos, Map) {
    let tiles = map
        .tiles
        .drain(..)
        .flat_map(|tile| match tile {
            Tile::Empty => [Tile::Empty, Tile::Empty],
            Tile::Wall => [Tile::Wall, Tile::Wall],
            Tile::SmallBox => [Tile::BigBoxLeft, Tile::BigBoxRight],
            Tile::BigBoxLeft | Tile::BigBoxRight => unimplemented!(),
        })
        .collect();

    (
        (robot.0 * 2, robot.1),
        Map {
            width: map.width * 2,
            height: map.height,
            tiles,
        },
    )
}

fn can_move(map: &Map, pos: Pos, direction: Pos) -> Option<Vec<Pos>> {
    let mut boxes = Vec::new();
    let mut open = Vec::new();
    open.push(pos);

    while let Some(next) = open.pop() {
        let tile = *map.get(next);
        match tile {
            Tile::Wall => return None,
            Tile::Empty => {}
            Tile::SmallBox | Tile::BigBoxLeft | Tile::BigBoxRight => {
                if !boxes.contains(&next) {
                    boxes.push(next);
                    match direction {
                        (-1, 0) | (1, 0) => {
                            open.push((next.0 + direction.0, next.1));
                        }
                        (0, -1) | (0, 1) => {
                            open.push((next.0, next.1 + direction.1));
                            if matches!(tile, Tile::BigBoxLeft) {
                                open.push((next.0 + 1, next.1));
                            } else if matches!(tile, Tile::BigBoxRight) {
                                open.push((next.0 - 1, next.1));
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    Some(boxes)
}

fn move_robot(mut robot: Pos, mut map: Map, direction: Pos) -> (Pos, Map) {
    let to_move = can_move(
        &map,
        (robot.0 + direction.0, robot.1 + direction.1),
        direction,
    );

    if let Some(boxes) = to_move {
        robot = (robot.0 + direction.0, robot.1 + direction.1);
        let old_map = map.clone();
        for b in &boxes {
            if !boxes.contains(&(b.0 - direction.0, b.1 - direction.1)) {
                *map.get_mut(*b) = Tile::Empty;
            }
            *map.get_mut((b.0 + direction.0, b.1 + direction.1)) = *old_map.get(*b);
        }
    }

    (robot, map)
}

fn main() -> Result<()> {
    let input = include_str!("../../res/input15.txt");
    let (robot, map, directions) = parse_input(input)?;

    let part_a = directions
        .iter()
        .fold((robot, map.clone()), |(robot, map), dir| {
            move_robot(robot, map, *dir)
        })
        .1
        .score();
    println!("Day 15, part A: {}", part_a);
    assert_eq!(part_a, 1465152);

    let part_b = directions
        .iter()
        .fold(resize_map(robot, map), |(robot, map), dir| {
            move_robot(robot, map, *dir)
        })
        .1
        .score();
    println!("Day 15, part B: {}", part_b);
    assert_eq!(part_b, 1511259);

    Ok(())
}
