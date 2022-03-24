use crate::{
    nom::{tag, value},
    token::{bool, token, IResult, Span},
};

use nom::branch::alt;
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// Boolean token.
///
/// Parses the literal boolean token (`true` or `false`).
///
/// # Examples
///
/// ```ignore
/// true
/// false
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));
    let parser = alt((parse_true, parse_false));

    token("bool", parser, |(o, v)| (o, bool(v)))(s)
}

#[cfg(test)]
#[test_case("true" => Ok(bool(true)))]
#[test_case("false" => Ok(bool(false)))]
#[test_case("truee" => Ok(bool(true)))]
#[test_case("falsee" => Ok(bool(false)))]
#[test_case("ttrue" => Err(r#"error at 1:1 ttrue: Context("false")"#.to_owned()))]
#[test_case("ffalse" => Err(r#"error at 1:1 ffalse: Context("false")"#.to_owned()))]
#[test_case("foo" => Err(r#"error at 1:1 foo: Context("false")"#.to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
