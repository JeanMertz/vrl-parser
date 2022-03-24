use crate::{
    nom::tag,
    parser::partials::string_literal_inner,
    token::{timestamp, token, IResult, Span},
};

use nom::{character::complete::char, sequence::delimited};
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// Parse a regex literal.
///
/// # Examples
///
/// ```ignore
/// t'2020-02-01T12:00:00.000Z'
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    let parser = delimited(tag("t'"), string_literal_inner, char('\''));

    token("timestamp", parser, |v| (input, timestamp(v)))(input)
}

#[cfg(test)]
#[test_case("t'foo'" => Ok(timestamp("foo".to_owned())))]
#[test_case("t'foo bar'" => Ok(timestamp("foo bar".to_owned())))]
#[test_case("t'foo \\' baz'" => Ok(timestamp("foo ' baz".to_owned())))]
#[test_case("t'foo \n qux'" => Ok(timestamp("foo \n qux".to_owned())))]
#[test_case("t'bar" => Err("error at 1:6 : Char('\\'')".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
