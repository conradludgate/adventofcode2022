use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, sequence::separated_pair,
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(usize)]
enum Rps {
    R = 1,
    P = 2,
    S = 3,
}

impl Rps {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("A").map(|_| Rps::R),
            tag("B").map(|_| Rps::P),
            tag("C").map(|_| Rps::S),
        ))
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(usize)]
enum Goal {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Goal {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("X").map(|_| Goal::Lose),
            tag("Y").map(|_| Goal::Draw),
            tag("Z").map(|_| Goal::Win),
        ))
        .parse(input)
    }
}

impl Goal {
    fn part1(self) -> Rps {
        match self {
            Goal::Lose => Rps::R,
            Goal::Draw => Rps::P,
            Goal::Win => Rps::S,
        }
    }
    fn score_against(self, against: Rps) -> usize {
        // magic
        (self as usize / 3 + against as usize + 1) % 3 + 1 + self as usize
    }
}
impl Rps {
    fn score_against(self, against: Self) -> usize {
        // magic
        (self as isize - against as isize + 4) as usize % 3 * 3 + self as usize
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<(Rps, Goal)>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        separated_pair(Rps::parse, tag(" "), Goal::parse)
            .separated_list1(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.0
            .into_iter()
            .map(|(against, player)| player.part1().score_against(against))
            .sum()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.0
            .into_iter()
            .map(|(against, goal)| goal.score_against(against))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 15);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 12);
    }
}
