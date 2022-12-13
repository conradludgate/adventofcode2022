#![feature(vec_push_within_capacity)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(u32, u32);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let input = input.as_bytes();
        let line = input
            .iter()
            .position(|&b| b == b'\n')
            .unwrap_or(input.len());
        let stride = line + 1;

        let width = stride - 1;
        let height = input.len() / stride;

        let mut set = vec![(1, 0); input.len()];
        let mut row_stack = Vec::<u32>::with_capacity(height);

        let mut col_stacks_idx = vec![0u32; stride - 1];
        let mut col_stacks = vec![0u32; width * height];

        let mut row_start = 0;
        for (i, b) in input.iter().copied().enumerate() {
            if b == b'\n' {
                // finish row
                for idx in row_stack.drain(..) {
                    set[idx as usize].0 *= i as u32 - idx - 1; // rightward view distance
                    set[idx as usize].1 = 1; // this tree is visible from the right edge
                }

                // reset row_start
                row_start = i + 1;
            } else {
                // deal with this entry in the row
                let start = loop {
                    match row_stack.last() {
                        Some(idx) if input[*idx as usize] <= b => {
                            let idx = row_stack.pop().unwrap();
                            let v = input[idx as usize];
                            set[idx as usize].0 *= i as u32 - idx; // rightward view distance
                            if v == b {
                                break idx;
                            }
                        }
                        Some(idx) => break *idx,
                        None => {
                            set[i].1 = 1; // this tree is visible from the left edge
                            break row_start as u32;
                        }
                    }
                };
                set[i].0 *= i as u32 - start; // leftward view distance

                let _ = row_stack.push_within_capacity(i as u32);

                // deal with this entry in the col
                let col = i % stride;
                let col_stack_idx = &mut col_stacks_idx[col];
                let col_stack = col_stacks.chunks_exact_mut(height).nth(col).unwrap();
                let start = loop {
                    if *col_stack_idx >= 1 {
                        let idx = col_stack[*col_stack_idx as usize - 1];
                        let v = input[idx as usize];
                        if v <= b {
                            *col_stack_idx -= 1;
                            set[idx as usize].0 *= (i as u32 - idx) / stride as u32;
                            // downward view distance
                        }
                        if v >= b {
                            break idx;
                        }
                    } else {
                        set[i].1 = 1; // this tree is visible from the top edge
                        break col as u32;
                    }
                };
                set[i].0 *= (i as u32 - start) / stride as u32; // upward view distance

                col_stack[*col_stack_idx as usize] = i as u32;
                *col_stack_idx += 1;
            }
        }

        // finish cols
        for (i, (col_stack, idx)) in col_stacks
            .chunks_mut(height)
            .zip(col_stacks_idx)
            .enumerate()
        {
            let i = input.len() + i;
            for idx in col_stack[..idx as usize].iter().copied() {
                set[idx as usize].0 *= (i as u32 - idx) / stride as u32 - 1; // downward view distance
                set[idx as usize].1 = 1; // this tree is visible from the bottom edge
            }
        }

        let (view, seen) = set.into_iter().fold((0, 0), |(max, sum), (view, seen)| {
            (u32::max(max, view), sum + seen)
        });

        Ok(("", Self(view, seen)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = u32;
    fn part_one(self) -> Self::Output1 {
        self.1
    }

    type Output2 = u32;
    fn part_two(self) -> Self::Output2 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 21);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 8);
    }
}
