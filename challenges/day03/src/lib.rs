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
            let a = a.as_bytes().to_vec();
            let b = b.as_bytes().to_vec();
            errors += value(SharedIter::new([a, b]).next().unwrap());
        }
        errors
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut badges = 0;
        for [a, b, c] in self.0.array_chunks() {
            let a = a.as_bytes().to_vec();
            let b = b.as_bytes().to_vec();
            let c = c.as_bytes().to_vec();
            let x = SharedIter::new([a, b, c]).next().unwrap();
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
struct SharedIter<const N: usize> {
    data: [Vec<u8>; N],
}

impl<const N: usize> SharedIter<N> {
    fn new(data: [Vec<u8>; N]) -> Self {
        SharedIter {
            data: data.map(|mut x| {
                x.sort();
                x
            }),
        }
    }
}

impl<const N: usize> Iterator for SharedIter<N> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut elems = ArrayVec::<u8, N>::new();
        // peek at the last byte
        for i in 0..N {
            elems.push(self.data[i].pop()?);
        }
        loop {
            // find the max of all first elements
            let max = elems.iter().copied().max().unwrap();

            // progress each that matches max
            let mut all_eq = true;
            let mut empty = false;
            for i in 0..N {
                if elems[i] == max {
                    if let Some(x) = self.data[i].pop() {
                        elems[i] = x
                    } else {
                        empty = true;
                    }
                } else {
                    all_eq = false
                }
            }
            // if all were equal min
            if all_eq {
                return Some(max);
            }
            // if one of the vecs was exhausted
            if empty {
                return None;
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
