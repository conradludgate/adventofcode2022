use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, character::complete::line_ending, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        number
            .separated_array(tag("-"))
            .map(|[start, end]| Range { start, end })
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day04(Vec<(Range, Range)>);

impl ChallengeParser for Day04 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Range::parse
            .separated_array(tag(","))
            .map(|[a, b]| (a, b))
            .separated_list1(line_ending)
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Day04 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut count = 0;
        for (a, b) in self.0 {
            if a.start <= b.start && b.end <= a.end || b.start <= a.start && a.end <= b.end {
                count += 1;
            }
        }
        count
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut count = 0;
        for (a, b) in self.0 {
            if a.start <= b.start && b.start <= a.end || b.start <= a.start && a.start <= b.end {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Day04;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn parse() {
        let output = Day04::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day04::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 2);
    }

    #[test]
    fn part_two() {
        let output = Day04::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 4);
    }
}
