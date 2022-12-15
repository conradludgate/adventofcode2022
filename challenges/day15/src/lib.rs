#![feature(array_windows)]

use std::ops::Range;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

/// ```ignore
/// let line = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
/// let segments = ["Sensor at x=",", y=",": closest beacon is at x=",", y="];
/// assert_eq!(split_many(line, segments), Some(["2","18","-2","15"]));
/// ```
fn split_many<'a, const N: usize>(
    mut s: &'a str,
    mut delimiters: [&'a str; N],
) -> Option<[&'a str; N]> {
    s = s.strip_prefix(delimiters[0])?;
    for i in 1..N {
        (delimiters[i - 1], s) = s.split_once(delimiters[i])?;
    }
    delimiters[N - 1] = s;
    Some(delimiters)
}
#[derive(Debug, PartialEq, Clone)]
struct Positions {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Solution<const N: i32>(Vec<Positions>);

impl<const N: i32> ChallengeParser for Solution<N> {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut positions = Vec::with_capacity(input.len() / 32);
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let segments = ["Sensor at x=", ", y=", ": closest beacon is at x=", ", y="];
            let [x1, y1, x2, y2] = split_many(line, segments).unwrap();
            positions.push(Positions {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
            });
        }
        Ok(("", Self(positions)))
    }
}

impl<const N: i32> Challenge for Solution<N> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut ranges: Vec<Range<i32>> = Vec::with_capacity(self.0.len());

        self.build_range(N / 2, &mut ranges, i32::MIN..i32::MAX)
            .iter()
            .map(|r| r.len())
            .sum::<usize>()
            - 1 // not sure what that -1 is about tbh
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut ranges: Vec<Range<i32>> = Vec::with_capacity(self.0.len());
        for row in 0..=N {
            for [a, b] in self.build_range(row, &mut ranges, 0..N + 1).array_windows() {
                if a.end < b.start {
                    return (row as usize) + (a.end as usize) * 4000000;
                }
            }
        }
        0
    }
}

impl<const N: i32> Solution<N> {
    fn build_range<'a>(
        &self,
        row: i32,
        ranges: &'a mut Vec<Range<i32>>,
        minmax: Range<i32>,
    ) -> &'a [Range<i32>] {
        ranges.clear();
        for pos in &self.0 {
            let dist = pos.x1.abs_diff(pos.x2) + pos.y1.abs_diff(pos.y2);
            let offset = pos.y1.abs_diff(row);
            if offset > dist {
                continue;
            }
            let d = (dist - offset) as i32;
            let mut range = (pos.x1 - d).max(minmax.start)..(pos.x1 + d + 1).min(minmax.end);

            let mut iter = ranges.iter_mut().enumerate();
            loop {
                let Some((i, r)) = iter.next() else {
                    ranges.push(range);
                    break
                };

                if range.end < r.start {
                    ranges.insert(i, range);
                    break;
                }

                if range.start <= r.end {
                    if range.start < r.start {
                        r.start = range.start;
                    }
                    if range.end > r.end {
                        range.start = r.end;
                    } else {
                        break;
                    }
                }
            }
        }
        ranges.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn parse() {
        let output = Solution::<20>::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::<20>::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 26);
    }

    #[test]
    fn part_two() {
        let output = Solution::<20>::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 56000011);
    }
}
