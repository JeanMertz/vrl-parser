use crate::token::{IResult, Span};

use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{anychar, char},
    combinator::{map, value, verify},
    multi::fold_many0,
    sequence::preceded,
};
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

#[tracable_parser]
pub fn parse(input: Span) -> IResult<String> {
    fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(&s),
            StringFragment::EscapedQuote => string.push('\''),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    })(input)
}

#[cfg(test)]
#[test_case("foo" => Ok("foo".to_owned()))]
#[test_case("foo bar" => Ok("foo bar".to_owned()))]
#[test_case("foo \\' baz" => Ok("foo ' baz".to_owned()))]
#[test_case("foo \n qux" => Ok("foo \n qux".to_owned()))]
fn test(s: &str) -> super::TResult<String> {
    use crate::token::span;

    super::test(parse(span(s)))
}

/// Parse an escaped quote: \'.
#[tracable_parser]
fn parse_escaped_quote(input: Span) -> IResult<char> {
    preceded(char('\\'), char('\''))(input)
}

/// Parse an escaped character.
#[tracable_parser]
fn parse_escaped_char(input: Span) -> IResult<char> {
    preceded(char('\\'), anychar)(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
#[tracable_parser]
fn parse_literal(input: Span) -> IResult<Span> {
    let not_quote_slash = is_not("\\'");

    verify(not_quote_slash, |s: &Span| !s.is_empty())(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(Span<'a>),
    EscapedQuote,
    EscapedChar(char),
}

#[tracable_parser]
fn parse_fragment<'a>(input: Span<'a>) -> IResult<StringFragment<'a>> {
    alt((
        map(parse_literal, StringFragment::Literal),
        value(StringFragment::EscapedQuote, parse_escaped_quote),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}
