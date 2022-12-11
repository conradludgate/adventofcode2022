#![feature(get_many_mut)]

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    IResult, Parser,
};
use parsers::{number, ParserExt};
use strength_reduce::StrengthReducedU64;

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
    items: ArrayVec<u64, 32>,
    op: Operation,
    test: StrengthReducedU64,
    throws: (usize, usize),
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
                test: StrengthReducedU64::new(test),
                throws: (throw1, throw2),
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
        let mut inspect_count = [0; 8];
        let relief = StrengthReducedU64::new(relief);
        let lcm = StrengthReducedU64::new(self.0.iter().map(|m| m.test.get()).product());

        for _ in 0..iterations {
            for i in 0..self.0.len() {
                let (j, k) = self.0[i].throws;
                let [monkey, j, k] = self.0.get_many_mut([i, j, k]).unwrap();
                for item in monkey.items.drain(..) {
                    inspect_count[i] += 1;
                    let worry = (monkey.op.apply(item) % lcm) / relief;
                    if worry % monkey.test == 0 { &mut *j } else { &mut *k }
                        .items
                        .push(worry)
                }
            }
        }

        inspect_count.select_nth_unstable(6);
        inspect_count[6] * inspect_count[7]
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
