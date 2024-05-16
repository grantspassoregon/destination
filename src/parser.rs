use crate::address::*;
use crate::address_components::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, digit1, space0};
use nom::character::is_alphanumeric;
use nom::combinator::{map_res, opt};
use nom::IResult;

/// The `parse_address_number` function expects one or more numeric digits, returned as an i64 value.
pub fn parse_address_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

/// The `parse_address_number_suffix` function peeks at the next value in the input, checking if
/// the second character in the string is non-alphanumeric.  Since address number suffixes in Grants Pass
/// take values of either `1/2` or `3/4`, the second character will be `/`, which is not a valid
/// character for any components of the street name.  So if a `/` is present, it must be an address
/// number suffix, and this function will parse and return it.  If no address number suffix is
/// present, the function returns `None`.
///
/// Note this approach is not valid for address number suffixes that do not conform to the
/// indicated pattern.
pub fn parse_address_number_suffix(input: &str) -> IResult<&str, Option<&str>> {
    let (remaining, _) = space0(input)?;
    let (rem, suffix) = take_until(" ")(remaining)?;
    if suffix.len() > 1 {
        match !is_alphanumeric(suffix.as_bytes()[1]) {
            true => Ok((rem, Some(suffix))),
            false => Ok((remaining, None)),
        }
    } else {
        Ok((remaining, None))
    }
}

/// The `parse_pre_directional` function attempts to parse the next word in the input to a
/// [`StreetNamePreDirectional`].  If a [`StreetNamePreDirectional`] is present, the function
/// returns the value and the remainder of the input.  If not present, the function returns `None`
/// as the directional and gives the full input back.
pub fn parse_pre_directional(input: &str) -> IResult<&str, Option<StreetNamePreDirectional>> {
    let (rem, _) = space0(input)?;
    let (rem, result) = alpha1(rem)?;
    let predir = match_mixed_pre_directional(result);
    match predir {
        Some(_) => Ok((rem, predir)),
        None => Ok((input, predir)),
    }
}

/// The `parse_post_type` function attempts to parse the next word in the input as a
/// [`StreetNamePostType`] value.  Since the street name post type is a required field for Grants
/// Pass addresses, there is no need to peek and conditionally return.  If the street name post
/// type evaluates to None, this is not a hard error, because the post type is not a required field
/// according to the FGDC standard, and partner agencies such as ECSO may have valid addresses
/// without a street name post type (e.g. "Broadway").
pub fn parse_post_type(input: &str) -> IResult<&str, Option<StreetNamePostType>> {
    let (rem, _) = space0(input)?;
    let (rem, result) = alpha1(rem)?;
    tracing::trace!("Post type check on {:#?}", &result);
    let post_type = match_mixed_post_type(result);
    Ok((rem, post_type))
}

/// The `single_word` function removes any preceding whitespace and parses the first group of
/// alphanumeric characters, returning the result as a "single word".
pub fn single_word(input: &str) -> IResult<&str, &str> {
    let (rem, _) = space0(input)?;
    alphanumeric1(rem)
}

/// The `is_post_type` function returns true if the input parses to a valid [`StreetNamePostType`].
/// Peeks at the data without consuming it.
pub fn is_post_type(input: &str) -> IResult<&str, bool> {
    tracing::trace!("Calling is_post_type");
    let (rem, post) = parse_post_type(input)?;
    tracing::trace!("Post type is {:#?}", &post);
    Ok((rem, post.is_some()))
}

/// The `multi_word` function expect at least one word, and then tests the remainder for the
/// presence of a street name post type.  If a street name post type is not present, the function will
/// continue iterating over words in the input until the next word parses as a street name
/// post type, when it will stop and return.  Josephine County has a couple instances of streets with multiple post type values (e.g. Azalea Drive Cutoff).  According to FGDC guidelines, multiple post type values can either be parsed as part of the street name component or as multiple values of the street name post type, and we have chosen the latter strategy.
pub fn multi_word(input: &str) -> IResult<&str, Vec<&str>> {
    let (rem, start) = single_word(input)?;
    let (_, test) = is_post_type(rem)?;
    let mut name = vec![start];
    tracing::trace!("Name is {:#?}", &name);
    let mut remaining = rem;
    tracing::trace!("Remaining is {:#?}", &rem);
    let mut cond = test;
    tracing::trace!("Starting condition is {:#?}", &test);
    while !cond {
        let (rem, next) = single_word(remaining)?;
        name.push(next);
        tracing::trace!("Name is {:#?}", &name);
        let (_, test) = is_post_type(rem)?;
        remaining = rem;
        cond = test;
    }
    Ok((remaining, name))
}

/// The `recursive_post_type` function expects one or more [`StreetNamePostType`] designations.
/// 
/// Originally I meant this function to call [`is_post_type`] and [`parse_post_type`] recursively.
/// Instead, there is no recursion here, we use a while loop to continue parsing street name post
/// types until failure, at which point the function returns the vector of parsed post types and
/// the remainder of the input.
pub fn recursive_post_type(input: &str) -> IResult<&str, Vec<StreetNamePostType>> {
    let mut post_type = Vec::new();
    let mut cond = true;
    let mut remaining = input;
    while cond {
        let (rem, post) = opt(single_word)(remaining)?;
        match post {
            Some(value) => {
                let val = match_mixed_post_type(value);
                match val {
                    Some(val_type) => {
                        post_type.push(val_type);
                        remaining = rem;
                    }
                    None => cond = false,
                }
            }
            None => cond = false,
        }
    }
    Ok((remaining, post_type))
}

