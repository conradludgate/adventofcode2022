use aoc::{Challenge, Parser as ChallengeParser};
use nom::{character::complete::line_ending, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone)]
pub struct Day01(Vec<Vec<usize>>);

impl<'i> ChallengeParser<'i> for Day01 {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        number::<usize>
            .terminate_list1(line_ending)
            .separated_list1(line_ending)
            .map(Day01)
            .parse(input)
    }
}

impl Challenge for Day01 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.0.into_iter().map(|x| x.into_iter().sum::<usize>()).max().unwrap()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut x = self
            .0
            .into_iter()
            .map(|x| x.into_iter().sum::<usize>())
            .collect::<Vec<_>>();
        x.sort();
        x[x.len() - 3..].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn parse() {
        let output = Day01::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day01::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 24000);
    }

    #[test]
    fn part_two() {
        let output = Day01::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 45000);
    }
}
