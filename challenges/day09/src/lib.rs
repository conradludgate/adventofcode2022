use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(u32, u32);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut t1: Vec<u64> = Vec::with_capacity(8192);
        let mut t9: Vec<u64> = Vec::with_capacity(8192);
        let mut knots = [(0, 0); 10];

        for line in input.as_bytes().split(|b| *b == b'\n') {
            if line.len() < 3 {
                continue;
            }
            let (x, y) = match line[0] {
                b'U' => (0, 1),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                b'R' => (1, 0),
                _ => continue,
            };
            let mut distance = line[2] - b'0';
            if line.len() == 4 {
                distance = 10 * distance + (line[3] - b'0');
            }

            for _ in 0..distance {
                knots[0].0 += x;
                knots[0].1 += y;

                for i in 0..9 {
                    knots[i + 1] = drag_knot(knots[i], knots[i + 1]);
                }

                let i1 = index(knots[1]);
                let i9 = index(knots[9]);

                set_bit(&mut t1, i1);
                set_bit(&mut t9, i9);
            }
        }

        Ok((
            "",
            Self(
                t1.iter().map(|b| b.count_ones()).sum::<u32>(),
                t9.iter().map(|b| b.count_ones()).sum::<u32>(),
            ),
        ))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = u32;
    fn part_one(self) -> Self::Output1 {
        self.0
    }

    type Output2 = u32;
    fn part_two(self) -> Self::Output2 {
        self.1
    }
}

fn drag_knot(head: (i16, i16), tail: (i16, i16)) -> (i16, i16) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    let (dx, dy) = match (dx.abs(), dy.abs()) {
        (2, _) | (_, 2) => (dx.signum(), dy.signum()),
        _ => (0, 0),
    };

    (tail.0 + dx, tail.1 + dy)
}

/// determines a 1d index from the 2d coordinate. points closer to 0,0 are given smaller indices
fn index((x, y): (i16, i16)) -> usize {
    const XOR: [(i16, i16); 4] = [(0, 0), (-1, 0), (0, -1), (-1, -1)];
    let quad = quad(x, y);
    let (xorx, xory) = unsafe { *XOR.get_unchecked(quad) };
    let x = (x ^ xorx) as usize;
    let y = (y ^ xory) as usize;
    quad + cantor(x, y) * 4
}

/// determines the quadrant a coordinate pair is in. Not in correct order: (0, 2, 3, 1 clockwise from top-right)
fn quad(x: i16, y: i16) -> usize {
    // const QUAD_LUT: [u8; 4] = [0, 1, 3, 2];
    let x = x as u16;
    let y = y as u16;
    // unsafe { *QUAD_LUT.get_unchecked(((x >> 15) | ((y >> 14) & 0x2)) as usize)}
    ((x >> 15) | ((y >> 14) & 0x2)) as usize
}

/// determines the index in the cantor positioning scheme
fn cantor(x: usize, y: usize) -> usize {
    let sum = x + y;
    let tri = sum * (sum + 1) / 2;
    tri + y
}

fn set_bit(v: &mut Vec<u64>, i: usize) {
    let x = i / 64;
    let y = i % 64;

    if x >= v.len() {
        v.resize(1 + x, 0);
    }
    unsafe { *v.get_unchecked_mut(x) |= 1 << y }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 13);
    }

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT2).unwrap().1;
        assert_eq!(output.part_two(), 36);
    }
}
