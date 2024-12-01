use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}
