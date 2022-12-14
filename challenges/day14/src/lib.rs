#![feature(array_windows)]

use core::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use next_gen::gen_iter;
use nom::{bytes::complete::tag, IResult, Parser};
use parsers::{gen::separated_list1, number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(u16, u16);

impl Point {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        number
            .separated_array(tag(","))
            .map(|[x, y]| Self(x, y))
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LineSegment {
    Point(Point),
    Break,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<LineSegment>);

impl ChallengeParser for Solution {
    fn parse(mut input: &'static str) -> IResult<&'static str, Self> {
        let mut lines = Vec::new();
        loop {
            input = gen_iter! {
                for point in separated_list1(input, Point::parse, tag(" -> ")) {
                    lines.push(LineSegment::Point(point));
                }
            }?;

            input = input.strip_prefix('\n').unwrap();
            if input.is_empty() {
                break;
            }
            lines.push(LineSegment::Break);
        }

        Ok(("", Self(lines)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut xstart = 500;
        let mut xend = 501;
        let mut yend = 1;

        for &i in &self.0 {
            if let LineSegment::Point(Point(x, y)) = i {
                xstart = u16::min(x, xstart);
                xend = u16::max(x + 1, xend);
                // yrange.start = u16::min(y, yrange.start);
                yend = u16::max(y + 1, yend);
            }
        }
        let width = xend - xstart;

        dbg!(xstart..xend, yend);

        let mut grid = vec![GridSpot::Air; (width * yend) as usize];

        for &[a, b] in self.0.array_windows() {
            let (a, b) = match (a, b) {
                (LineSegment::Point(a), LineSegment::Point(b)) => (a, b),
                _ => continue,
            };

            if a.0 == b.0 {
                let x = a.0 - xstart;
                let y = if a.1 > b.1 { b.1..=a.1 } else { a.1..=b.1 };
                for y in y {
                    grid[(y * width + x) as usize] = GridSpot::Rock;
                }
            } else if a.1 == b.1 {
                let y = a.1;
                let x = if a.0 > b.0 {
                    (b.0 - xstart)..=(a.0 - xstart)
                } else {
                    (a.0 - xstart)..=(b.0 - xstart)
                };
                for x in x {
                    grid[(y * width + x) as usize] = GridSpot::Rock;
                }
            }
        }

        let mut i = 0;
        'outer: loop {
            let mut x = 500 - xstart;
            let mut y = 0;
            loop {
                if y + 1 == yend {
                    // fallen off grid
                    break 'outer;
                }
                let k = ((y + 1) * width + x) as usize;
                if grid[k] == GridSpot::Air {
                    y += 1;
                } else if x == 0 {
                    // fallen off grid
                    break 'outer;
                } else if grid[k - 1] == GridSpot::Air {
                    y += 1;
                    x -= 1;
                } else if x + 1 == xend {
                    // fallen off grid
                    break 'outer;
                } else if grid[k + 1] == GridSpot::Air {
                    y += 1;
                    x += 1;
                } else {
                    // settled
                    grid[(y * width + x) as usize] = GridSpot::Sand;
                    break
                }
            }

            i += 1;

            // for line in grid.chunks(width as usize) {
            //     for v in line {
            //         print!("{v}");
            //     }
            //     println!()
            // }
        }

        for line in grid.chunks(width as usize) {
            for v in line {
                print!("{v}");
            }
            println!()
        }

        i
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GridSpot {
    Sand,
    Rock,
    Air,
}
impl fmt::Display for GridSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridSpot::Sand => f.write_str("o"),
            GridSpot::Rock => f.write_str("#"),
            GridSpot::Air => f.write_str("."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 24);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
