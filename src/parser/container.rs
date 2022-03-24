mod group;

use crate::token::{IResult, Span};

use nom::branch::alt;
use nom_tracable::tracable_parser;

#[cfg(test)]
use super::{test, TResult};
#[cfg(test)]
use test_case::test_case;

pub use group::parse as group;

/// Parse a single container.
///
/// # Examples
///
/// ```ignore
/// (true)
/// { true }
/// [true]
/// { "foo": true }
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    alt((group,))(input)
}

#[cfg(test)]
#[test_case("(true)" => Ok(crate::token::group(crate::parser::literal::bool(crate::token::span_sliced("(true)", 1..5)).unwrap().1)))]
fn test_parse(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
