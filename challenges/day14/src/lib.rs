#![feature(array_windows, vec_push_within_capacity)]

use core::fmt;
use std::fmt::Write;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};
use parsers::{number, ParserExt};

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

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Grid);

impl ChallengeParser for Solution {
    fn parse(mut input: &'static str) -> IResult<&'static str, Self> {
        let mut grid = Grid::with_capacity(input.len());
        loop {
            let mut a;
            (input, a) = Point::parse(input)?;
            while let Some(i) = input.strip_prefix(" -> ") {
                let (i, b) = Point::parse(i)?;
                input = i;

                grid.draw_line(a, b);
                a = b;
            }

            input = input.strip_prefix('\n').unwrap();
            if input.is_empty() {
                break;
            }
        }

        Ok(("", Self(grid)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(mut self) -> Self::Output1 {
        self.0.fill_one()
    }

    type Output2 = usize;
    fn part_two(mut self) -> Self::Output2 {
        self.0.fill_two()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    grid: Vec<GridSpot>,
    maxy: u16,
    minx: u16,
    maxx: u16,
}

impl Point {
    /// determines the index in the idea of cantor positioning scheme
    /// 500,0 -> 0
    ///
    /// 499,1 -> 1
    /// 500,1 -> 2
    /// 501,1 -> 3
    ///
    /// 498,2 -> 4
    /// 499,2 -> 5
    /// 500,2 -> 6
    /// 501,2 -> 7
    /// 502,2 -> 8
    /// ...
    fn index(self) -> usize {
        let group = self.1 * self.1;
        let i = self.0 + self.1 - 500;
        (group + i) as usize
    }
}

impl Grid {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            grid: Vec::with_capacity(capacity),
            maxx: 501,
            minx: 500,
            maxy: 0,
        }
    }

    fn draw_line(&mut self, a: Point, b: Point) {
        let mut xs;
        let xe;
        let mut ys;
        let ye;
        let xi;
        let yi;
        if a.0 == b.0 {
            xi = 0;
            yi = 1;
            xs = a.0;
            xe = a.0 + 1;
            (ys, ye) = if a.1 > b.1 {
                (b.1, a.1 + 1)
            } else {
                (a.1, b.1 + 1)
            };
        } else {
            yi = 0;
            xi = 1;
            ys = a.1;
            ye = a.1 + 1;
            (xs, xe) = if a.0 > b.0 {
                (b.0, a.0 + 1)
            } else {
                (a.0, b.0 + 1)
            };
        }

        self.minx = u16::min(xs, self.minx);
        self.maxx = u16::max(xe, self.maxx);
        self.maxy = u16::max(ye - 1, self.maxy);
        while xs < xe && ys < ye {
            self.draw_point(Point(xs, ys), GridSpot::Rock);
            xs += xi;
            ys += yi;
        }
    }

    fn draw_point(&mut self, a: Point, s: GridSpot) {
        let i = a.index();
        if i >= self.grid.len() {
            self.grid.resize(i + 1, GridSpot::Air);
        }
        self.grid[i] = s;
    }

    fn fill_one(&mut self) -> usize {
        // fill rest of final row
        let i = (self.maxy + 1).pow(2) as usize;
        if i >= self.grid.len() {
            self.grid.resize(i + 1, GridSpot::Air);
        }

        let mut stack = Vec::with_capacity(self.maxy as usize);
        let mut settled = 0;

        let mut p = Point(500, 0);

        loop {
            if p.1 == self.maxy {
                // fallen off grid
                break;
            }

            let i = p.index();
            let y1 = p.1 + 1;
            let di = i + (y1 * 2) as usize;
            let [down_left, down, down_right]: [GridSpot; 3] =
                self.grid[di - 1..di + 2].try_into().unwrap();

            p = if down == GridSpot::Air {
                // we can move directly down
                let _ = stack.push_within_capacity(p);
                Point(p.0, y1)
            } else if p.0 == self.minx {
                // we can't move down, so we fall left off grid
                break;
            } else if down_left == GridSpot::Air {
                // we can't move down, so we fall left
                let _ = stack.push_within_capacity(p);
                Point(p.0 - 1, y1)
            } else if y1 == self.maxx {
                // we can't move down or left, so we fall right off grid
                break;
            } else if down_right == GridSpot::Air {
                // we can't move down or left, so we fall right
                let _ = stack.push_within_capacity(p);
                Point(p.0 + 1, y1)
            } else {
                // we can't move anywhere.
                settled += 1;
                self.grid[i] = GridSpot::Sand;

                match stack.pop() {
                    Some(i) => i,
                    // if nothing left in the stack, we must be at the source point of the sand.
                    None => break,
                }
            }
        }
        settled
    }

    fn fill_two(&mut self) -> usize {
        // fill rest of final row
        self.maxy += 2;
        let i = (self.maxy).pow(2) as usize;
        let j = (self.maxy + 1).pow(2) as usize;
        if j >= self.grid.len() {
            self.grid.resize(j + 1, GridSpot::Air);
        }
        self.grid[i..j].fill(GridSpot::Rock);
        // println!("{self}");

        let mut stack = Vec::with_capacity(self.maxy as usize);
        let mut settled = 0;

        let mut p = Point(500, 0);

        loop {
            if p.1 == self.maxy {
                // fallen off grid
                break;
            }

            let i = p.index();
            let y1 = p.1 + 1;
            let di = i + (y1 * 2) as usize;
            let [down_left, down, down_right]: [GridSpot; 3] =
                self.grid[di - 1..di + 2].try_into().unwrap();

            p = if down == GridSpot::Air {
                // we can move directly down
                let _ = stack.push_within_capacity(p);
                Point(p.0, y1)
            } else if down_left == GridSpot::Air {
                // we can't move down, so we fall left
                let _ = stack.push_within_capacity(p);
                Point(p.0 - 1, y1)
            } else if down_right == GridSpot::Air {
                // we can't move down or left, so we fall right
                let _ = stack.push_within_capacity(p);
                Point(p.0 + 1, y1)
            } else {
                // we can't move anywhere.
                settled += 1;
                self.grid[i] = GridSpot::Sand;

                match stack.pop() {
                    Some(i) => i,
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
        let mut start = 0;
        if self.grid.is_empty() {
            return Ok(());
        }
        for y in 1..self.maxy as usize + 2 {
            let end = y * y;
            let e = usize::min(end, self.grid.len());
            let mut row = &self.grid[start..e];
            start = end;

            let left = 500 + 1 - y;
            if self.minx as usize > left {
                row = &row[self.minx as usize - left..]
            } else {
                for _ in self.minx as usize..left {
                    f.write_char(' ')?;
                }
            }

            // jank lmao
            let width = usize::min(
                row.len(),
                (self.maxx as usize) - usize::max(self.minx as usize, left),
            );

            for spot in &row[..width] {
                spot.fmt(f)?;
            }
            f.write_char('\n')?;
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
        // println!("{output:?}");
        println!("{}", output.0);
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
