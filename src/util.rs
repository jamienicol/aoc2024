use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    sequence::pair,
    IResult,
};
use std::{ops::Neg, str::FromStr};

pub fn parse_unsigned<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse::<T>())(input)
}

pub fn parse_signed<T: FromStr + Neg<Output = T>>(input: &str) -> IResult<&str, T> {
    map(
        pair(opt(char('-')), map_res(digit1, |s: &str| s.parse::<T>())),
        |(minus, num): (Option<char>, T)| match minus {
            Some('-') => num.neg(),
            None => num,
            Some(_) => unreachable!(),
        },
    )(input)
}
