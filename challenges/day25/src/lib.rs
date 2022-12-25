use core::fmt;
use std::{convert::Infallible, str::FromStr};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Snafu);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let n = input
            .lines()
            .map(|line| line.parse::<Snafu>().unwrap())
            .reduce(|a, b| Snafu(a.0 + b.0))
            .unwrap();

        Ok(("", Self(n)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Snafu(i64);

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut space = [0; 32];
        let mut i = 32;

        let mut n = self.0;

        while n > 0 {
            let m = n % 5;
            const LUT: [u8; 5] = [b'0', b'1', b'2', b'=', b'-'];
            i -= 1;
            space[i] = LUT[m as usize];
            n = n / 5 + (m > 2) as i64;
        }

        let s = std::str::from_utf8(&space[i..]).map_err(|_| fmt::Error)?;
        f.write_str(s)
    }
}

impl FromStr for Snafu {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = 0;
        for b in s.bytes() {
            n *= 5;
            n += match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => 0,
            };
        }
        Ok(Self(n))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = Snafu;
    fn part_one(self) -> Self::Output1 {
        self.0
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

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "2=-1=0");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
