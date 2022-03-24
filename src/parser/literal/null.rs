use crate::{
    nom::tag,
    token::{null, token, IResult, Span},
};

use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// Null token.
///
/// Parses the literal `null` token.
///
/// # Examples
///
/// ```ignore
/// null
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    let parser = tag("null");

    token("null", parser, |v| (v, null()))(s)
}

#[cfg(test)]
#[test_case("null" => Ok(null()))]
#[test_case("nulll" => Ok(null()))]
#[test_case("nnull" => Err(r#"error at 1:1 nnull: Context("null")"#.to_owned()))]
#[test_case("foo" => Err(r#"error at 1:1 foo: Context("null")"#.to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
