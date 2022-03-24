use crate::token::{group, token, IResult, Span};

use nom::{character::complete::char, sequence::delimited};
use nom_tracable::tracable_parser;

use crate::parser::literal::bool;

#[cfg(test)]
use test_case::test_case;

/// Parse a group of expressions.
///
/// # Examples
///
/// ```ignore
/// (true && false)
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    let parser = delimited(char('('), bool, char(')'));

    token("group", parser, |v| (input, group(v)))(input)
}

// TODO: Come up with a similar test framework used in the existing lexer code.
#[cfg(test)]
#[test_case("(true)" => Ok(group(super::bool(crate::token::span_sliced("(true)", 1..5)).unwrap().1)))]
fn test(s: &str) -> super::TResult {
    super::test(parse(crate::token::span(s)))
}
