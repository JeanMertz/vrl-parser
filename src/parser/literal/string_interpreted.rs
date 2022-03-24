use crate::token::{string, token, IResult, Span};

use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while_m_n},
    character::complete::{char, multispace1},
    combinator::{map, map_opt, map_res, value, verify},
    multi::fold_many0,
    sequence::{delimited, preceded},
};
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(&s),
            StringFragment::EscapedChar(c) => string.push(c),
            StringFragment::EscapedWS => {}
        }
        string
    });

    let parser = delimited(char('"'), build_string, char('"'));

    token("string", parser, |v| (input, string(v)))(input)
}

#[cfg(test)]
#[test_case(r#""foo""# => Ok(string("foo".to_owned())))]
#[test_case(r#""foo bar""# => Ok(string("foo bar".to_owned())))]
#[test_case(r#""bar"# => Err("error at 1:5 : Char('\"')".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}

// ------------------------------------------------------------------------------

/// Parse a unicode sequence, of the form u{XXXX}, where XXXX is 1 to 6
/// hexadecimal numerals. We will combine this later with parse_escaped_char
/// to parse sequences like \u{00AC}.
#[tracable_parser]
fn parse_unicode(s: Span) -> IResult<char> {
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_hex = map_res(parse_delimited_hex, |hex: Span| {
        u32::from_str_radix(&hex, 16)
    });

    map_opt(parse_hex, std::char::from_u32)(s)
}

/// Parse an escaped character: \n, \t, \r, \u{00AC}, etc.
#[tracable_parser]
fn parse_escaped_char(input: Span) -> IResult<char> {
    preceded(
        char('\\'),
        alt((
            parse_unicode,
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
        )),
    )(input)
}

/// Parse a backslash, followed by any amount of whitespace. This is used later
/// to discard any escaped whitespace.
#[tracable_parser]
fn parse_escaped_whitespace(input: Span) -> IResult<Span> {
    preceded(char('\\'), multispace1)(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
#[tracable_parser]
fn parse_literal(input: Span) -> IResult<Span> {
    let not_quote_slash = is_not("\"\\");

    verify(not_quote_slash, |s: &Span| !s.is_empty())(input)
}

/// A string fragment contains a fragment of a string being parsed: either
/// a non-empty Literal (a series of non-escaped characters), a single
/// parsed escaped character, or a block of escaped whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(Span<'a>),
    EscapedChar(char),
    EscapedWS,
}

/// Combine parse_literal, parse_escaped_whitespace, and parse_escaped_char
/// into a StringFragment.
#[tracable_parser]
fn parse_fragment<'a>(input: Span<'a>) -> IResult<StringFragment<'a>> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))(input)
}
