#![feature(array_chunks)]

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
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
            let (a, b) = i.split_at(i.len() / 2);
            let mut a = a.as_bytes().to_vec();
            let mut b = b.as_bytes().to_vec();
            let x = SharedIter::new([&mut a, &mut b]).next().unwrap();
            errors += value(x);
        }
        errors
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut badges = 0;
        for [a, b, c] in self.0.array_chunks() {
            let mut a = a.as_bytes().to_vec();
            let mut b = b.as_bytes().to_vec();
            let mut c = c.as_bytes().to_vec();
            let x = SharedIter::new([&mut a, &mut b, &mut c]).next().unwrap();
            badges += value(x);
        }
        badges
    }
}

fn value(x: u8) -> usize {
    if x > b'Z' {
        (x - b'a') as usize + 1
    } else {
        (x - b'A') as usize + 27
    }
}

/// an iterator that finds matches in a set of bytes
struct SharedIter<'a, const N: usize> {
    data: [&'a [u8]; N],
}

impl<'a, const N: usize> SharedIter<'a, N> {
    fn new(data: [&'a mut [u8]; N]) -> Self {
        SharedIter {
            data: data.map(|x| {
                x.sort();
                &*x
            }),
        }
    }
}

impl<const N: usize> Iterator for SharedIter<'_, N> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut splits = ArrayVec::<_, N>::new();
        // split off the first byte of each subslice
        for i in 0..N {
            let (a, rest) = self.data[i].split_first()?;
            splits.push((*a, rest));
        }
        loop {
            // find the min of all first elements
            let min = splits.iter().map(|x| x.0).min().unwrap();
            // if all elements equal min
            if splits.iter().all(|x| x.0 == min) {
                // progress all internal states
                for i in 0..N {
                    self.data[i] = splits[i].1;
                }
                return Some(min);
            }
            // progress each that matches min
            for i in 0..N {
                if splits[i].0 == min {
                    self.data[i] = splits[i].1;
                    let (a, rest) = self.data[i].split_first()?;
                    splits[i] = (*a, rest);
                }
            }
        }
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
