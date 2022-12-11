#![feature(portable_simd, slice_swap_unchecked, slice_split_at_unchecked)]

use std::simd::Simd;

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
    fn apply(self, x: Simd<u64, N>) -> Simd<u64, N> {
        match self {
            Operation::Square => x * x,
            Operation::Mul(y) => x * Simd::<u64, N>::splat(y),
            Operation::Add(y) => x + Simd::<u64, N>::splat(y),
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
        let mut items = [[Simd::<u64, N>::splat(0); M]; 8];
        let mut lengths = [0usize; 8];

        for (i, monkey) in self.0.iter().enumerate() {
            let len = monkey.items.len();
            *lengths.get_unchecked_mut(i) = len;
            let chunk = items.get_unchecked_mut(i);
            as_array(chunk)[..len].copy_from_slice(&monkey.items)
        }

        for _round in 0..rounds {
            for (i, monkey) in self.0.iter_mut().enumerate() {
                let [j, k] = monkey.throws;

                let len = std::mem::take(lengths.get_unchecked_mut(i));
                monkey.inspect += len as u32;

                let mut item_chunks = (*items.get_unchecked(i)).map(|item_chunk| {
                    let item_chunk = monkey.op.apply(item_chunk);
                    // ensure the worries stay bounded
                    let item_chunk = lcm.remn(item_chunk);
                    // apply the worry relief
                    relief.divn(item_chunk)
                });
                let mut test = item_chunks.map(|worry_chunk| monkey.test.remn(worry_chunk));

                // let worries = items[i];
                // // apply the worry operation
                // let worries = monkey.op.apply(worries);
                // // ensure the worries stay bounded
                // let worries = lcm.div_rem8(worries).1;
                // // apply the worry relief
                // let mut worries = relief.div_rem8(worries).0;
                // let mut test = monkey.test.div_rem8(worries).1;

                // partition the worries
                let (left, right) = {
                    let item_chunks = as_array(&mut item_chunks);
                    let test = as_array(&mut test);

                    let (item_chunks, _) = unsafe { item_chunks.split_at_mut_unchecked(len) };

                    let mut left = 0;
                    let mut right = len;
                    while right > left {
                        if *test.get_unchecked(left) != 0 {
                            right -= 1;
                            unsafe {
                                test.swap_unchecked(left, right);
                                item_chunks.swap_unchecked(left, right);
                            }
                        } else {
                            left += 1;
                        }
                    }

                    unsafe { item_chunks.split_at_unchecked(left) }
                };

                let lenj = *lengths.get_unchecked(j);
                let lenk = *lengths.get_unchecked(k);

                let endj = lenj + left.len();
                let endk = lenk + right.len();

                as_array(items.get_unchecked_mut(j))[lenj..endj].copy_from_slice(left);
                as_array(items.get_unchecked_mut(k))[lenk..endk].copy_from_slice(right);

                *lengths.get_unchecked_mut(j) = endj;
                *lengths.get_unchecked_mut(k) = endk;
            }
            // for i in 0..self.0.len() {
            //     println!("{} {i}: {:?}", round+1, &items[i].as_array()[..lengths[i]]);
            // }
            // if [1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000].contains(&(round+1)) {
            //     println!("round {}", round+1);
            //     for i in &self.0 {
            //         println!("{}", i.inspect);
            //     }
            // }
        }
    }
}

fn as_array(x: &mut [Simd<u64, N>; M]) -> &mut [u64; CAP] {
    debug_assert_eq!(
        std::mem::size_of::<[Simd<u64, N>; M]>(),
        std::mem::size_of::<[u64; CAP]>()
    );
    // x[0].as_mut_array()
    unsafe { &mut *(x as *mut [Simd<u64, N>; M] as *mut [u64; CAP]) }
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

const CAP: usize = 24;
const N: usize = 1;
const M: usize = CAP/N;

impl Div {
    // #[inline(never)]
    // fn div_rem8(self, numerator: u64x8) -> (u64x8, u64x8) {
    //     if self.0 == 0 {
    //         (
    //             numerator >> u64x8::splat(self.1.trailing_zeros() as u64),
    //             numerator & u64x8::splat(self.1 - 1),
    //         )
    //     } else {
    //         let hi = u64x8::splat(self.0 >> 32);
    //         let lo = u64x8::splat(self.0 & 0xffffffff);
    //         let shift = u64x8::splat(32);

    //         let multiplied_hi = numerator * hi;
    //         let multiplied_lo = (numerator * lo) >> shift;

    //         let quotient = (multiplied_hi + multiplied_lo) >> shift;
    //         let remainder = numerator - quotient * u64x8::splat(self.1);
    //         (quotient, remainder)
    //     }
    // }

    fn remn(self, numerator: Simd<u64, N>) -> Simd<u64, N> {
        numerator - self.divn(numerator) * Simd::<u64, N>::splat(self.1)
    }

    fn divn(self, numerator: Simd<u64, N>) -> Simd<u64, N> {
        if self.0 == 0 {
            numerator >> Simd::<u64, N>::splat(self.1.trailing_zeros() as u64)
        } else {
            let shift = Simd::<u64, N>::splat(32);
            let hi = Simd::<u64, N>::splat(self.0 >> 32);
            let lo = Simd::<u64, N>::splat(self.0 & 0xffffffff);

            let multiplied_hi = numerator * hi;
            let multiplied_lo = (numerator * lo) >> shift;

            (multiplied_hi + multiplied_lo) >> shift
        }
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
