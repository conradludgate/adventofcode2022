use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::directed::bfs;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    map: &'static [u8],
    stride: usize,
    end: usize,
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        // memchr?
        let width = input.as_bytes().iter().position(|b| *b == b'\n').unwrap();
        let end = input.as_bytes().iter().position(|b| *b == b'E').unwrap();
        Ok((
            "",
            Self {
                map: input.as_bytes(),
                stride: width + 1,
                end,
            },
        ))
    }
}

impl Solution {
    fn solve(self, any: bool) -> usize {
        let Self { map, stride, end } = self;
        // pathfind from E to S (or any 'a' if the flag is set)
        bfs::bfs(
            &end,
            |&p| {
                let mut vp = map[p];
                if vp == b'E' {
                    vp = b'z'
                }
                // try step in any right, down, left, up direction
                [p + 1, p + stride, p.wrapping_sub(1), p.wrapping_sub(stride)]
                    .into_iter()
                    .filter(|&q| q < map.len())
                    .filter(move |&q| {
                        let mut vq = map[q];
                        if vq == b'S' {
                            vq = b'a'
                        }
                        vp != b'\n' && vp <= vq + 1
                    })
            },
            |p| map[*p] == b'S' || (any && map[*p] == b'a'),
        )
        .unwrap()
        .len()
            - 1
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.solve(false)
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.solve(true)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 31);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 29);
    }
}
