#![feature(array_chunks)]

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Day03<'i>(ArrayVec<&'i [u8], 300>);

impl<'i> ChallengeParser<'i> for Day03<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Self> {
        let mut lines = ArrayVec::new();
        for slice in input.as_bytes().split(|&x| x == b'\n') {
            let _ = lines.try_push(slice);
        }
        Ok(("", Self(lines)))
    }
}

impl<'i> Challenge for Day03<'i> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut errors = 0;
        for i in self.0 {
            let (a, b) = i.split_at(i.len() / 2);
            let a = bitset(a);
            let b = bitset(b);
            errors += (a & b).trailing_zeros() as usize;
        }
        errors
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut badges = 0;
        for [a, b, c] in self.0.array_chunks() {
            let a = bitset(a);
            let b = bitset(b);
            let c = bitset(c);
            badges += (a & b & c).trailing_zeros() as usize;
        }
        badges
    }
}

fn bitset(x: &[u8]) -> usize {
    let mut set = 0;
    const LUT: [u8; 256] = {
        let mut lut = [0; 256];
        let mut i = 0;
        while i < 26 {
            lut[(i + b'a') as usize] = i + 1;
            lut[(i + b'A') as usize] = i + 27;
            i += 1;
        }
        lut
    };

    for &x in x {
        set |= 1 << LUT[x as usize];
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
