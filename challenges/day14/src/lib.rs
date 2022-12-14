#![feature(array_windows, vec_push_within_capacity)]

use core::fmt;
use std::ops::Range;

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
        let (x, y) = Grid::bounds(&self.0);
        let mut grid = Grid::draw_rocks(x, y, &self.0, false);
        grid.fill()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let y = Grid::bounds(&self.0).1 + 2;
        let mut grid = Grid::draw_rocks(500 - y..500 + y, y, &self.0, true);
        grid.fill()
    }
}

struct Grid {
    grid: Vec<GridSpot>,
    x: Range<u16>,
    y: u16,
}

impl Grid {
    fn bounds(lines: &[LineSegment]) -> (Range<u16>, u16) {
        let mut xstart = 500;
        let mut xend = 501;
        let mut yend = 1;

        for &i in lines {
            if let LineSegment::Point(Point(x, y)) = i {
                xstart = u16::min(x, xstart);
                xend = u16::max(x + 1, xend);
                yend = u16::max(y + 1, yend);
            }
        }
        (xstart..xend, yend)
    }

    fn draw_rocks(x: Range<u16>, y: u16, lines: &[LineSegment], bottom: bool) -> Self {
        let width = x.len() as u16;
        let mut grid = vec![GridSpot::Air; (y * width) as usize];
        let xstart = x.start;

        for &[a, b] in lines.array_windows() {
            let (a, b) = match (a, b) {
                (LineSegment::Point(a), LineSegment::Point(b)) => (a, b),
                _ => continue,
            };

            if a.0 == b.0 {
                let x = a.0;
                let y = if a.1 > b.1 { b.1..=a.1 } else { a.1..=b.1 };
                for y in y {
                    grid[(y * width + x - xstart) as usize] = GridSpot::Rock;
                }
            } else if a.1 == b.1 {
                let y = a.1;
                let x = if a.0 > b.0 { b.0..=a.0 } else { a.0..=b.0 };
                for x in x {
                    grid[(y * width + x - xstart) as usize] = GridSpot::Rock;
                }
            }
        }

        if bottom {
            for x in 0..width {
                grid[((y - 1) * width + x) as usize] = GridSpot::Rock;
            }
        }

        Self { grid, x, y }
    }

    fn fill(&mut self) -> usize {
        let mut stack = Vec::with_capacity(self.y as usize);

        let Range { start, end } = self.x;
        let width = end - start;
        let mut settled = 0;

        let mut x = 500 - start;
        let mut y = 0;

        loop {
            if y + 1 == self.y {
                // fallen off grid
                break;
            }
            // k is the index of the point below our current x,y coordinate
            let k = ((y + 1) * width + x) as usize;

            (x, y) = if self.grid[k] == GridSpot::Air {
                // we can move directly down
                let _ = stack.push_within_capacity((x, y));
                (x, y + 1)
            } else if x == 0 {
                // we can't move down, so we fall left off grid
                break;
            } else if self.grid[k - 1] == GridSpot::Air {
                // we can't move down, so we fall left
                let _ = stack.push_within_capacity((x, y));
                (x - 1, y + 1)
            } else if x + 1 == end {
                // we can't move down or left, so we fall right off grid
                break;
            } else if self.grid[k + 1] == GridSpot::Air {
                // we can't move down or left, so we fall right
                let _ = stack.push_within_capacity((x, y));
                (x + 1, y + 1)
            } else {
                // we can't move anywhere.
                settled += 1;
                self.grid[(y * width + x) as usize] = GridSpot::Sand;

                match stack.pop() {
                    Some(p) => p,
                    // if nothing left in the stack, we must be at the source point of the sand.
                    None => break,
                }
            }
        }
        settled
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.chunks(self.x.len()) {
            for spot in row {
                spot.fmt(f)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
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
        assert_eq!(output.part_two(), 93);
    }
}
