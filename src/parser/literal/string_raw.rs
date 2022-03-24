use crate::{
    nom::tag,
    parser::partials::string_literal_inner,
    token::{string, token, IResult, Span},
};

use nom::{character::complete::char, sequence::delimited};
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// Parse a raw string literal.
///
/// # Examples
///
/// ```ignore
/// s'foo bar'
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    let parser = delimited(tag("s'"), string_literal_inner, char('\''));

    token("string_raw", parser, |v| (input, string(v)))(input)
}

#[cfg(test)]
#[test_case("s'foo'" => Ok(string("foo".to_owned())))]
#[test_case("s'foo bar'" => Ok(string("foo bar".to_owned())))]
#[test_case("s'foo \\' baz'" => Ok(string("foo ' baz".to_owned())))]
#[test_case("s'foo \n qux'" => Ok(string("foo \n qux".to_owned())))]
#[test_case("s'bar" => Err("error at 1:6 : Char('\\'')".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