/// The `parse_complete_street_name` function attempts to parse a complete street name from the
/// input.  First it tries to read the pre directional, then one or more street names.  If it
/// parses more than one post type, it returns the last post type in the post type field, and
/// appends the remainder to the street name.
///
/// TODO:  For streets with multiple post type values, such as Azalea Dr Cutoff, this will
/// incorrectly classify the post type and street name.
pub fn parse_complete_street_name(
    input: &str,
) -> IResult<
    &str,
    (
        Option<StreetNamePreDirectional>,
        Vec<&str>,
        StreetNamePostType,
    ),
> {
    let (rem, predir) = parse_pre_directional(input)?;
    tracing::trace!("Predir is {:#?}", &predir);
    let (name_rem, name) = multi_word(rem)?;
    let mut name = name;
    tracing::trace!("Name is {:#?}", &name);
    tracing::trace!("Remaining: {:#?}", &name_rem);
    let (rem, post_type) = recursive_post_type(name_rem)?;
    let post_len = post_type.len();
    let post = post_type[post_len - 1];
    tracing::trace!("Post type is {:#?}", &post);
    let mut post_type = post_type;
    if post_len > 1 {
        post_type = post_type[0..post_len - 1].to_vec();
        let mut remaining = name_rem;
        for _ in post_type {
            let (name_rem, next) = single_word(remaining)?;
            remaining = name_rem;
            name.push(next);
        }
    }
    tracing::trace!("Name is {:#?}", &name);
    Ok((rem, (predir, name, post)))
}

/// The `parse_subaddress_type` function attempts to find a word following the street name post
/// type and preceding the postal community.  If a word is present, and parses to a subaddress
/// type, the function will return the type and the remainder.  If no subaddress type is present,
/// the function will return the full input.
pub fn parse_subaddress_type(input: &str) -> IResult<&str, Option<SubaddressType>> {
    let (rem, next) = opt(single_word)(input)?;
    let subtype = if let Some(word) = next {
        match_mixed_subaddress_type(word)
    } else {
        None
    };
    match subtype {
        Some(_) => Ok((rem, subtype)),
        None => Ok((input, subtype)),
    }
}

pub fn parse_subaddress_element(input: &str) -> IResult<&str, Option<&str>> {
    let (next, _) = space0(input)?;
    let mut element = next;
    let mut remaining = "";
    let (rem, next) = opt(take_until(" "))(next)?;
    if let Some(value) = next {
        element = value;
        remaining = rem;
    }
    let (element, _) = opt(alt((tag("#"), tag("&"))))(element)?;
    match element {
        "" => Ok((remaining, None)),
        value => Ok((remaining, Some(value))),
    }
}

pub fn parse_subaddress_elements(input: &str) -> IResult<&str, Vec<&str>> {
    let mut elements = Vec::new();
    let (rem, next) = parse_subaddress_element(input)?;
    let mut remaining = rem;
    if let Some(value) = next {
        elements.push(value);
    }
    while !remaining.is_empty() {
        let (rem, next) = parse_subaddress_element(remaining)?;
        if let Some(value) = next {
            elements.push(value);
        }
        remaining = rem;
    }
    Ok((remaining, elements))
}

pub fn parse_subaddress_identifiers(input: &str) -> IResult<&str, Option<Vec<&str>>> {
    let mut subaddress = None;
    let (rem, next) = opt(take_until(","))(input)?;
    let mut remaining = rem;
    match next {
        Some(value) => {
            let (_, elements) = parse_subaddress_elements(value)?;
            if !elements.is_empty() {
                subaddress = Some(elements);
            }
        }
        None => {
            let (rem, elements) = parse_subaddress_elements(remaining)?;
            if !elements.is_empty() {
                remaining = rem;
                subaddress = Some(elements);
            }
        }
    }
    Ok((remaining, subaddress))
}

pub fn parse_address(input: &str) -> IResult<&str, PartialAddress> {
    let mut address = PartialAddress::default();
    let (rem, address_number) = parse_address_number(input)?;
    tracing::trace!("Address number: {}", &address_number);
    address.set_address_number(address_number);
    let (rem, suffix) = parse_address_number_suffix(rem)?;
    tracing::trace!("Address number suffix: {:#?}", &suffix);
    address.set_address_number_suffix(suffix);
    let (rem, (predir, name, post_type)) = parse_complete_street_name(rem)?;
    if let Some(value) = predir {
        address.set_pre_directional(&value)
    }
    let mut street_name = String::new();
    for (i, val) in name.iter().enumerate() {
        street_name.push_str(val);
        if name.len() > 1 && i < name.len() - 1 {
            street_name.push(' ');
        }
    }
    tracing::trace!("Street name: {:#?}", &street_name);
    address.set_street_name(&street_name);
    tracing::trace!("Street post type: {:#?}", &post_type);
    address.set_post_type(&post_type);
    let (rem, subtype) = parse_subaddress_type(rem)?;
    if let Some(value) = subtype {
        tracing::trace!("Subaddress type: {:#?}", &value);
        address.set_subaddress_type(&value);
    }
    let (rem, elements) = parse_subaddress_identifiers(rem)?;
    if let Some(value) = elements {
        let mut subaddress_identifier = String::new();
        for (i, val) in value.iter().enumerate() {
            subaddress_identifier.push_str(val);
            if value.len() > 1 && i < value.len() - 1 {
                subaddress_identifier.push(' ');
            }
        }
        tracing::trace!("Subaddress identifier: {:#?}", &subaddress_identifier);
        address.set_subaddress_identifier(&subaddress_identifier);
    }
    Ok((rem, address))
}

// pub fn deserialize_address<'de, D: Deserializer<'de>>(
//     de: D,
// ) -> Result<PartialAddress, D::Error> {
//     let intermediate = Deserialize::deserialize(de)?;
//     let (_, result) = parse_address(intermediate)?;
//     Ok(result)
// }
