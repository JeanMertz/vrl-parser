use crate::token::{token, IResult, Span, Variant::Ident};

use nom::character::complete::alphanumeric1;
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
    let parser = alphanumeric1;

    token("ident", parser, |v| (v, Ident(&v)))(s)
}

#[cfg(test)]
#[test_case("foo" => Ok(Ident("foo")))]
#[test_case("foo bar" => Ok(Ident("foo")))]
#[test_case(" bar" => Err(r#"error at 1:1  bar: Nom(AlphaNumeric)"#.to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
