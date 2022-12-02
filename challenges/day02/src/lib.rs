use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, sequence::separated_pair, IResult, Parser,
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
    fn should_play(self, against: Rps) -> Rps {
        match (self, against) {
            (Goal::Lose, Rps::P) | (Goal::Draw, Rps::R) | (Goal::Win, Rps::S) => Rps::R,
            (Goal::Lose, Rps::S) | (Goal::Draw, Rps::P) | (Goal::Win, Rps::R) => Rps::P,
            (Goal::Lose, Rps::R) | (Goal::Draw, Rps::S) | (Goal::Win, Rps::P) => Rps::S,
        }
    }
}
impl Rps {
    fn outcome(self, against: Self) -> Goal {
        match (self, against) {
            (Rps::R, Rps::R) | (Rps::P, Rps::P) | (Rps::S, Rps::S) => Goal::Draw,
            (Rps::R, Rps::P) | (Rps::P, Rps::S) | (Rps::S, Rps::R) => Goal::Lose,
            (Rps::R, Rps::S) | (Rps::P, Rps::R) | (Rps::S, Rps::P) => Goal::Win,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day02(Vec<(Rps, Goal)>);

impl<'i> ChallengeParser<'i> for Day02 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        separated_pair(Rps::parse, tag(" "), Goal::parse)
            .separated_list1(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Day02 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.0
            .into_iter()
            .map(|(against, player)| player.part1() as usize + player.part1().outcome(against) as usize)
            .sum()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.0
            .into_iter()
            .map(|(against, goal)| goal as usize + goal.should_play(against) as usize)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day02;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn parse() {
        let output = Day02::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day02::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 15);
    }

    #[test]
    fn part_two() {
        let output = Day02::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 12);
    }
}
