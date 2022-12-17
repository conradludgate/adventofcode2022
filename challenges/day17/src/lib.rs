use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(&'static [u8]);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Ok(("", Self(input.trim_end().as_bytes())))
    }
}

// stored as bitpattern, width, height.
// bit pattern starts assuming 2 from the left.
// bottom of the pattern is the left-most septet
// lines are length 7, but we pad them to 8
#[allow(clippy::unusual_byte_groupings)]
const PIECES: [(u32, u32, u32); 5] = [
    (0b00011110_00000000_00000000_00000000, 4, 1), // horizontal line
    (0b00001000_00011100_00001000_00000000, 3, 3), // cross
    (0b00011100_00000100_00000100_00000000, 3, 3), // L
    (0b00010000_00010000_00010000_00010000, 1, 4), // vertical line
    (0b00011000_00011000_00000000_00000000, 2, 2), // square
];

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = u32;
    fn part_one(self) -> Self::Output1 {
        let mut bitset = Vec::<u32>::with_capacity(2022 * 7 / 2 / 32); // that is not a date
        bitset.push(0);

        // index in our jet cycle
        let mut i = 0;

        // current height of our stack
        let mut stack_height = 0u32;

        for piece in 0..2022 {
            // current offset from the left of our falling piece
            let mut x = 2;
            let mut y = stack_height + 3;
            dbg!(y);
            let (bits, width, height) = PIECES[piece % 5];
            let (mut a, mut b) = match y % 4 {
                0 => (0, bits),
                1 => (bits >> 8, bits << 24),
                2 => (bits >> 16, bits << 16),
                3.. => (bits >> 24, bits << 8),
            };
            // let mut j = bitset.len() - 1;

            loop {
                // for p in b.to_le_bytes() {
                //     println!("{p:07b}");
                // }
                // for p in a.to_le_bytes() {
                //     println!("{p:07b}");
                // }
                // println!();

                let mut j = (y / 4) as usize;
                // move horizontally
                {
                    let jet_left = self.0[i] == b'<';
                    i += 1;
                    i %= self.0.len();

                    let (x1, a1, b1) = if jet_left && x > 0 {
                        (x - 1, a << 1, b << 1)
                    } else if !jet_left && x + width < 7 {
                        (x + 1, a >> 1, b >> 1)
                    } else {
                        (x, a, b)
                    };
                    if j >= bitset.len() || a1 & bitset[j] == 0 {
                        (a, b, x) = (a1, b1, x1)
                    }
                }

                // for p in b.to_le_bytes() {
                //     println!("{p:07b}");
                // }
                // for p in a.to_le_bytes() {
                //     println!("{p:07b}");
                // }
                // println!();

                // attempt to move down vertically
                {
                    if y > 0 {
                        let (a1, b1) = if (a >> 24) > 0 {
                            (a >> 24, a << 8)
                        } else {
                            (a << 8 | b >> 24, b << 8)
                        };
                        if j >= bitset.len() || a1 & bitset[j] == 0 {
                            y -= 1;
                            (a, b) = (a1, b1)
                        } else {
                            stack_height = y + height;
                            bitset[j] |= a;
                            if j + 1 == bitset.len() {
                                bitset.push(b);
                            } else {
                                bitset[j + 1] |= b;
                            }
                            break;
                        }
                    } else {
                        stack_height = height;
                        bitset[0] |= a;
                        break;
                    }
                }
            }
        }

        // for b in bitset.into_iter().rev() {
        //     for p in b.to_le_bytes() {
        //         println!("{p:07b}");
        //     }
        // }

        stack_height
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 3068);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
