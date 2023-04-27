use nom::character::complete::{alpha1, alphanumeric1, digit1, space0};
use nom::combinator::map_res;
use nom::branch::alt;
use nom::sequence::tuple;
use nom::bytes::complete::take_until;
use nom::IResult;
use crate::address_components::*;

pub fn parse_address_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

pub fn parse_pre_directional(input: &str) -> IResult<&str, Option<StreetNamePreDirectional>> {
    let (rem, _) = space0(input)?;
    let (rem, result) = alpha1(rem)?;
    let predir = match_abbreviated_pre_directional(result);
    Ok((rem, predir))
}

pub fn parse_post_type(input: &str) -> IResult<&str, Option<StreetNamePostType>> {
    let (rem, _) = space0(input)?;
    let (rem, result) = alpha1(rem)?;
    let post_type = match_mixed_post_type(result);
    Ok((rem, post_type))
}

pub fn parse_street_name(input: &str) -> IResult<&str, (Option<StreetNamePreDirectional>, &str)> {
    let (remaining, predir) = parse_pre_directional(input)?;
    match predir {
        Some(dir) => {
            let (rem, _) = space0(remaining)?;
            let (post, name) = alphanumeric1(rem)?;
            Ok((post, (Some(dir), name)))
        }
        None => {
            let (rem, _) = space0(input)?;
            let (post, name) = alphanumeric1(rem)?;
            Ok((post, (None, name)))
        }
    }
}

pub fn parse_complete_street_name(input: &str) -> IResult<&str, (Option<StreetNamePreDirectional>, &str, Option<StreetNamePostType>)> {
    let (rem, (predir, name)) = parse_street_name(input)?;
    let (rem, post) = parse_post_type(rem)?;
    Ok((rem, (predir, name, post)))
}

pub fn single_word(input: &str) -> IResult<&str, &str> {
    let (rem, _) = space0(input)?;
    alphanumeric1(rem)
}

pub fn is_post_type(input: &str) -> IResult<&str, bool> {
    let (rem, post) = parse_post_type(input)?;
    let test = match post {
        Some(_) => true,
        None => false,
    };
    Ok((rem, test))
}

pub fn multi_word(input: &str) -> IResult<&str, Vec<&str>> {
    let (rem, start) = single_word(input)?;
    let (_, test) = is_post_type(rem)?;
    let mut name = vec![start];
    let mut cond = test;
    while !cond {
        let (rem, next) = single_word(rem)?;
        name.push(next);
        let (_, test) = is_post_type(rem)?;
        cond = test;
    }
    Ok((rem, name))
}
