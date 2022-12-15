use std::marker::PhantomData;

use crate::gen::separated_list1_inner;
use next_gen::gen_iter;
use nom::{
    error::{ErrorKind, ParseError},
    Err, InputLength, Parser,
};

pub struct SeperatedList1<F, G, O, O2, C> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<(O, O2, C)>,
}

impl<I, F, G, O, O2, C, E> Parser<I, C, E> for SeperatedList1<F, G, O, O2, C>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
    C: Default + Extend<O>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, C, E> {
        let mut res = C::default();
        let input = gen_iter! {
            for v in separated_list1_inner(input, &mut self.f, &mut self.g) {
                res.extend(Some(v));
            }
        }?;
        Ok((input, res))
    }
}

pub struct SeperatedList0<F, G, O2> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<O2>,
}

impl<I, F, G, O, O2, E> Parser<I, Vec<O>, E> for SeperatedList0<F, G, O2>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, Vec<O>, E> {
        let mut res = Vec::new();

        match self.f.parse(input.clone()) {
            Err(Err::Error(_)) => return Ok((input, res)),
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                input = i1;
            }
        }

        loop {
            let len = input.input_len();
            match self.g.parse(input.clone()) {
                Err(Err::Error(_)) => return Ok((input, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                    }

                    match self.f.parse(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((input, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            input = i2;
                        }
                    }
                }
            }
        }
    }
}

pub struct Many1<F> {
    pub(crate) f: F,
}

impl<I, F, O, E> Parser<I, Vec<O>, E> for Many1<F>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, Vec<O>, E> {
        let mut res = Vec::new();

        // Parse the first element
        let (i1, n) = self.f.parse(input)?;
        res.push(n);
        input = i1;

        loop {
            match self.f.parse(input.clone()) {
                Err(Err::Error(_)) => return Ok((input, res)),
                Err(e) => return Err(e),
                Ok((i1, o)) => {
                    res.push(o);
                    input = i1;
                }
            }
        }
    }
}
