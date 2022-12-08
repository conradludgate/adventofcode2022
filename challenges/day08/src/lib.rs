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
        for j in 1..height-1 {
            let mut max = self.heights[j*self.stride];
            set[j*self.stride] = 1;
            for i in 1..width-1 {
                let b = self.heights[j*self.stride + i];
                if b > max {
                    max = b;
                    set[j*self.stride + i] = 1;
                }
            }
            let mut max = self.heights[j*self.stride+self.stride-2];
            set[j*self.stride+self.stride-2] = 1;
            for i in (1..width-1).rev() {
                let b = self.heights[j*self.stride + i];
                if b > max {
                    max = b;
                    set[j*self.stride + i] = 1;
                }
            }
        }

        // top-to-bottom
        for i in 1..width-1 {
            let mut max = self.heights[i];
            set[i] = 1;
            for j in 1..height-1 {
                let b = self.heights[j*self.stride + i];
                if b > max {
                    max = b;
                    set[j*self.stride + i] = 1;
                }
            }
            let mut max = self.heights[(height-1)*self.stride + i];
            set[(height-1)*self.stride + i] = 1;
            for j in (1..height-1).rev() {
                let b = self.heights[j*self.stride + i];
                if b > max {
                    max = b;
                    set[j*self.stride + i] = 1;
                }
            }
        }

        set.into_iter().sum::<usize>() + 4 /* corners */
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        todo!()
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
        assert_eq!(output.part_two(), 0);
    }
}
