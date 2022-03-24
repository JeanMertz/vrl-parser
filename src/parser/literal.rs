mod bool;
mod float;
mod integer;
mod null;
mod regex;
mod string_interpreted;
mod string_raw;
mod timestamp;

use crate::token::{IResult, Span};

use nom::branch::alt;
use nom_tracable::tracable_parser;

#[cfg(test)]
use super::{test, TResult};
#[cfg(test)]
use test_case::test_case;

pub use self::bool::parse as bool;
pub use float::parse as float;
pub use integer::parse as integer;
pub use null::parse as null;
pub use regex::parse as regex;
pub use string_interpreted::parse as string_interpreted;
pub use string_raw::parse as string_raw;
pub use timestamp::parse as timestamp;

/// Parse a single literal.
///
/// # Examples
///
/// ```ignore
/// true
/// "foo"
/// null
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    alt((
        string_interpreted,
        string_raw,
        float,
        integer,
        bool,
        null,
        regex,
        timestamp,
    ))(input)
}

#[cfg(test)]
#[test_case(r#""foo""# => Ok(crate::token::string("foo")))]
#[test_case("s'bar'" => Ok(crate::token::string("bar")))]
#[test_case("42.0" => Ok(crate::token::float(42.0)))]
#[test_case("42" => Ok(crate::token::integer(42)))]
#[test_case("true" => Ok(crate::token::bool(true)))]
#[test_case("null" => Ok(crate::token::null()))]
#[test_case("r'foo'" => Ok(crate::token::regex("foo")))]
#[test_case("t'bar'" => Ok(crate::token::timestamp("bar")))]
fn test_parse(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
