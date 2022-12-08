#![feature(vec_push_within_capacity)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Day08 {
    heights: &'static [u8],
    stride: usize,
}

impl ChallengeParser for Day08 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let line = input.as_bytes().iter().position(|&b| b == b'\n').unwrap_or(input.len());
        let stride = line + 1;

        Ok((
            "",
            Self {
                heights: input.as_bytes(),
                stride,
            },
        ))
    }
}

impl Challenge for Day08 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let width = self.stride - 1;
        let height = self.heights.len() / self.stride;

        let mut set = vec![0; self.heights.len()];

        // left-to-right
        for j in 1..height - 1 {
            let mut max = self.heights[j * self.stride];
            set[j * self.stride] = 1;
            for i in 1..width - 1 {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
            let mut max = self.heights[j * self.stride + self.stride - 2];
            set[j * self.stride + self.stride - 2] = 1;
            for i in (1..width - 1).rev() {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
        }

        // top-to-bottom
        for i in 1..width - 1 {
            let mut max = self.heights[i];
            set[i] = 1;
            for j in 1..height - 1 {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
            let mut max = self.heights[(height - 1) * self.stride + i];
            set[(height - 1) * self.stride + i] = 1;
            for j in (1..height - 1).rev() {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
        }

        set.into_iter().sum::<usize>() + 4 /* corners */
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut set = vec![1; self.heights.len()];
        let mut stack = Vec::<(usize, u8)>::with_capacity(usize::min(self.stride, self.heights.len() / self.stride));

        let mut i = 0;
        let mut line_start = 0;
        while i < self.heights.len() {
            let b = self.heights[i];

            if b == b'\n' {
                // reset row
                for (idx, _) in stack.drain(..) {
                    set[idx] *= i - idx - 1; // rightward view distance
                }

                line_start = i + 1;
            } else {
                let start = loop {
                    match stack.last() {
                        Some((_, v)) if *v <= b => {
                            let (idx, v) = stack.pop().unwrap();
                            set[idx] *= i - idx; // rightward view distance
                            if v == b {
                                break idx;
                            }
                        }
                        Some((idx, _)) => break *idx,
                        None => break line_start,
                    }
                };
                set[i] *= i - start; // leftward view distance

                let _ = stack.push_within_capacity((i, b));
            }

            i += 1;
        }

        i = 0;
        line_start = 0;
        loop {
            if i >= self.heights.len() {
                // reset column
                for (idx, _) in stack.drain(..) {
                    set[idx] *= (i - idx) / self.stride - 1; // downward view distance
                }
                i %= self.stride;
                i += 1;
                if self.heights[i] == b'\n' {
                    break;
                }
                line_start = i;
            } else {
                let b = self.heights[i];
                let start = loop {
                    match stack.last() {
                        Some((_, v)) if *v <= b => {
                            let (idx, v) = stack.pop().unwrap();
                            set[idx] *= (i - idx) / self.stride; // downward view distance
                            if v == b {
                                break idx;
                            }
                        }
                        Some((idx, _)) => break *idx,
                        None => break line_start,
                    }
                };
                set[i] *= (i - start) / self.stride; // upward view distance

                let _ = stack.push_within_capacity((i, b));
                i += self.stride;
            }
        }

        set.into_iter().max().unwrap_or(0)
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
