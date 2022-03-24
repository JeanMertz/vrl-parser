use crate::token::{integer, token, IResult, Span};

use nom::character::complete::i64;
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// A single-line comment.
///
/// Comments are allowed to be present anywhere in an expression, but once
/// a comment starts, the rest of the line (until a newline character is
/// detected) is considered to be part of that comment token.
///
/// Leading whitespace in comments is preserved, to keep the exact indentation
/// of content within comments.
///
/// # Examples
///
/// ```ignore
/// # this is a comment
/// #this too
/// #   and this
/// this is not
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    token("integer", i64, |v| (s, integer(v)))(s)
}

#[cfg(test)]
#[test_case("1" => Ok(integer(1)))]
#[test_case("192" => Ok(integer(192)))]
#[test_case("-193" => Ok(integer(-193)))]
#[test_case("--194" => Err(r#"error at 1:1 --194: Nom(Digit)"#.to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
