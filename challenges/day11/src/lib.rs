#![feature(slice_swap_unchecked)]

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Square,
    Mul(u64),
    Add(u64),
}
impl Operation {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((
            tag("new = old * old").map(|_| Self::Square),
            number.preceded_by(tag("new = old * ")).map(Self::Mul),
            number.preceded_by(tag("new = old + ")).map(Self::Add),
        ))
        .parse(input)
    }
    fn apply(self, x: u64) -> u64 {
        match self {
            Operation::Square => x * x,
            Operation::Mul(y) => x * y,
            Operation::Add(y) => x + y,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: ArrayVec<u64, 8>,
    op: Operation,
    test: Div,
    throws: [usize; 2],
    inspect: u32,
}

impl Monkey {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (input, _) = tag("Monkey ").parse(input)?;
        let (input, _) = take_until("items: ").parse(input)?;
        let (input, _) = tag("items: ").parse(input)?;
        let (input, items) = number.separated_list1(tag(", ")).parse(input)?;
        let (input, op) = Operation::parse.preceded_by(tag("\n  Operation: ")).parse(input)?;
        let (input, test) = number.preceded_by(tag("\n  Test: divisible by ")).parse(input)?;
        let (input, throw1) = number
            .preceded_by(tag("\n    If true: throw to monkey "))
            .parse(input)?;
        let (input, throw2) = number
            .preceded_by(tag("\n    If false: throw to monkey "))
            .parse(input)?;

        Ok((
            input,
            Self {
                items,
                op,
                test: Div::new(test),
                throws: [throw1, throw2],
                inspect: 0,
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Solution(ArrayVec<Monkey, 8>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Monkey::parse.separated_list1(tag("\n\n")).map(Self).parse(input)
    }
}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    fn solve(mut self, relief: u64, iterations: usize) -> usize {
        let relief = Div::new(relief);
        let lcm = Div::new(self.0.iter().map(|m| m.test.1).product());

        for (i, m) in self.0.iter().enumerate() {
            assert!(m.throws[0] < self.0.len());
            assert!(m.throws[1] < self.0.len());
            assert_ne!(m.throws[0], i);
            assert_ne!(m.throws[1], i);
        }

        // SAFETY: checked above
        unsafe { self.rounds(iterations, relief, lcm) }

        let mut max1 = 0;
        let mut max2 = 0;
        for monkey in self.0 {
            let min1 = u32::min(max1, monkey.inspect);
            max1 = u32::max(max1, monkey.inspect);
            max2 = u32::max(max2, min1);
        }

        (max1 as usize) * (max2 as usize)
    }

    /// # Safety
    /// all monkey throw indices should be within the bounds of the monkey array
    #[inline(never)]
    unsafe fn rounds(&mut self, rounds: usize, relief: Div, lcm: Div) {
        // let inspect = [0u32; 8];
        let mut items = [0u64; 24 * 8];
        let mut lengths = [0usize; 8];

        for (i, monkey) in self.0.iter().enumerate() {
            let len = monkey.items.len();
            *lengths.get_unchecked_mut(i) = len;
            let chunk = items.get_unchecked_mut(i * 24..i * 24 + len);
            chunk.copy_from_slice(&monkey.items)
        }

        for _round in 0..rounds {
            for (i, monkey) in self.0.iter_mut().enumerate() {
                let [j, k] = monkey.throws;

                let len = std::mem::take(lengths.get_unchecked_mut(i));
                monkey.inspect += len as u32;

                let item_set = items.get_unchecked_mut(i * 24..i * 24 + len);
                // let mut test = [0; 24];
                for item in item_set {
                    *item = monkey.op.apply(*item);
                    // ensure the worries stay bounded
                    *item = *item % lcm;
                    // apply the worry relief
                    if relief.1 > 1 {
                        *item = *item / relief;
                    }
                }

                let mut lenj = *lengths.get_unchecked(j);
                let mut lenk = *lengths.get_unchecked(k);

                for i in i * 24..i * 24 + len {
                    let item = *items.get_unchecked(i);
                    if item % monkey.test == 0 {
                        items.swap_unchecked(i, lenj + j * 24);
                        lenj += 1;
                    } else {
                        items.swap_unchecked(i, lenk + k * 24);
                        lenk += 1;
                    };
                }

                *lengths.get_unchecked_mut(j) = lenj;
                *lengths.get_unchecked_mut(k) = lenk;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Div(u64, u64);

impl Div {
    fn new(n: u64) -> Self {
        assert!(n > 0);

        if n.is_power_of_two() {
            Self(0, n)
        } else {
            Self(u64::MAX / n + 1, n)
        }
    }
}

impl Div {
    fn div_rem(self, numerator: u64) -> (u64, u64) {
        if numerator > 0xffff_ffff {
            (numerator / self.1, numerator % self.1)
        } else if self.0 == 0 {
            let mask = self.1 - 1;
            (numerator >> (mask.count_ones() as u64), numerator & mask)
        } else {
            let hi = self.0 >> 32;
            let lo = self.0 & 0xffffffff;

            let multiplied_hi = numerator.wrapping_mul(hi);
            let multiplied_lo = numerator.wrapping_mul(lo) >> 32;

            let quotient = multiplied_hi.wrapping_add(multiplied_lo) >> 32;
            let remainder = numerator - quotient * self.1;
            (quotient, remainder)
        }
    }
}

impl std::ops::Div<Div> for u64 {
    type Output = u64;

    fn div(self, rhs: Div) -> Self::Output {
        rhs.div_rem(self).0
    }
}
impl std::ops::Rem<Div> for u64 {
    type Output = u64;

    fn rem(self, rhs: Div) -> Self::Output {
        rhs.div_rem(self).1
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.solve(3, 20)
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.solve(1, 10000)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn parse() {
        let (input, output) = Solution::parse(INPUT).unwrap();
        println!("{input:?} {output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 10605);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 2713310158);
    }
}
