use std::cmp;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{character::complete::line_ending, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone)]
pub struct Day01(Vec<usize>);

#[derive(Default)]
struct Sum(usize);

impl Extend<usize> for Sum {
    fn extend<T: IntoIterator<Item = usize>>(&mut self, iter: T) {
        for i in iter {
            self.0 += i;
        }
    }
}

impl ChallengeParser for Day01 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        number::<usize> // numbers
            .terminate_list1(line_ending) // terminated by new lines
            .map(|Sum(s)| s) // which are summed together
            .separated_list1(line_ending) // number groups are separated by more lines
            .map(Self)
            .parse(input)
    }
}

impl Day01 {
    fn solve(mut self, n: usize) -> Vec<usize> {
        self.0.select_nth_unstable_by_key(n - 1, |x| cmp::Reverse(*x));
        self.0.truncate(n);
        self.0
    }
}

impl Challenge for Day01 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.solve(1).swap_remove(0)
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.solve(3).into_iter().sum()
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
        println!("{output:?}");
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
