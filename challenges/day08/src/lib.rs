#![feature(vec_push_within_capacity)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Day08(usize, usize);

impl ChallengeParser for Day08 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let input = input.as_bytes();
        let line = input.iter().position(|&b| b == b'\n').unwrap_or(input.len());
        let stride = line + 1;

        let mut set = vec![(1, 0); input.len()];
        let mut stack = Vec::<(usize, u8)>::with_capacity(usize::min(stride, input.len() / stride));

        let mut i = 0;
        let mut line_start = 0;
        while i < input.len() {
            let b = input[i];

            if b == b'\n' {
                // reset row
                for (idx, _) in stack.drain(..) {
                    set[idx].0 *= i - idx - 1; // rightward view distance
                    set[idx].1 = 1; // this tree is visible from the right edge
                }

                line_start = i + 1;
            } else {
                let start = loop {
                    match stack.last() {
                        Some((_, v)) if *v <= b => {
                            let (idx, v) = stack.pop().unwrap();
                            set[idx].0 *= i - idx; // rightward view distance
                            if v == b {
                                break idx;
                            }
                        }
                        Some((idx, _)) => break *idx,
                        None => {
                            set[i].1 = 1; // this tree is visible from the left edge
                            break line_start;
                        }
                    }
                };
                set[i].0 *= i - start; // leftward view distance

                let _ = stack.push_within_capacity((i, b));
            }

            i += 1;
        }

        i = 0;
        line_start = 0;
        loop {
            if i >= input.len() {
                // reset column
                for (idx, _) in stack.drain(..) {
                    set[idx].0 *= (i - idx) / stride - 1; // downward view distance
                    set[idx].1 = 1; // this tree is visible from the bottom edge
                }
                i %= stride;
                i += 1;
                if input[i] == b'\n' {
                    break;
                }
                line_start = i;
            } else {
                let b = input[i];
                let start = loop {
                    match stack.last() {
                        Some((_, v)) if *v <= b => {
                            let (idx, v) = stack.pop().unwrap();
                            set[idx].0 *= (i - idx) / stride; // downward view distance
                            if v == b {
                                break idx;
                            }
                        }
                        Some((idx, _)) => break *idx,
                        None => {
                            set[i].1 = 1; // this tree is visible from the top edge
                            break line_start;
                        }
                    }
                };
                set[i].0 *= (i - start) / stride; // upward view distance

                let _ = stack.push_within_capacity((i, b));
                i += stride;
            }
        }
        let (view, seen) = set
            .into_iter()
            .fold((0, 0), |(max, sum), (view, seen)| (usize::max(max, view), sum + seen));

        Ok(("", Self(view, seen)))
    }
}

impl Challenge for Day08 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.1
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Day08;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn parse() {
        let output = Day08::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day08::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 21);
    }

    #[test]
    fn part_two() {
        let output = Day08::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 8);
    }
}
