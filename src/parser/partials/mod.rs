mod string_literal_inner;

pub(crate) use string_literal_inner::parse as string_literal_inner;

#[cfg(test)]
use super::TResult;

#[cfg(test)]
fn test(result: crate::token::IResult<String>) -> TResult<String> {
    use nom::Err;

    result.map(|v| v.1).map_err(|e| {
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
