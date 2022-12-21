use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<(u8, u8, u8)>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut points = Vec::with_capacity(input.len() / 6); // "a,b,c\n".len() == 6
        for line in input.lines() {
            let Some((x, yz)) = line.split_once(',') else { continue };
            let Some((y, z)) = yz.split_once(',') else { continue };
            points.push((x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()));
        }

        Ok(("", Self(points)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(mut self) -> Self::Output1 {
        let mut area = 6;
        self.0.sort_unstable_by_key(|&(a, b, c)| (a, b, c));
        self.0.iter().copied().reduce(|x, y| {
            let check_last = (x.0, x.1, x.2 + 1);
            if y != check_last {
                area += 2;
            }
            y
        });

        self.0.sort_unstable_by_key(|&(a, b, c)| (a, c, b));
        self.0.iter().copied().reduce(|x, y| {
            let check_last = (x.0, x.1 + 1, x.2);
            if y != check_last {
                area += 2;
            }
            y
        });

        self.0.sort_unstable_by_key(|&(a, b, c)| (b, c, a));
        self.0.iter().copied().reduce(|x, y| {
            let check_last = (x.0 + 1, x.1, x.2);
            if y != check_last {
                area += 2;
            }
            y
        });
        area
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

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 64);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
