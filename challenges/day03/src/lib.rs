#![feature(array_chunks)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    character::complete::{alpha1, line_ending},
    IResult, Parser,
};
use parsers::ParserExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Day03<'i>(Vec<&'i str>);

impl<'i> ChallengeParser<'i> for Day03<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        alpha1.separated_list1(line_ending).map(Self).parse(input)
    }
}

impl<'i> Challenge for Day03<'i> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut errors = 0;

        for i in self.0 {
            let (first, second) = i.split_at(i.len() / 2);
            let mut first = first.as_bytes().to_vec();
            first.sort();
            let mut last = 0;
            for b in first {
                if b == last {
                    continue;
                }
                last = b;
                if second.as_bytes().contains(&b) {
                    if b > b'Z' {
                        errors += (b - b'a') as usize + 1;
                    } else {
                        errors += (b - b'A') as usize + 27;
                    }
                }
            }
        }
        errors
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut badges = 0;

        'main: for [a, b, c] in self.0.array_chunks() {
            let mut a = a.as_bytes().to_vec();
            let mut b = b.as_bytes().to_vec();
            let mut c = c.as_bytes().to_vec();
            a.sort();
            b.sort();
            c.sort();
            let (mut i, mut j, mut k) = (0, 0, 0);
            while i < a.len() && j < b.len() && k < c.len() {
                if a[i] == b[j] && a[i] == c[k] {
                    let x = a[i];
                    if x > b'Z' {
                        badges += (x - b'a') as usize + 1;
                    } else {
                        badges += (x - b'A') as usize + 27;
                    }
                    continue 'main;
                } else if a[i] <= b[j] && a[i] <= c[k] {
                    i += 1;
                } else if b[j] <= a[i] && b[j] <= c[k] {
                    j += 1
                } else if c[k] <= a[i] && c[k] < b[j] {
                    k += 1;
                }
            }
        }
        badges
    }
}

#[cfg(test)]
mod tests {
    use super::Day03;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn parse() {
        let output = Day03::parse(INPUT).unwrap().1;
        println!("{:?}", output);
    }

    #[test]
    fn part_one() {
        let output = Day03::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 157);
    }

    #[test]
    fn part_two() {
        let output = Day03::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 70);
    }
}
