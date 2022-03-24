use crate::token::{token, IResult, Span, Variant::Expression};

use nom::branch::alt;
use nom_tracable::tracable_parser;

use super::{container, literal};

/// Parse a single expression.
///
/// # Examples
///
/// ```ignore
/// true && false
/// ```
#[tracable_parser]
pub fn parse(input: Span) -> IResult {
    let parser = alt((literal, container));

    token("expression", parser, |v| (input, Expression(Box::new(v))))(input)
}
