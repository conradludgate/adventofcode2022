use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<isize>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let output = input.lines().flat_map(|line| line.parse().ok()).collect();
        Ok(("", Self(output)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = isize;
    fn part_one(self) -> Self::Output1 {
        let n = self.0.len();
        let mut mixed = self.0.iter().copied().enumerate().collect::<Vec<_>>();
        for i in 0..n {
            let j = mixed.iter().position(|x| x.0 == i).unwrap();
            let (i, v) = mixed.remove(j);

            let n = mixed.len();
            let j = n.wrapping_add_signed((v.wrapping_add_unsigned(j)) % (n as isize)) % n;

            mixed.insert(j, (i, v));
        }

        let Some(j) = mixed.iter().position(|x| x.1 == 0) else { return 0 };
        mixed[(j + 1000) % mixed.len()].1
            + mixed[(j + 2000) % mixed.len()].1
            + mixed[(j + 3000) % mixed.len()].1
    }

    type Output2 = isize;
    fn part_two(self) -> Self::Output2 {
        let n = self.0.len();
        let mut mixed = self.0.iter().map(|x| *x * 811589153).enumerate().collect::<Vec<_>>();
        for _ in 0..10 {
            for i in 0..n {
                let j = mixed.iter().position(|x| x.0 == i).unwrap();
                let (i, v) = mixed.remove(j);
    
                let n = mixed.len();
                let j = n.wrapping_add_signed((v.wrapping_add_unsigned(j)) % (n as isize)) % n;
    
                mixed.insert(j, (i, v));
            }
        }

        let Some(j) = mixed.iter().position(|x| x.1 == 0) else { return 0 };
        mixed[(j + 1000) % mixed.len()].1
            + mixed[(j + 2000) % mixed.len()].1
            + mixed[(j + 3000) % mixed.len()].1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 3);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 1623178306);
    }
}
