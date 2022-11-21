use std::marker::PhantomData;

use nom::{error::ParseError, InputLength, Parser};

pub struct Skip<F, G, O2> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<O2>,
}

impl<I, F, G, O, O2, E> Parser<I, O, E> for Skip<F, G, O2>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, O, E> {
        let (input, output) = self.f.parse(input)?;
        let (input, _) = self.g.parse(input)?;
        Ok((input, output))
    }
}

pub struct PrecededBy<F, G, O2> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<O2>,
}

impl<I, F, G, O, O2, E> Parser<I, O, E> for PrecededBy<F, G, O2>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, O, E> {
        let (input, _) = self.g.parse(input)?;
        let (input, output) = self.f.parse(input)?;
        Ok((input, output))
    }
}
