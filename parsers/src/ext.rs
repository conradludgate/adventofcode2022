use std::{marker::PhantomData, ops::{Range, RangeFrom, RangeTo}};

use nom::{error::{FromExternalError, ParseError}, IResult, Parser, character::complete::line_ending, Slice, InputIter, InputLength, Compare};

pub use self::{
    map_res::MapRes,
    separated_array::Array,
    separated_array::SeperatedArray,
    separated_list::Many1,
    separated_list::{SeperatedList0, SeperatedList1},
    skip::{PrecededBy, Skip},
    terminated_list::TerminatedList0,
    terminated_list::TerminatedList1,
};

mod map_res;
mod separated_array;
mod separated_list;
mod skip;
mod terminated_list;

pub struct Noop;

impl<I, E> Parser<I, (), E> for Noop {
    fn parse(&mut self, input: I) -> IResult<I, (), E> {
        Ok((input, ()))
    }
}

impl<I, O, E, P: Parser<I, O, E>> ParserExt<I, O, E> for P {}
pub trait ParserExt<I, O, E>: Parser<I, O, E> {
    fn map_res<G, O2, E2>(self, g: G) -> MapRes<Self, G, O>
    where
        E: FromExternalError<I, E2>,
        G: FnMut(O) -> Result<O2, E2>,
        Self: Sized,
    {
        MapRes {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn separated_list0<G, O2>(self, g: G) -> SeperatedList0<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        SeperatedList0 {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn separated_list1<G, O2, C>(self, g: G) -> SeperatedList1<Self, G, O, O2, C>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        SeperatedList1 {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn lines<O2, C>(self) -> TerminatedList1<Self, fn(input: I) -> IResult<I, I, E>, O, O2, C>
    where
        Self: Sized,
        I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
        I: InputIter + InputLength,
        I: Compare<&'static str>,
        E: ParseError<I>,
    {
        TerminatedList1 {
            f: self,
            g: line_ending,
            _output: PhantomData,
        }
    }

    fn terminate_list1<G, O2, C>(self, g: G) -> TerminatedList1<Self, G, O, O2, C>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        TerminatedList1 {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn many1(self) -> Many1<Self>
    where
        Self: Sized,
    {
        Many1 { f: self }
    }

    fn array<const N: usize>(self) -> Array<Self, N>
    where
        Self: Sized,
    {
        Array { f: self }
    }

    fn separated_array<G, O2, const N: usize>(self, g: G) -> SeperatedArray<Self, G, O2, N>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        SeperatedArray {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn followed_by<G, O2>(self, g: G) -> Skip<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        self.skip(g)
    }

    fn skip<G, O2>(self, g: G) -> Skip<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        Skip {
            f: self,
            g,
            _output: PhantomData,
        }
    }

    fn preceded_by<G, O2>(self, g: G) -> PrecededBy<Self, G, O2>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
    {
        PrecededBy {
            f: self,
            g,
            _output: PhantomData,
        }
    }
}
