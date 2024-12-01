use nom::{character::complete::digit1, combinator::map_res, IResult};
use std::str::FromStr;

pub fn parse_unsigned<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse::<T>())(input)
}
