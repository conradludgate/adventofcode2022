use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(u32, u32);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut nine: Vec<u64> = Vec::with_capacity(input.len().pow(2));
        let mut one: Vec<u64> = Vec::with_capacity(input.len().pow(2));
        let mut knots = [(0, 0); 10];
        one.push(1);
        nine.push(1);

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

                let i = index(knots[1]);
                if i >= one.len() * 64 {
                    one.resize((i / 64) + 1, 0);
                }
                one[i / 64] |= 1 << (i % 64);

                let i = index(knots[9]);
                if i >= nine.len() * 64 {
                    nine.resize((i / 64) + 1, 0);
                }
                nine[i / 64] |= 1 << (i % 64);
            }
        }

        Ok((
            "",
            Self(
                one.iter().map(|b| b.count_ones()).sum::<u32>(),
                nine.iter().map(|b| b.count_ones()).sum::<u32>(),
            ),
        ))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = u32;
    fn part_one(self) -> Self::Output1 {
        self.0
    }

    type Output2 = u32;
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

fn index((x, y): (i16, i16)) -> usize {
    let xa = x.unsigned_abs() as usize;
    let ya = y.unsigned_abs() as usize;
    let n = xa + ya;
    if n == 0 {
        return 0;
    }
    let m = 2 * n * (n - 1) + 1;

    let a = (x + y < n as i16) as usize;
    let a = a + (x < 0 || y == -(n as i16)) as usize;
    let a = a + (x < 0 && y > 0) as usize;

    m + n * a + xa
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
