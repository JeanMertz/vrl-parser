use crate::token::{float, token, IResult, Span};

use nom::{
    combinator::{map_parser, verify},
    number::complete::{double, recognize_float_or_exceptions},
};
use nom_tracable::tracable_parser;

#[cfg(test)]
use test_case::test_case;

/// A float literal.
///
/// # Examples
///
/// ```ignore
/// 1.0
/// 1.2345
/// .2
/// 3.
/// 4e-1
/// ```
#[tracable_parser]
pub fn parse(s: Span) -> IResult {
    // By default, the `double` parser accepts `4` as a valid float, and turns
    // it into `4.0`. We reject this, as `4` should be parsed as an integer.
    let float_str = verify(recognize_float_or_exceptions, |v: &Span| v.contains('.'));

    let parser = map_parser(float_str, double);

    token("float", parser, |v| (s, float(v)))(s)
}

#[cfg(test)]
#[test_case("1.0" => Ok(float(1.0)))]
#[test_case("192.0" => Ok(float(192.0)))]
#[test_case("-193.2" => Ok(float(-193.2)))]
#[test_case(".2" => Ok(float(0.2)))]
#[test_case("3." => Ok(float(3.0)))]
#[test_case("4" => Err(r#"error at 1:1 4: Nom(Verify)"#.to_owned()))]
#[test_case("..194" => Err(r#"error at 1:1 ..194: Nom(Float)"#.to_owned()))]
fn test(s: &str) -> super::TResult {
    use crate::token::span;

    super::test(parse(span(s)))
}
