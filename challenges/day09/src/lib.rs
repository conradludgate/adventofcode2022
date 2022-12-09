use std::collections::HashSet;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut nine = HashSet::with_capacity_and_hasher(input.len(), fxhash::FxBuildHasher::default());
        let mut one = HashSet::with_capacity_and_hasher(input.len(), fxhash::FxBuildHasher::default());
        let mut knots = [(0, 0); 10];
        one.insert((0, 0));
        nine.insert((0, 0));

        for line in input.as_bytes().split(|b| *b == b'\n') {
            if line.len() < 3 {
                continue;
            }
            let (x, y) = match line[0] {
                b'U' => (0, 1),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                b'R' => (1, 0),
                _ => continue,
            };
            let mut distance = line[2] - b'0';
            if line.len() == 4 {
                distance = 10 * distance + (line[3] - b'0');
            }

            for _ in 0..distance {
                knots[0].0 += x;
                knots[0].1 += y;

                for i in 0..9 {
                    knots[i + 1] = drag_knot(knots[i], knots[i + 1]);
                }

                one.insert(knots[1]);
                nine.insert(knots[9]);
            }
        }

        Ok(("", Self(one.len(), nine.len())))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.0
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.1
    }
}

fn drag_knot(head: (i16, i16), tail: (i16, i16)) -> (i16, i16) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    let (dx, dy) = match (dx.abs(), dy.abs()) {
        (2, _) | (_, 2) => (dx.signum(), dy.signum()),
        _ => (0, 0),
    };

    (tail.0 + dx, tail.1 + dy)
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

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT2).unwrap().1;
        assert_eq!(output.part_two(), 36);
    }
}
