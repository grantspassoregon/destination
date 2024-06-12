//! The `parser` module contains functions for parsing unstructured text into address components.
use crate::prelude::{
    match_mixed_post_type, match_mixed_pre_directional, match_mixed_subaddress_type,
    PartialAddress, StreetNamePostType, StreetNamePreDirectional, SubaddressType,
};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, digit1, space0};
use nom::character::is_alphanumeric;
use nom::combinator::{map_res, opt};
use nom::IResult;
use serde::de::{Deserialize, Deserializer};

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

/// The `multi_word` function expects at least one word, and then tests the remainder for the
/// presence of a street name post type.  If a street name post type is not present, the function will
/// continue iterating over words in the input until the next word parses as a street name
/// post type, when it will stop and return.  Josephine County has a couple instances of streets with multiple
/// post type values (e.g. Azalea Drive Cutoff).  
/// According to FGDC guidelines, multiple post type values can either be parsed as part of the street name
/// component or as multiple values of the street name post type, and we have chosen the latter strategy.
/// TODO: Maybe should be renamed to parse_street_name_elements, multi_word is too vague.
pub fn multi_word(input: &str) -> IResult<&str, Vec<&str>> {
    // Take the first set of alphanumeric characters.
    let (rem, start) = single_word(input)?;
    // Determine if the following word is a post type.
    let (_, test) = is_post_type(rem)?;
    // A vector to hold street name parts.
    let mut name = vec![start];
    tracing::trace!("Name is {:#?}", &name);
    // If no post type is found, we return the input, so we back it up here.
    let mut remaining = rem;
    tracing::trace!("Remaining is {:#?}", &rem);
    // True if a post type is already present, false is not.
    let mut cond = test;
    tracing::trace!("Starting condition is {:#?}", &test);
    // While no post type is present
    while !cond {
        // Take the next set of alphanumeric characters.
        let (rem, next) = single_word(remaining)?;
        // Since it comes after the beginning of the street name, and before a post type
        // declaration, additional words must be part of a compound street name.
        name.push(next);
        tracing::trace!("Name is {:#?}", &name);
        // Check if the next word is a post type.
        let (_, test) = is_post_type(rem)?;
        // Update the return input, since we have processed a word
        remaining = rem;
        // Update the test condition based on whether the next word is a post type
        cond = test;
    }
    // Note we do not parse the post type, just stop when we hit it.  Here we just return the name
    // and the remainder, including the post type.
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
///
/// TODO: When parsing a partial address, the street type may not be included, so must be optional.
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
    // Take a predirectional if present.
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

/// The `parse_subaddress_element` function attempts to pull a single subaddress identifier from
/// the input string, removing any preceding pound or ampersand.
pub fn parse_subaddress_element(input: &str) -> IResult<&str, Option<&str>> {
    // strip preceding whitespace
    let (next, _) = space0(input)?;
    let mut element = next;
    let mut remaining = "";
    // if the input contains a space
    // take the word before the space (element)
    let (rem, next) = opt(take_until(" "))(next)?;
    if let Some(value) = next {
        element = value;
        // update the remaining input
        remaining = rem;
    }
    // strip any preceding pound or ampersand
    let (element, _) = opt(alt((tag("#"), tag("&"))))(element)?;
    match element {
        // if nothing is left, return remaining
        "" => Ok((remaining, None)),
        // return the identifier (value)
        value => Ok((remaining, Some(value))),
    }
}

/// The `parse_subaddress_elements` functions attempts to take one or more subaddress identifiers
/// from the input, removing any preceding pound or ampersand.
pub fn parse_subaddress_elements(input: &str) -> IResult<&str, Vec<&str>> {
    // vector of subadress ids
    let mut elements = Vec::new();
    let (rem, next) = parse_subaddress_element(input)?;
    let mut remaining = rem;
    // if a subaddress id is present, push it to id vector
    if let Some(value) = next {
        elements.push(value);
    }
    // repeat until remainder is empty
    while !remaining.is_empty() {
        let (rem, next) = parse_subaddress_element(remaining)?;
        if let Some(value) = next {
            elements.push(value);
        }
        remaining = rem;
    }
    Ok((remaining, elements))
}

/// The `parse_subaddress_identifiers` functions attempts to find one or more subaddress
/// identifiers using the [`parse_subaddress_elements`] function.
pub fn parse_subaddress_identifiers(input: &str) -> IResult<&str, Option<Vec<&str>>> {
    let mut subaddress = None;
    // Here we assume a comma delimits the street address from the postal community.
    // TODO: Make robust against cases where multiple subaddress ids are comma delimited.
    let (rem, _) = opt(tag("."))(input)?;
    let (rem, next) = opt(take_until(","))(rem)?;
    let mut remaining = rem;
    match next {
        // Try parsing the input before the comma as subaddress ids.
        Some(value) => {
            let (_, elements) = parse_subaddress_elements(value)?;
            if !elements.is_empty() {
                subaddress = Some(elements);
            }
        }
        // If there are no ids before the comma, try the input after
        None => {
            let (rem, elements) = parse_subaddress_elements(remaining)?;
            if !elements.is_empty() {
                // return input that does not parse as ids
                remaining = rem;
                subaddress = Some(elements);
            }
        }
    }
    Ok((remaining, subaddress))
}

/// The `parse_address` function attempts to read the complete address and parse it into its
/// constituent components.
pub fn parse_address(input: &str) -> IResult<&str, PartialAddress> {
    // this struct will hold the values of the parsed address components
    let mut address = PartialAddress::default();
    // attempt to read the complete address number
    let (rem, address_number) = parse_address_number(input)?;
    tracing::trace!("Address number: {}", &address_number);
    // we avoid an if let clause because address_number is none if not present.
    address.set_address_number(address_number);
    let (rem, suffix) = parse_address_number_suffix(rem)?;
    tracing::trace!("Address number suffix: {:#?}", &suffix);
    address.set_address_number_suffix(suffix);
    // attempt to read the complete street name
    let (rem, (predir, name, post_type)) = parse_complete_street_name(rem)?;
    // can't we remove this if let like above?
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
    address.set_street_name(&street_name.to_uppercase());
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

/// The parse_phone_number function expects a phone number that may optionally include parenthesis
/// around the area code, and the use of periods or a hyphen as a separator.
pub fn parse_phone_number(input: &str) -> IResult<&str, String> {
    // Strip a leading parenthesis if present.
    let (rem, _) = opt(tag("("))(input)?;
    // Takes one or more numbers, either the prefix or the whole sequence.
    let (rem, prefix) = nom::character::complete::digit1(rem)?;
    // Strip the closing parenthesis or a period if present.
    let (rem, _) = opt(alt((tag(")"), tag("."))))(rem)?;
    // Strip any whitespace.
    let (rem, _) = space0(rem)?;
    // Takes one or more numbers, targeting the first three of the primary seven.
    let (rem, first) = nom::character::complete::digit1(rem)?;
    // Strip any whitespace used before the separator.
    let (rem, _) = space0(rem)?;
    // Remove a hyphen or dot separator.
    let (rem, _) = opt(alt((tag("-"), tag("."))))(rem)?;
    // Strip any whitespace used after the separator.
    let (rem, _) = space0(rem)?;
    // Takes one or more numbers, targeting the last four of the primary seven.
    let (rem, second) = nom::character::complete::digit1(rem)?;
    // Empty string to receive digits pulled from input.
    let mut phone = String::new();
    // Push digits to the string.
    phone.push_str(prefix);
    phone.push_str(first);
    phone.push_str(second);
    Ok((rem, phone))
}

/// The `deserialize_phone_number` function deserializes text input into an integer representation
/// of a phone number.  Strips parentheses around the area code, as well as periods or a hyphen
/// used as a separator.
pub fn deserialize_phone_number<'de, D: Deserializer<'de>>(de: D) -> Result<Option<i64>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    let mut res = None;
    // First we make sure the string contains only digits.
    if let Ok((_, text)) = parse_phone_number(intermediate) {
        // Then we parse the string to an integer.
        if let Ok(num) = text.parse::<i64>() {
            res = Some(num);
        }
    }
    Ok(res)
}
