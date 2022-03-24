use crate::{
    nom::tag,
    parser::partials::string_literal_inner,
    token::{regex, token, IResult, Span},
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
/// r'foo bar'
/// r'.*'
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    let parser = delimited(tag("r'"), string_literal_inner, char('\''));

    token("regex", parser, |v| (input, regex(v)))(input)
}

#[cfg(test)]
#[test_case("r'foo'" => Ok(regex("foo".to_owned())))]
#[test_case("r'foo bar'" => Ok(regex("foo bar".to_owned())))]
#[test_case("r'foo \\' baz'" => Ok(regex("foo ' baz".to_owned())))]
#[test_case("r'foo \n qux'" => Ok(regex("foo \n qux".to_owned())))]
#[test_case("r'bar" => Err("error at 1:6 : Char('\\'')".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
