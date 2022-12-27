#![feature(map_entry_replace)]
use std::collections::hash_map::Entry;

use aoc::{Challenge, Parser as ChallengeParser};
use fxhash::{FxHashMap, FxHashSet};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut x = 0_i16;
        let mut y = 0_i16;
        let mut points = FxHashSet::with_capacity_and_hasher(input.len(), Default::default());

        for b in input.bytes() {
            if b == b'#' {
                points.insert(P(x, y));
                x += 1;
            }
            if b == b'.' {
                x += 1;
            }
            if b == b'\n' {
                x = 0;
                y += 1;
            }
        }

        let len = points.len();

        const EIGHT: [P; 8] = [
            P(-1, -1),
            P(0, -1),
            P(1, -1),
            P(1, 0),
            P(1, 1),
            P(0, 1),
            P(-1, 1),
            P(-1, 0),
        ];

        const N: (usize, u8) = (1, 0b0000_0111);
        const E: (usize, u8) = (3, 0b0001_1100);
        const S: (usize, u8) = (5, 0b0111_0000);
        const W: (usize, u8) = (7, 0b1100_0001);

        let mut directions = [N, S, W, E];

        let mut considered = FxHashMap::with_capacity_and_hasher(len, Default::default());
        let mut duplicate = FxHashSet::with_capacity_and_hasher(len, Default::default());

        let mut round10 = 0;

        let mut round = 0;
        loop {
            round += 1;

            'points: for &point in &points {
                let inhabited = points.contains(&(point + EIGHT[0])) as u8
                    | (points.contains(&(point + EIGHT[1])) as u8) << 1
                    | (points.contains(&(point + EIGHT[2])) as u8) << 2
                    | (points.contains(&(point + EIGHT[3])) as u8) << 3
                    | (points.contains(&(point + EIGHT[4])) as u8) << 4
                    | (points.contains(&(point + EIGHT[5])) as u8) << 5
                    | (points.contains(&(point + EIGHT[6])) as u8) << 6
                    | (points.contains(&(point + EIGHT[7])) as u8) << 7;

                if inhabited > 0 {
                    for (mov, mask) in directions {
                        if inhabited & mask == 0 {
                            match considered.entry(point + EIGHT[mov]) {
                                Entry::Occupied(dupe) => {
                                    duplicate.insert(point);
                                    duplicate.insert(dupe.remove());
                                }
                                Entry::Vacant(v) => {
                                    v.insert(point);
                                }
                            }
                            continue 'points;
                        }
                    }
                }
                duplicate.insert(point);
            }
            if duplicate.len() == len {
                break;
            }

            for point in considered.drain() {
                duplicate.insert(point.0);
            }
            std::mem::swap(&mut points, &mut duplicate);
            duplicate.clear();

            directions.rotate_left(1);

            if round == 10 {
                let mut iter = points.iter().copied();
                let first = iter.next().unwrap();
                let (tl, br) = iter.fold((first, first), |(mut tl, mut br), p| {
                    tl.0 = i16::min(tl.0, p.0);
                    tl.1 = i16::min(tl.1, p.1);
                    br.0 = i16::max(br.0, p.0);
                    br.1 = i16::max(br.1, p.1);

                    (tl, br)
                });

                let width = (br.0.abs_diff(tl.0) + 1) as usize;
                let height = (br.1.abs_diff(tl.1) + 1) as usize;

                round10 = width * height - points.len();
            }
        }

        Ok(("", Self(round10, round)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct P(i16, i16);
impl std::ops::Add for P {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
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

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 110);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 20);
    }
}
