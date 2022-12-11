use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::streaming::line_ending,
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Square,
    Mul(u32),
    Add(u32),
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
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: ArrayVec<u32, 64>,
    op: Operation,
    test: u32,
    throws: (usize, usize),
}

impl Monkey {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (input, _) = tag("Monkey ").parse(input)?;
        let (input, _) = take_until("items: ").parse(input)?;
        let (input, _) = tag("items: ").parse(input)?;
        let (input, items) = number.separated_list1(tag(", ")).parse(input)?;
        let (input, op) = Operation::parse.preceded_by(tag("\nOperation: ")).parse(input)?;
        let (input, test) = number.preceded_by(tag("\nTest: divisible by ")).parse(input)?;
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
                test,
                throws: (throw1, throw2),
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(ArrayVec<Monkey, 8>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Monkey::parse.separated_list1(tag("\n\n")).map(Self).parse(input)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        0
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        0
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
        assert_eq!(output.part_one(), 0);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
