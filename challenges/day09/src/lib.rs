use std::collections::HashSet;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<(i32, i32, u32)>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut output = Vec::with_capacity(input.len() / 5);
        for line in input.as_bytes().split(|b| *b == b'\n') {
            if line.len() < 3 {
                continue;
            }
            let (x, y) = match line[0] {
                b'U' => (0, 1),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                b'R' => (1, 0),
                _ => panic!("{line:?}"),
            };
            let distance = match line {
                [_, _, a] => *a - b'0',
                [_, _, a, b] => 10 * (*a - b'0') + (*b - b'0'),
                _ => panic!("{line:?}"),
            } as u32;
            output.push((x, y, distance));
        }
        Ok(("", Self(output)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut data = HashSet::with_capacity(self.0.len());

        let mut head = (0, 0);
        let mut tail = (0, 0);
        data.insert(tail);

        for (x, y, dist) in self.0 {
            for _ in 0..dist {
                head.0 += x;
                head.1 += y;

                tail = match (head.0 - tail.0, head.1 - tail.1) {
                    // diagonals
                    (-2, -1) => (tail.0 - 1, tail.1 - 1),
                    (2, -1) => (tail.0 + 1, tail.1 - 1),
                    (-1, -2) => (tail.0 - 1, tail.1 - 1),
                    (-1, 2) => (tail.0 - 1, tail.1 + 1),
                    (-2, 1) => (tail.0 - 1, tail.1 + 1),
                    (2, 1) => (tail.0 + 1, tail.1 + 1),
                    (1, -2) => (tail.0 + 1, tail.1 - 1),
                    (1, 2) => (tail.0 + 1, tail.1 + 1),
                    // vertical
                    (0, -2) => (tail.0, tail.1 - 1),
                    (0, 2) => (tail.0, tail.1 + 1),
                    // horizontal
                    (-2, 0) => (tail.0 - 1, tail.1),
                    (2, 0) => (tail.0 + 1, tail.1),
                    // any others, don't move
                    _ => tail,
                };
                data.insert(tail);
            }
        }

        data.len()
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

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 13);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}