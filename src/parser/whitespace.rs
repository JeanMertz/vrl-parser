use crate::token::{token, IResult, Span, Variant::*};

use nom::bytes::complete::take_while1;
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// whitespace characters.
///
/// This parser consumes any consecutive whitespace. The following characters
/// are considered whitespace by the parser:
///
/// - space (` `)
/// - tab   (`\t`)
/// - CR    (`\r`)
/// - LF    (`\n`)
///
/// # Examples
///
/// ```ignore
///   \n
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    let whitespace_chars = " \t\r\n";
    let parser = take_while1(move |c| whitespace_chars.contains(c));

    token("whitespace", parser, |v| (v, Whitespace))(s)
}

#[cfg(test)]
#[test_case(" " => Ok(Whitespace))]
#[test_case("\t" => Ok(Whitespace))]
#[test_case("\n" => Ok(Whitespace))]
#[test_case("\n\r" => Ok(Whitespace))]
#[test_case("  \n\r bar" => Ok(Whitespace))]
#[test_case("foobar" => Err("error at 1:1 foobar: Nom(TakeWhile1)".to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
