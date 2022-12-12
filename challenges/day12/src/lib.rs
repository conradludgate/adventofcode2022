use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::directed::{astar, dijkstra};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    map: &'static [u8],
    width: usize,
    // height => map.len() / (width + 1)
    start: usize, // as a 1d index in map
    end: usize,   // as a 1d index in map
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        // memchr?
        let width = input.as_bytes().iter().position(|b| *b == b'\n').unwrap();
        let start = input.as_bytes().iter().position(|b| *b == b'S').unwrap();
        let end = input.as_bytes().iter().position(|b| *b == b'E').unwrap();
        Ok((
            "",
            Self {
                map: input.as_bytes(),
                width,
                start,
                end,
            },
        ))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let Self { map, width, end, start } = self;
        let stride = width + 1;
        astar::astar(
            &start,
            |&p| {
                [p + 1, p + stride, p.wrapping_sub(1), p.wrapping_sub(stride)]
                    .into_iter()
                    // .inspect(|q| println!("{q}"))
                    .filter(move |&q| {
                        if q >= map.len() {
                            return false;
                        }
                        let mut vp = map[p];
                        let mut vq = map[q];
                        if vp == b'S' {
                            vp = b'a'
                        }
                        if vq == b'E' {
                            vq = b'z'
                        }
                        vq != b'\n' && vq <= vp + 1
                    })
                    .map(|q| (q, 1))
            },
            |p| {
                let dist = end.abs_diff(*p);
                dist / stride + dist % stride
            },
            |p| *p == end,
        )
        .unwrap()
        .1
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let Self { map, width, end, .. } = self;
        let stride = width + 1;
        dijkstra::dijkstra(
            &end,
            |&p| {
                [p + 1, p + stride, p.wrapping_sub(1), p.wrapping_sub(stride)]
                    .into_iter()
                    .filter(move |&q| {
                        if q >= map.len() {
                            return false;
                        }
                        let mut vp = map[p];
                        let mut vq = map[q];
                        if vp == b'E' {
                            vp = b'z'
                        }
                        if vq == b'S' {
                            vq = b'a'
                        }
                        vp != b'\n' && vp <= vq + 1
                    })
                    .map(|q| (q, 1))
            },
            |p| map[*p] == b'S' || map[*p] == b'a',
        )
        .unwrap()
        .1
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
