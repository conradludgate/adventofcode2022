use next_gen::generator;
use nom::{
    error::{ErrorKind, ParseError},
    Err, InputLength, Parser,
};

#[generator(yield(O))]
pub fn separated_list1<I, O, O2, F, G, E>(
    mut input: I,
    mut f: F,
    mut g: G,
) -> Result<I, Err<E>>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    // Parse the first element
    let (i1, n) = f.parse(input)?;
    yield_!(n);
    input = i1;

    loop {
        let len = input.input_len();
        match g.parse(input.clone()) {
            Err(Err::Error(_)) => return Ok(input),
            Err(e) => return Err(e),
            Ok((i1, _)) => {
                // infinite loop check: the parser must always consume
                if i1.input_len() == len {
                    return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                }

                match f.parse(i1.clone()) {
                    Err(Err::Error(_)) => return Ok(input),
                    Err(e) => return Err(e),
                    Ok((i2, o)) => {
                        yield_!(o);
                        input = i2;
                    }
                }
            }
        }
    }
}
