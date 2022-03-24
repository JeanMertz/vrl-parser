use std::ops::Range;

use nom::{
    combinator::map,
    error::{context, VerboseError},
    // error::{context, ContextError, VerboseError},
    // Err,
    Parser,
};
use nom_locate::{position, LocatedSpan};
use nom_tracable::TracableInfo;

pub type IResult<'a, T = Token<'a>> = nom::IResult<Span<'a>, T, VerboseError<Span<'a>>>;
pub type Span<'a> = LocatedSpan<&'a str, TracableInfo>;

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
    pub variant: Variant<'a>,
}

impl<'a> Token<'a> {
    pub(crate) fn new(span: Span<'a>, variant: Variant<'a>) -> Self {
        Self { span, variant }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variant<'a> {
    Expression(Box<Token<'a>>),
    Comment(&'a str),
    EndOfExpression,
    Ident(&'a str),
    Whitespace,
    Literal(Literal),
    Container(Container<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    Null,
    Regex(String),
    String(String),
    Timestamp(String),
}

pub(crate) fn float<'a>(v: f64) -> Variant<'a> {
    Variant::Literal(Literal::Float(v))
}

pub(crate) fn integer<'a>(v: i64) -> Variant<'a> {
    Variant::Literal(Literal::Integer(v))
}

pub(crate) fn bool<'a>(v: bool) -> Variant<'a> {
    Variant::Literal(Literal::Bool(v))
}

pub(crate) fn null<'a>() -> Variant<'a> {
    Variant::Literal(Literal::Null)
}

pub(crate) fn string<'a>(v: impl Into<String>) -> Variant<'a> {
    Variant::Literal(Literal::String(v.into()))
}

pub(crate) fn timestamp<'a>(v: impl Into<String>) -> Variant<'a> {
    Variant::Literal(Literal::Timestamp(v.into()))
}

pub(crate) fn regex<'a>(v: impl Into<String>) -> Variant<'a> {
    Variant::Literal(Literal::Regex(v.into()))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Container<'a> {
    Group(Box<Token<'a>>),
    Block,
    Array,
    Object,
}

pub(crate) fn group(v: Token) -> Variant {
    Variant::Container(Container::Group(Box::new(v)))
}

pub fn token<'a, O, F, T>(
    ctx: &'static str,
    parser: F,
    tokenizer: T,
) -> impl FnMut(Span<'a>) -> IResult
where
    F: Parser<Span<'a>, O, VerboseError<Span<'a>>>,
    T: FnMut(O) -> (Span<'a>, Variant<'a>),
{
    let tokenize = move |(i, (span, variant))| Ok((i, Token::new(span, variant)));
    let mut parser = map(context(ctx, parser), tokenizer);
    move |i| {
        let (i, _) = position(i)?;
        parser.parse(i).and_then(tokenize)
    }
}

pub fn span(s: &str) -> Span {
    let info = TracableInfo::new().parser_width(64).fold("term");
    Span::new_extra(s, info)
}

pub fn span_sliced(s: &str, range: Range<usize>) -> Span {
    use nom::Slice;

    let info = TracableInfo::new().parser_width(64).fold("term");
    let span = Span::new_extra(s, info);

    span.slice(range)
}
