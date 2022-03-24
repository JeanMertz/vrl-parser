use crate::token::{token, IResult, Span, Variant::Comment};

use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::sequence::pair;
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// A single-line comment.
///
/// Comments are allowed to be present anywhere in an expression, but once
/// a comment starts, the rest of the line (until a newline character is
/// detected) is considered to be part of that comment token.
///
/// Leading whitespace in comments is preserved, to keep the exact indentation
/// of content within comments.
///
/// # Examples
///
/// ```ignore
/// # this is a comment
/// #this too
/// #   and this
/// this is not
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    let start = char('#');
    let comment = is_not("\n\r");
    let parser = pair(start, comment);

    token("comment", parser, |(_, v)| (v, Comment(&v)))(s)
}

#[cfg(test)]
#[test_case("# foo bar" => Ok(Comment(" foo bar")))]
#[test_case("# foo # baz" => Ok(Comment(" foo # baz")))]
#[test_case("#foobar" => Ok(Comment("foobar")))]
#[test_case("#    foo" => Ok(Comment("    foo")))]
#[test_case("# foo\nbar" => Ok(Comment(" foo")))]
#[test_case("# foo\n\rbar" => Ok(Comment(" foo")))]
#[test_case("# foo\rbar" => Ok(Comment(" foo")))]
#[test_case("foobar" => Err("error at 1:1 foobar: Char('#')".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
