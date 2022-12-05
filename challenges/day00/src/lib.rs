use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Day00<'i>(&'i str);

impl ChallengeParser for Day00<'static> {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        tag("").map(Self).parse(input)
    }
}

impl Challenge for Day00<'static> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        todo!()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day00;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "";

    #[test]
    fn parse() {
        let output = Day00::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day00::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 0);
    }

    #[test]
    fn part_two() {
        let output = Day00::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
