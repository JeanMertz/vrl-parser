use crate::{
    nom::tag,
    token::{token, IResult, Span, Variant::EndOfExpression},
};

use nom::{branch::alt, character::complete::line_ending};
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// End-of-expression token.
///
/// Completes parsing when at the end of an expression (line ending or `;`).
///
/// # Examples
///
/// ```ignore
/// foo; bar
/// baz
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    let parser = alt((tag(";"), line_ending));

    token("end-of-expression", parser, |v| (v, EndOfExpression))(s)
}

#[cfg(test)]
#[test_case(";" => Ok(EndOfExpression) ; "semicolon")]
#[test_case("\n" => Ok(EndOfExpression) ; "n")]
#[test_case("\r\n" => Ok(EndOfExpression) ; "rn")]
#[test_case(";\r\n" => Ok(EndOfExpression) ; "semicolon-rn")]
#[test_case("true;" => Err("error at 1:1 true;: Nom(CrLf)".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
