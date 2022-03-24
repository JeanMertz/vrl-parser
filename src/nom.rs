use nom::{
    error::{ParseError, VerboseError, VerboseErrorKind},
    Compare, CompareResult, IResult, InputLength, InputTake, Parser,
};

/// Similar to [`ParseError`] and [`ContextError`], this trait allows a parser
/// to create an error representing an unmatched tag. This allows error
/// messages to produce more useful context about what went wrong.
pub(crate) trait TagError<T, I>: Sized {
    /// Create an error from an expected tag at a location.
    fn from_tag(input: I, tag: T) -> Self;

    /// As above, but for a case insensitive tag. By default this just
    /// calls from_tag
    fn from_no_case(input: I, tag: T) -> Self {
        Self::from_tag(input, tag)
    }
}

impl<I> TagError<&'static str, I> for VerboseError<I> {
    fn from_tag(input: I, tag: &'static str) -> Self {
        Self {
            errors: vec![(input, VerboseErrorKind::Context(tag))],
        }
    }
}

/// Enhanced tag parser that records the tag in the error in the event of
/// a parse failure via TagError
pub(crate) fn tag<T, I, E>(tag: T) -> impl Clone + Fn(I) -> IResult<I, I, E>
where
    T: InputLength + Clone,
    I: InputTake + Compare<T>,
    E: TagError<T, I>,
{
    move |input: I| match input.compare(tag.clone()) {
        CompareResult::Ok => Ok(input.take_split(tag.input_len())),
        _ => Err(nom::Err::Error(E::from_tag(input, tag.clone()))),
    }
}

/// Enhanced tag parser that records the tag in the error in the event of
/// a parse failure via TagError
#[allow(dead_code)]
pub(crate) fn tag_no_case<T, I, E>(tag: T) -> impl Clone + Fn(I) -> IResult<I, I, E>
where
    T: InputLength + Clone,
    I: InputTake + Compare<T>,
    E: TagError<T, I>,
{
    move |input: I| match input.compare_no_case(tag.clone()) {
        CompareResult::Ok => Ok(input.take_split(tag.input_len())),
        _ => Err(nom::Err::Error(E::from_no_case(input, tag.clone()))),
    }
}

/// Enhanced value parser that returns a tuple of the value, and the original
/// output.
pub(crate) fn value<I, O1: Clone, O2, E: ParseError<I>, F>(
    val: O1,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, (O2, O1), E>
where
    F: Parser<I, O2, E>,
{
    move |input: I| parser.parse(input).map(|(i, o)| (i, (o, val.clone())))
}
