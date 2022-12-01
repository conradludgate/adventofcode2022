use std::marker::PhantomData;

use nom::{
    error::{ErrorKind, ParseError},
    Err, InputLength, Parser,
};

pub struct TerminatedList1<F, G, O, O2, C> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<(O, O2, C)>,
}

impl<I, F, G, O, O2, C, E> Parser<I, C, E> for TerminatedList1<F, G, O, O2, C>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
    C: Default + Extend<O>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, C, E> {
        let mut res = C::default();

        // Parse the first element
        let n;
        (input, n) = self.f.parse(input)?;
        (input, _) = self.g.parse(input)?;
        res.extend(Some(n));

        loop {
            let n;
            (input, n) = match self.f.parse(input.clone()) {
                Ok((i, n)) => (i, n),
                Err(Err::Error(_)) => break,
                Err(e) => return Err(e),
            };

            let len = input.input_len();

            input = match self.g.parse(input.clone()) {
                Ok((i, _)) => i,
                Err(Err::Error(_)) => break,
                Err(e) => return Err(e),
            };

            // infinite loop check: the parser must always consume
            if input.input_len() == len {
                return Err(Err::Error(E::from_error_kind(input, ErrorKind::SeparatedList)));
            }

            res.extend(Some(n));
        }

        Ok((input, res))
    }
}

pub struct TerminatedList0<F, G, O2> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<O2>,
}

impl<I, F, G, O, O2, E> Parser<I, Vec<O>, E> for TerminatedList0<F, G, O2>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, Vec<O>, E> {
        let mut res = Vec::new();

        loop {
            let n;
            (input, n) = match self.f.parse(input.clone()) {
                Ok((i, n)) => (i, n),
                Err(Err::Error(_)) => break,
                Err(e) => return Err(e),
            };

            let len = input.input_len();

            input = match self.g.parse(input.clone()) {
                Ok((i, _)) => i,
                Err(Err::Error(_)) => break,
                Err(e) => return Err(e),
            };

            // infinite loop check: the parser must always consume
            if input.input_len() == len {
                return Err(Err::Error(E::from_error_kind(input, ErrorKind::SeparatedList)));
            }

            res.extend(Some(n));
        }

        Ok((input, res))
    }
}
