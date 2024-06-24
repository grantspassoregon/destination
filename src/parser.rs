//! The `parser` module contains functions for parsing unstructured text into address components.
use crate::address_components::{StreetNamePreModifier, StreetSeparator};
use crate::prelude::{
    PartialAddress, PostalCommunity, State, StreetNamePostType, StreetNamePreDirectional,
    StreetNamePreType, SubaddressType,
};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{alpha1, alphanumeric1, digit1, space0};
use nom::character::is_alphanumeric;
use nom::combinator::{self, eof};
use nom::combinator::{map_res, opt};
use nom::IResult;
use serde::de::{Deserialize, Deserializer};

/// The `Parser` struct holds methods for parsing addresses.
#[derive(Debug, Copy, Clone)]
pub struct Parser;

impl Parser {
    /// The `address_number` function expects one or more numeric digits, returned as an i64 value.
    /// TODO: 2501-2503 address range should read as 2501 and discard remainder of range.
    pub fn address_number(input: &str) -> IResult<&str, Option<i64>> {
        // Strip preceding whitespace
        let (remaining, _) = space0(input)?;
        // Digit1 takes one or more digits.
        // Map the digits to str::parse to convert them from str to i64
        if let Ok((rem, num)) = combinator::map_res(
            complete::digit1::<&str, nom::error::Error<_>>,
            str::parse,
        )(remaining)
        {
            Ok((rem, Some(num)))
        } else {
            Ok((remaining, None))
        }
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
    pub fn address_number_suffix(input: &str) -> IResult<&str, Option<&str>> {
        // Strip preceding whitespace
        let (remaining, _) = space0(input)?;
        // Assumes no commas or dashes, space delimited from the street name
        if let Ok((rem, suffix)) = take_until::<&str, &str, nom::error::Error<_>>(" ")(remaining) {
            if suffix.len() > 1 {
                // The second character in the sequence is not alaphanumeric if it is a "/"
                let test = suffix.as_bytes()[1];
                // Screen out periods so it does not confuse suffixes with N.W. or S.E. patterns
                match !is_alphanumeric(test) && test != b'.' {
                    true => Ok((rem, Some(suffix))),
                    // If not a suffix, return the remainder before trying to parse the suffix
                    false => Ok((remaining, None)),
                }
            } else {
                Ok((remaining, None))
            }
        } else {
            // Cannot use question mark because finding none is not an error to bubble up.
            Ok((remaining, None))
        }
    }

    /// The `parse_pre_directional` function attempts to parse the next word in the input to a
    /// [`StreetNamePreDirectional`].  If a [`StreetNamePreDirectional`] is present, the function
    /// returns the value and the remainder of the input.  If not present, the function returns `None`
    /// as the directional and gives the full input back.
    pub fn pre_directional(input: &str) -> IResult<&str, Option<StreetNamePreDirectional>> {
        // Strip preceding whitespace.
        let (rem, _) = space0(input)?;
        // Take one or more alphabetic character.
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(rem) {
            tracing::trace!("Predir read: {}", result);
            // Strip trailing period after directional.
            let (rem, _) = opt(tag("."))(rem)?;
            // Strip preceding whitespace on next word.
            let (rem, _) = space0(rem)?;
            // Save remainder input if next word does not parse as directional.
            let remaining = rem;
            // Match against valid directional values.
            let predir = StreetNamePreDirectional::match_mixed(result);
            match predir {
                // If some, return the remainder after parsing the directional.
                Some(value) => match value {
                    // If North or South, need to check for trailing W or E.
                    StreetNamePreDirectional::NORTH => {
                        if let Ok((rem, res)) = alpha1::<&str, nom::error::Error<_>>(rem) {
                            let trailing = StreetNamePreDirectional::match_mixed(res);
                            tracing::trace!("Trailing: {:?}", trailing);
                            tracing::trace!("Remaining: {}", rem);
                            match trailing {
                                // Check if next word is West or East.
                                Some(second) => match second {
                                    // Compound directional found.
                                    StreetNamePreDirectional::EAST => {
                                        // Strip trailing period after directional.
                                        let (rem, _) = opt(tag("."))(rem)?;
                                        Ok((rem, Some(StreetNamePreDirectional::NORTHEAST)))
                                    }
                                    StreetNamePreDirectional::WEST => {
                                        let (rem, _) = opt(tag("."))(rem)?;
                                        Ok((rem, Some(StreetNamePreDirectional::NORTHWEST)))
                                    }
                                    // Next word is a directional, but not a valid one, return "remaining" instead
                                    // of "rem" with the value of the original directional found.
                                    other => {
                                        tracing::trace!(
                                            "Unexpected directional encountered: {}",
                                            other
                                        );
                                        Ok((remaining, Some(value)))
                                    }
                                },
                                // No additional directional found, return first value of
                                // riectional and original "remaining" instead of "rem".
                                None => Ok((remaining, Some(value))),
                            }
                        } else {
                            // No additional word found, return first value of
                            Ok((remaining, Some(value)))
                        }
                    }
                    StreetNamePreDirectional::SOUTH => {
                        if let Ok((rem, res)) = alpha1::<&str, nom::error::Error<_>>(rem) {
                            let trailing = StreetNamePreDirectional::match_mixed(res);
                            tracing::trace!("Trailing: {:?}", trailing);
                            tracing::trace!("Remaining: {}", rem);
                            match trailing {
                                // Check if next word is West or East.
                                Some(second) => match second {
                                    // Compound directional found.
                                    StreetNamePreDirectional::EAST => {
                                        // Strip trailing period after directional.
                                        let (rem, _) = opt(tag("."))(rem)?;
                                        Ok((rem, Some(StreetNamePreDirectional::SOUTHEAST)))
                                    }
                                    StreetNamePreDirectional::WEST => {
                                        let (rem, _) = opt(tag("."))(rem)?;
                                        Ok((rem, Some(StreetNamePreDirectional::SOUTHWEST)))
                                    }
                                    // Next word is a directional, but not a valid one, return "remaining" instead
                                    // of "rem" with the value of the original directional found.
                                    other => {
                                        tracing::trace!(
                                            "Unexpected directional encountered: {}",
                                            other
                                        );
                                        Ok((remaining, Some(value)))
                                    }
                                },
                                // No additional directional found, return first value of
                                // riectional and original "remaining" instead of "rem".
                                None => Ok((remaining, Some(value))),
                            }
                        } else {
                            Ok((remaining, Some(value)))
                        }
                    }
                    // Directional found, but no continuation, so return remainder and value.
                    _ => Ok((remaining, predir)),
                },
                // If none, return the original input, sinced we haven't parsed anything from it.
                None => Ok((input, predir)),
            }
        } else {
            // No pre-directional present, return original input.
            Ok((input, None))
        }
    }

    /// The `pre_modifier` method attempts to parse the next word in the input as a
    /// [`StreetNamePreModifier`] variant.  Returns the full input in no pre-modifier is present.
    pub fn pre_modifier(input: &str) -> IResult<&str, Option<StreetNamePreModifier>> {
        // Strip preceding whitespace.
        let (rem, _) = space0(input)?;
        // Take one or more alphabetic character.
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(rem) {
            // Attempt to read as pre-modifier.
            let check = StreetNamePreModifier::match_mixed(result);
            match check {
                // If some, return the remainder and the value.
                Some(value) => Ok((rem, Some(value))),
                // If none, return the original input.
                None => Ok((input, None)),
            }
        } else {
            // If parsing input fails, return the original input.
            Ok((input, None))
        }
    }

    /// The `pre_type` method attempts to parse the next word in the input as a
    /// [`StreetNamePreType`] variant.  Returns the full input if no pre-type is present.
    pub fn pre_type(input: &str) -> IResult<&str, Option<StreetNamePreType>> {
        // Strip preceding whitespace.
        let (rem, _) = space0(input)?;
        // Take one or more alphabetic character.
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(rem) {
            // Attempt to read as pre-type.
            let check = StreetNamePreType::match_mixed(result);
            match check {
                // If some, return the remainder and the value.
                Some(value) => Ok((rem, Some(value))),
                // If none, return the original input.
                None => Ok((input, None)),
            }
        } else {
            // If parsing input fails, return the original input.
            Ok((input, None))
        }
    }

    /// The `separator` method attempts to parse the next word in the input as a
    /// [`StreetSeparator`] variant.  Returns the full input if no separator is present.
    pub fn separator(input: &str) -> IResult<&str, Option<StreetSeparator>> {
        // Strip preceding whitespace.
        let (rem, _) = space0(input)?;
        // Take one or more alphabetic character.
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(rem) {
            if result.to_lowercase().as_str() == "of" {
                // Strip preceding whitespace.
                let (rem, _) = space0(rem)?;
                if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(rem) {
                    if result.to_lowercase().as_str() == "the" {
                        Ok((rem, Some(StreetSeparator::OfThe)))
                    } else {
                        // If not "of the", return original input with None
                        Ok((input, None))
                    }
                } else {
                    Ok((input, None))
                }
            } else {
                Ok((input, None))
            }
        } else {
            // If parsing input fails, return the original input.
            Ok((input, None))
        }
    }

    /// The `street_name` method attempts to parse the next sequence of words in the input as a
    /// street name.  After finding at least one word, will return if the next word in `input` is a
    /// street name post type.
    /// Screen for PO Boxes?
    pub fn street_name(input: &str) -> IResult<&str, Option<String>> {
        // On the initial pass, we read the first word of the street name.
        let mut name = String::new();
        // Strip preceding whitespace.
        let (rem, _) = space0(input)?;
        // Try to take the first word.
        if let Ok((rem, result)) = alphanumeric1::<&str, nom::error::Error<_>>(rem) {
            // Push the word to the empty name variable.
            name.push_str(result);
            tracing::trace!("Working name: {}", name);
            // Strip preceding whitespace from the remainder.
            let (rem, _) = space0(rem)?;
            // Save the remainder to possibly return later.
            let mut remaining = rem;
            // Capture apostrophes in street names.
            tracing::trace!("Apostrophe check on {}", remaining);
            let (rem, apostrophe) = opt(tag("'"))(remaining)?;
            if let Some(value) = apostrophe {
                tracing::trace!("Apostrophe found, rem: {}", rem);
                name.push_str(value);
                // Could probably just tack on an 'S' here.
                // Skipping check for other types because it follows an apostrophe.
                if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(rem) {
                    name.push_str(result);
                    remaining = rem;
                }
            }
            // The next word of remaining may be part of the street name.
            // It could also be a post type, subaddress, city, state or zip.
            // Check if the next word parses as a post type.
            let (_, mut cond) = Self::is_post_type(remaining)?;
            // If a post type is found, check to see if it is followed by a post type.
            if cond {
                let (first, _) = Self::post_type(remaining)?;
                let (_, next) = Self::is_post_type(first)?;
                if next {
                    // If so, only the last type is the post type.
                    cond = false;
                }
                // If the post type could also be a subaddress, parse as post type and not part of
                // the street name.
                if let Ok((_, Some(_))) = Self::subaddress_type(first) {
                    cond = true;
                }
            }
            // Check if the next word parses as a postal community.
            let (_, check) = Self::is_postal_community(remaining)?;
            if check {
                // Cond is the variable that will control the while loop
                cond = true;
            }
            // If cond is false because input is empty, set to true
            if eof::<&str, nom::error::Error<_>>(remaining).is_ok() {
                tracing::trace!("Eof detected.");
                cond = true;
            // Break if input is not alphanumeric.
            } else if alphanumeric1::<&str, nom::error::Error<_>>(remaining).is_err() {
                tracing::trace!("Nonalphanumeric characters detected: {}", remaining);
                cond = true;
            }
            tracing::trace!("Initial condition is {}", cond);
            while !cond {
                // Strip preceding whitespace.
                let (rem, _) = space0(remaining)?;
                // Take one or more alphabetic character.
                if let Ok((rem, result)) = alphanumeric1::<&str, nom::error::Error<_>>(rem) {
                    // Strip preceding whitespace.
                    let (rem, _) = space0(rem)?;
                    // Read has succeeded, reset remainder.
                    remaining = rem;
                    // Push parsed word to street name.
                    name.push(' ');
                    name.push_str(result);
                    tracing::trace!("Working name: {}", name);
                    // If next word is a post type, end loop.
                    (_, cond) = Self::is_post_type(rem)?;
                    // If a post type is found, check to see if it is followed by a post type.
                    if cond {
                        let (first, _) = Self::post_type(rem)?;
                        let (_, next) = Self::is_post_type(first)?;
                        if next {
                            // If so, only the last type is the post type.
                            cond = false;
                        }
                        // If the post type could also be a subaddress, parse as post type and not part of
                        // the street name.
                        if let Ok((_, Some(_))) = Self::subaddress_type(first) {
                            cond = true;
                        }
                    }
                    // If next word is a postal community, end loop.
                    let (_, check) = Self::is_postal_community(rem)?;
                    if check {
                        cond = true;
                    }
                    // End loop if at end of input.
                    if eof::<&str, nom::error::Error<_>>(rem).is_ok() {
                        tracing::trace!("Eof detected.");
                        cond = true;
                    // Break if input is not alphanumeric.
                    } else if alphanumeric1::<&str, nom::error::Error<_>>(rem).is_err() {
                        tracing::trace!("Nonalphanumeric characters detected.");
                        cond = true;
                    }
                }
            }
            tracing::trace!("Rem: {}", remaining);
            Ok((remaining, Some(name.to_uppercase())))
        } else {
            Ok((input, None))
        }
    }

    /// The `post_type` function attempts to parse the next word in the input as a
    /// [`StreetNamePostType`] value.  Since the street name post type is a required field for Grants
    /// Pass addresses, there is no need to peek and conditionally return.  If the street name post
    /// type evaluates to None, this is not a hard error, because the post type is not a required field
    /// according to the FGDC standard, and partner agencies such as ECSO may have valid addresses
    /// without a street name post type (e.g. "Broadway").
    pub fn post_type(input: &str) -> IResult<&str, Option<StreetNamePostType>> {
        tracing::trace!("Calling post_type on {}", input);
        let (remaining, _) = space0(input)?;
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(remaining) {
            tracing::trace!("Post type check on {:#?}", &result);
            let post_type = StreetNamePostType::match_mixed(result);
            match post_type {
                Some(_) => Ok((rem, post_type)),
                None => Ok((remaining, post_type)),
            }
        } else {
            tracing::trace!("Invalid post type input.");
            Ok((remaining, None))
        }
    }

    /// The `is_post_type` function returns true if the input parses to a valid [`StreetNamePostType`].
    /// Peeks at the data without consuming it.
    pub fn is_post_type(input: &str) -> IResult<&str, bool> {
        tracing::trace!("Calling is_post_type");
        if let Ok((_, post)) = Self::post_type(input) {
            tracing::trace!("Post type is {:#?}", &post);
            Ok((input, post.is_some()))
        } else {
            tracing::trace!("No post type detected.");
            Ok((input, false))
        }
    }

    /// The `subaddress_type` function attempts to find a word following the street name post
    /// type and preceding the postal community.  If a word is present, and parses to a subaddress
    /// type, the function will return the type and the remainder.  If no subaddress type is present,
    /// the function will return the full input.
    pub fn subaddress_type(input: &str) -> IResult<&str, Option<SubaddressType>> {
        tracing::trace!("Calling subaddress_type on {}", input);
        // Strip preceding period.
        let (rem, _) = opt(tag("."))(input)?;
        // Strip preceding whitespace.
        let (rem, _) = space0(rem)?;
        // Strip preceding number sign.
        let (rem, _) = opt(tag("#"))(rem)?;
        // Take one or more alphabetic character.
        if let Ok((rem, result)) = alphanumeric1::<&str, nom::error::Error<_>>(rem) {
            // Attempt to read as subaddress type.
            let check = SubaddressType::match_mixed(result);
            match check {
                // If some, return the remainder and the value.
                Some(value) => Ok((rem, Some(value))),
                // If none, return the original input.
                None => Ok((input, None)),
            }
        } else {
            // If parsing input fails, return the original input.
            Ok((input, None))
        }
    }

    /// The `subaddress_id` function attempts to find a word following the street name post
    /// type and preceding the postal community.  If a word is present, and parses to a subaddress
    /// type, the function will return the type and the remainder.  If no subaddress type is present,
    /// the function will return the full input.
    pub fn subaddress_id(input: &str) -> IResult<&str, Option<String>> {
        tracing::trace!("Calling subaddress_id on {}", input);
        // Strip preceding period.
        let (rem, _) = opt(tag("."))(input)?;
        // Strip preceding whitespace.
        let (rem, _) = space0(rem)?;
        // Strip common subaddress identifier symbols.
        let (rem, _) = opt(tag("#"))(rem)?;
        let (rem, _) = opt(tag("&"))(rem)?;
        let (rem, _) = opt(tag("-"))(rem)?;
        // Strip whitespace between symbol and id.
        let (rem, _) = space0(rem)?;
        // If there is no subaddress, we expect the city name next.
        let (_, mut cond) = Self::is_postal_community(rem)?;
        // Could be a state name instead of a subaddress.
        let (_, state) = Self::is_state(rem)?;
        // Could be a zip code.
        let (_, zip) = Self::is_zip(rem)?;
        cond = cond | state | zip;
        // End loop if at end of input.
        if eof::<&str, nom::error::Error<_>>(rem).is_ok() {
            tracing::trace!("Eof detected.");
            cond = true;
        }
        // Variable to store id.
        let mut id = String::new();
        let mut remain = rem;
        // Next value is likely a subaddress.
        // Loop to parse potentially multiple subaddresses.
        while !cond {
            tracing::trace!("Cond is {}", cond);
            tracing::trace!("Rem: {}", &rem);
            // Take one or more alphanumeric characters.
            if let Ok((rem, result)) = alphanumeric1::<&str, nom::error::Error<_>>(remain) {
                // Add the value to the subaddress string.
                if !id.is_empty() {
                    id.push(' ');
                }
                id.push_str(result);
                tracing::trace!("Id: {}", &id);
                tracing::trace!("Rem: {}", &rem);

                // Second pass.
                // Strip preceding whitespace.
                let (rem, _) = space0(rem)?;
                // Strip common subaddress identifier symbols.
                let (rem, _) = opt(tag("#"))(rem)?;
                let (rem, _) = opt(tag("&"))(rem)?;
                let (rem, _) = opt(tag("-"))(rem)?;
                let (rem, _) = opt(tag(","))(rem)?;
                // Strip whitespace between symbol and id.
                let (rem, _) = space0(rem)?;
                remain = rem;
                // If there is no subaddress, we expect the city name next.
                let (_, comm) = Self::is_postal_community(rem)?;
                // Could be a state name instead of a subaddress.
                let (_, state) = Self::is_state(rem)?;
                // Could be a zip code.
                let (_, zip) = Self::is_zip(rem)?;
                cond = comm | state | zip;
                // End loop if at end of input.
                if eof::<&str, nom::error::Error<_>>(rem).is_ok() {
                    tracing::trace!("Eof detected.");
                    cond = true;
                }
            } else {
                tracing::trace!("Subaddress ID not present where expected.");
                // Exit loop, we have come off the rails.
                cond = true;
            }
        }
        if id.is_empty() {
            Ok((input, None))
        } else {
            Ok((remain, Some(id)))
        }
    }

    /// The `postal_community` function attempts to parse the next word in the input as a
    /// [`PostalCommunity`] value.
    #[allow(clippy::single_match)]
    pub fn postal_community(input: &str) -> IResult<&str, Option<PostalCommunity>> {
        tracing::trace!("Calling postal_community on {}", input);
        // Holds potentially compound community name.
        let mut comm = String::new();
        let (rem, _) = opt(tag(","))(input)?;
        // Strip preceding whitespace.
        let (remaining, _) = space0(rem)?;
        // Take one or more alphanumeric characters.
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(remaining) {
            // Strip preceding whitespace.
            let (mut rem, _) = space0(rem)?;
            // Add first word to community name.
            comm.push_str(result);
            tracing::trace!("Postal community check on {:#?}", &result);
            // Check for compound community name.
            match result.to_lowercase().as_str() {
                // Grants Pass
                "grants" => {
                    tracing::trace!("Attempting to match remainder: {}", rem);
                    // Add the next word to community name.
                    if let Ok((remain, next)) = alpha1::<&str, nom::error::Error<_>>(rem) {
                        tracing::trace!("Next is {}", next);
                        comm.push(' ');
                        comm.push_str(next);
                        rem = remain;
                    }
                    tracing::trace!("Comm is {}", comm);
                }
                _ => {}
            }
            // Match community name against valid postal communities.
            let post_comm = PostalCommunity::match_mixed(&comm);
            Ok((rem, post_comm))
        } else {
            tracing::trace!("Invalid postal community input.");
            Ok((remaining, None))
        }
    }

    /// The `is_postal_community` function returns true if the input parses to a valid [`PostalCommunity`].
    /// Peeks at the data without consuming it.
    pub fn is_postal_community(input: &str) -> IResult<&str, bool> {
        tracing::trace!("Calling is_postal_community");
        if let Ok((_, post)) = Self::postal_community(input) {
            tracing::trace!("Postal community is {:#?}", &post);
            Ok((input, post.is_some()))
        } else {
            tracing::trace!("No postal community detected.");
            Ok((input, false))
        }
    }

    /// The `state` function attempts to parse the next word in the input as a
    /// [`State`] value.
    pub fn state(input: &str) -> IResult<&str, Option<State>> {
        tracing::trace!("Calling state on {}", input);
        // Strip preceding comma.
        let (rem, _) = opt(tag(","))(input)?;
        // Strip preceding whitespace.
        let (remaining, _) = space0(rem)?;
        // State name is alphabetic
        if let Ok((rem, result)) = alpha1::<&str, nom::error::Error<_>>(remaining) {
            tracing::trace!("State check on {:#?}", &result);
            if let Some(state) = State::match_mixed(result) {
                Ok((rem, Some(state)))
            } else {
                Ok((remaining, None))
            }
        } else {
            tracing::trace!("Invalid state input.");
            Ok((remaining, None))
        }
    }

    /// The `is_state` function returns true if the input parses to a valid [`State`].
    /// Peeks at the data without consuming it.
    pub fn is_state(input: &str) -> IResult<&str, bool> {
        tracing::trace!("Calling is_state");
        if let Ok((_, state)) = Self::state(input) {
            tracing::trace!("Postal community is {:#?}", &state);
            Ok((input, state.is_some()))
        } else {
            tracing::trace!("No state detected.");
            Ok((input, false))
        }
    }

    /// The `zip` function attempts to parse the next word in the input as a
    /// postal zip code.
    pub fn zip(input: &str) -> IResult<&str, Option<i64>> {
        tracing::trace!("Calling zip on {}", input);
        // Strip preceding comma.
        let (rem, _) = opt(tag(","))(input)?;
        // Strip preceding whitespace.
        let (remaining, _) = space0(rem)?;
        // Zip code is an integer.
        if let Ok((rem, result)) = digit1::<&str, nom::error::Error<_>>(remaining) {
            tracing::trace!("Zip check on {:#?}", &result);
            // Zip code must have 5 digits
            if result.len() == 5 {
                // Try to parse as number.
                if let Ok(num) = result.parse() {
                    // Return successful zip code.
                    Ok((rem, Some(num)))
                } else {
                    // If it doesn't parse, return input
                    Ok((remaining, None))
                }
            } else {
                Ok((remaining, None))
            }
        } else {
            tracing::trace!("Invalid zip input.");
            Ok((remaining, None))
        }
    }

    /// The `is_zip` function returns true if the input parses to a valid zip code.
    /// Peeks at the data without consuming it.
    pub fn is_zip(input: &str) -> IResult<&str, bool> {
        tracing::trace!("Calling is_zip");
        if let Ok((_, zip)) = Self::zip(input) {
            tracing::trace!("Zip is {:#?}", &zip);
            Ok((input, zip.is_some()))
        } else {
            tracing::trace!("No state detected.");
            Ok((input, false))
        }
    }

    /// The `address` function attempts to read the complete address and parse it into its
    /// constituent components.
    pub fn address(input: &str) -> IResult<&str, PartialAddress> {
        // When reading a partial address, any field can fail, so we cannot use the question mark
        // operator or it will short circuit cases where we correctly infer None when given an
        // invalid string.
        // this struct will hold the values of the parsed address components
        let mut address = PartialAddress::default();
        // attempt to read the complete address number
        let (rem, address_number) = Self::address_number(input)?;
        tracing::trace!("Address number: {:?}", &address_number);
        // we avoid an if let clause because address_number is none if not present.
        address.address_number = address_number;
        let (rem, suffix) = Self::address_number_suffix(rem)?;
        tracing::trace!("Address number suffix: {:#?}", &suffix);
        address.set_address_number_suffix(suffix);
        let (rem, directional) = Self::pre_directional(rem)?;
        tracing::trace!("Street name pre-directional: {:#?}", &directional);
        address.street_name_pre_directional = directional;
        let (rem, premod) = Self::pre_modifier(rem)?;
        tracing::trace!("Street name pre-modifier: {:#?}", &premod);
        address.pre_modifier = premod;
        let (rem, pretype) = Self::pre_type(rem)?;
        tracing::trace!("Street name pre-type: {:#?}", &pretype);
        address.pre_type = pretype;
        let (rem, separator) = Self::separator(rem)?;
        tracing::trace!("Street name separator: {:#?}", &separator);
        address.separator = separator;
        let (rem, name) = Self::street_name(rem)?;
        tracing::trace!("Street name element: {:#?}", &name);
        address.street_name = name;
        let (rem, post_type) = Self::post_type(rem)?;
        tracing::trace!("Street name post-type: {:#?}", &post_type);
        address.street_name_post_type = post_type;
        let (rem, sub_type) = Self::subaddress_type(rem)?;
        tracing::trace!("Subaddress type: {:#?}", &sub_type);
        address.subaddress_type = sub_type;
        let (rem, sub_id) = Self::subaddress_id(rem)?;
        tracing::trace!("Subaddress id: {:#?}", &sub_id);
        address.subaddress_identifier = sub_id;
        let (rem, post_comm) = Self::postal_community(rem)?;
        tracing::trace!("Postal community: {:#?}", &post_comm);
        address.postal_community = post_comm;
        let (rem, state) = Self::state(rem)?;
        tracing::trace!("Postal community: {:#?}", &state);
        address.state_name = state;
        let (rem, zip) = Self::zip(rem)?;
        tracing::trace!("Zip code: {:#?}", &zip);
        address.zip_code = zip;
        Ok((rem, address))
    }
}

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
    // Strip preceding whitespace.
    let (rem, _) = space0(input)?;
    // Take one or more alphabetic character.
    let (rem, result) = alpha1(rem)?;
    // Match against valid directional values.
    let predir = StreetNamePreDirectional::match_mixed(result);
    match predir {
        // If some, return the remainder after parsing the directional.
        Some(_) => Ok((rem, predir)),
        // If none, return the original input, sinced we haven't parsed anything from it.
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
    let post_type = StreetNamePostType::match_mixed(result);
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
    // Empty vec to populate with post type values.
    let mut post_type = Vec::new();
    // Condition for while loop, starts at true and runs until set false
    let mut cond = true;
    // Copy the input string to return if unable to parse any input.
    let mut remaining = input;
    while cond {
        // `post` may contain one or more alphabetic characters.
        let (rem, post) = opt(single_word)(remaining)?;
        match post {
            Some(value) => {
                // Check if the value is a post type.
                let val = StreetNamePostType::match_mixed(value);
                match val {
                    // If so, push to the vector of post type values.
                    Some(val_type) => {
                        post_type.push(val_type);
                        // Set 'remaining' to the remainder after parsing the value.
                        remaining = rem;
                    }
                    // If the word is not a post type, no more post types are present, break and
                    // return.
                    None => cond = false,
                }
            }
            // If no word is present, break and return.
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
        SubaddressType::match_mixed(word)
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
    // can't we remove this if let like above? No, the set method takes a value and wraps it in
    // Some.
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
