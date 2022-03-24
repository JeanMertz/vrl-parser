mod comment;
mod container;
mod eoe;
mod expression;
mod ident;
mod literal;
pub(crate) mod partials;
mod whitespace;

pub use comment::parse as comment;
pub use container::parse as container;
pub use eoe::parse as eoe;
pub use expression::parse as expression;
pub use ident::parse as ident;
pub use literal::parse as literal;
pub use whitespace::parse as whitespace;

// ------------------------------------------------------------------------------

#[cfg(test)]
type TResult<'a, T = crate::token::Variant<'a>> = Result<T, String>;

#[cfg(test)]
fn test(result: crate::token::IResult) -> TResult {
    use nom::Err;

    result.map(|v| v.1.variant).map_err(|e| {
        nom_tracable::histogram();
        nom_tracable::cumulative_histogram();

        // Helpful during error debugging.
        println!("{e:#?}");

        match e {
            Err::Incomplete(..) => unreachable!(),
            Err::Error(e) => format!(
                "error at {}:{} {}: {:?}",
                e.errors[0].0.location_line(),
                e.errors[0].0.get_utf8_column(),
                e.errors[0].0.fragment(),
                e.errors[0].1,
            ),
            Err::Failure(e) => format!(
                "failure at {}:{} {}: {:?}",
                e.errors[0].0.location_line(),
                e.errors[0].0.get_utf8_column(),
                e.errors[0].0.fragment(),
                e.errors[0].1,
            ),
        }
    })
}
