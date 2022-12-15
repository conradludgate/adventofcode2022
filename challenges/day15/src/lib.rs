use std::collections::HashSet;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
struct Positions {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Positions>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut positions = Vec::with_capacity(input.len() / 32);
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let line = line.strip_prefix("Sensor at x=").unwrap();
            let (x1, line) = line.split_once(", y=").unwrap();
            let (y1, line) = line.split_once(": closest beacon is at x=").unwrap();
            let (x2, y2) = line.split_once(", y=").unwrap();
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

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.part_one(2000000)
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        0
    }
}

impl Solution {
    fn part_one(self, row: i32) -> usize {
        let mut not_beacons = HashSet::new();
        for pos in &self.0 {
            let dist = pos.x1.abs_diff(pos.x2) + pos.y1.abs_diff(pos.y2);
            let offset = pos.y1.abs_diff(row);
            if offset > dist {
                continue;
            }
            let d = (dist-offset) as i32;
            for i in pos.x1-d..=pos.x1+d {
                not_beacons.insert(i);
            }

            // not_beacons += 2 * (dist - offset);
            // if pos.y2 != row {
            //     not_beacons += 1;
            // }
        }

        for pos in self.0 {
            if pos.y2 == row {
                not_beacons.remove(&pos.x2);
            }
        }

        not_beacons.len()
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
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(10), 26);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
