use next_gen::{gen_iter, generator};
use nom::{
    error::{ErrorKind, ParseError},
    Err, InputLength, Parser,
};

#[generator(yield(O))]
pub fn separated_list1<I, O, O2, F, G, E>(input: I, mut f: F, mut g: G) -> Result<I, Err<E>>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    gen_iter! {
        for a in separated_list1_inner(input, &mut f, &mut g) {
            yield_!(a);
        }
    }
}

#[generator(yield(O))]
pub(crate) fn separated_list1_inner<I, O, O2, F, G, E>(
    input: I,
    f: &mut F,
    g: &mut G,
) -> Result<I, Err<E>>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    // Parse the first element
    let (mut input, o) = f.parse(input)?;
    yield_!(o);

    loop {
        match g.parse(input.clone()) {
            Err(Err::Error(_)) => return Ok(input),
            Err(e) => return Err(e),
            Ok((i, _)) => {
                // infinite loop check: the parser must always consume
                if i.input_len() == input.input_len() {
                    return Err(Err::Error(E::from_error_kind(i, ErrorKind::SeparatedList)));
                }
                match f.parse(i) {
                    Err(Err::Error(_)) => return Ok(input),
                    Err(e) => return Err(e),
                    Ok((i, o)) => {
                        yield_!(o);
                        input = i;
                    }
                }
            }
        }
    }
}

#[generator(yield((O, O)))]
// parses [f, g, f, g, f, g, f] and returns each consecutive pair of f. like separated_list1 but requires at least 2 f parses
pub fn separated_pairs<I, O, O2, F, G, E>(mut input: I, mut f: F, mut g: G) -> Result<I, Err<E>>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
    O: Clone,
{
    // Parse the first pair
    let mut a;
    (input, a) = f.parse(input)?;
    (input, _) = g.parse(input)?;

    gen_iter! {
        for b in separated_list1(input, f, g) {
            let a = std::mem::replace(&mut a, b.clone());
            yield_!((a, b));
        }
    }
}
