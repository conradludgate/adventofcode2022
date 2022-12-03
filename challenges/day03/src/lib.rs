#![feature(slice_as_chunks)]

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Day03(ArrayVec<(usize, usize), 300>);

impl ChallengeParser for Day03 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut lines = ArrayVec::new();
        for slice in input.as_bytes().split(|&x| x == b'\n') {
            let (a, b) = slice.split_at(slice.len() / 2);
            let _ = lines.try_push((bitset(a), bitset(b)));
        }
        Ok(("", Self(lines)))
    }
}

impl Challenge for Day03 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut errors = 0;
        for (a, b) in self.0 {
            errors += (a & b).trailing_zeros() as usize;
        }
        errors
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut badges = 0;
        for [(a1, a2), (b1, b2), (c1, c2)] in self.0.as_chunks().0 {
            badges += ((a1 | a2) & (b1 | b2) & (c1 | c2)).trailing_zeros() as usize;
        }
        badges
    }
}

fn bitset(x: &[u8]) -> usize {
    let mut set = 0;
    for &x in x {
        set |= 1 << if x >= b'a' { x - b'a' + 1 } else { x - b'A' + 27 };
    }
    set
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
