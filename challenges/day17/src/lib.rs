#![feature(iterator_try_reduce, control_flow_enum)]
use core::panic;
use std::ops::ControlFlow;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::prelude::{brent, floyd};

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
const PIECES: [([u8; 4], u8, u8); 5] = [
    ([0b00011110, 0b00000000, 0b00000000, 0b00000000], 4, 1), // horizontal line
    ([0b00001000, 0b00011100, 0b00001000, 0b00000000], 3, 3), // cross
    ([0b00011100, 0b00000100, 0b00000100, 0b00000000], 3, 3), // L
    ([0b00010000, 0b00010000, 0b00010000, 0b00010000], 1, 4), // vertical line
    ([0b00011000, 0b00011000, 0b00000000, 0b00000000], 2, 2), // square
];

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut bitset = Vec::<u8>::with_capacity(2022 * 7 / 2 / 32); // that is not a date
        bitset.resize(3 + 4, 0);

        // index in our jet cycle
        let mut i = 0;

        // current height of our stack
        let mut stack_height = 0usize;

        for piece in 0..2022 {
            // current offset from the left of our falling piece
            let mut x = 2;
            let mut y = stack_height + 3;
            let (mut bits, width, height) = PIECES[piece % 5];

            loop {
                // move horizontally
                {
                    let jet_left = self.0[i] == b'<';
                    i += 1;
                    i %= self.0.len();

                    let (x1, new_bits) = if jet_left && x > 0 {
                        (x - 1, bits.map(|x| x << 1))
                    } else if !jet_left && x + width < 7 {
                        (x + 1, bits.map(|x| x >> 1))
                    } else {
                        (x, bits)
                    };
                    let chunk = u32::from_ne_bytes(bitset[y..y + 4].try_into().unwrap());
                    let mask = u32::from_ne_bytes(new_bits);

                    if chunk & mask == 0 {
                        (x, bits) = (x1, new_bits);
                    }
                }

                // attempt to move down vertically
                {
                    if y > 0 {
                        let chunk = u32::from_ne_bytes(bitset[y - 1..y + 3].try_into().unwrap());
                        let mask = u32::from_ne_bytes(bits);
                        if chunk & mask == 0 {
                            y -= 1;
                            continue;
                        }
                    }

                    for (c, m) in bitset[y..y + 4].iter_mut().zip(bits) {
                        *c |= m;
                    }
                    stack_height = usize::max(stack_height, y + height as usize);
                    if bitset.len() < stack_height + 7 {
                        bitset.resize(stack_height + 7, 0)
                    }
                    break;
                }
            }
        }

        // for b in bitset.into_iter().rev() {
        //     println!("{b:07b}");
        // }

        stack_height
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut bitset = Vec::<u8>::with_capacity(self.0.len() * 360);
        bitset.resize(3 + 4, 0);

        // index in our jet cycle
        let mut i = 0;

        // current height of our stack
        let mut stack_height = 0usize;

        let mut total_block_falls = 1_000_000_000_000_usize; // trillion

        let mut indices = vec![];
        let mut heights = vec![];

        for piece in 0..self.0.len() * 60 {
            indices.push(i);
            heights.push(stack_height);

            // current offset from the left of our falling piece
            let mut x = 2;
            let mut y = stack_height + 3;
            let (mut bits, width, height) = PIECES[piece % 5];

            loop {
                // move horizontally
                {
                    let jet_left = self.0[i] == b'<';
                    i += 1;
                    i %= self.0.len();

                    let (x1, new_bits) = if jet_left && x > 0 {
                        (x - 1, bits.map(|x| x << 1))
                    } else if !jet_left && x + width < 7 {
                        (x + 1, bits.map(|x| x >> 1))
                    } else {
                        (x, bits)
                    };
                    let chunk = u32::from_ne_bytes(bitset[y..y + 4].try_into().unwrap());
                    let mask = u32::from_ne_bytes(new_bits);

                    if chunk & mask == 0 {
                        (x, bits) = (x1, new_bits);
                    }
                }

                // attempt to move down vertically
                {
                    if y > 0 {
                        let chunk = u32::from_ne_bytes(bitset[y - 1..y + 3].try_into().unwrap());
                        let mask = u32::from_ne_bytes(bits);
                        if chunk & mask == 0 {
                            y -= 1;
                            continue;
                        }
                    }

                    for (c, m) in bitset[y..y + 4].iter_mut().zip(bits) {
                        *c |= m;
                    }
                    stack_height = usize::max(stack_height, y + height as usize);
                    if bitset.len() < stack_height + 7 {
                        bitset.resize(stack_height + 7, 0)
                    }
                    break;
                }
            }
        }

        let mut cycle_len = 1;
        let value = loop {
            let mut i = 0;
            let (ControlFlow::Continue(Some(a)) | ControlFlow::Break(a)) = indices.rchunks_exact(cycle_len).try_reduce(|a, b| {
                if a == b {
                    i += 1;
                    ControlFlow::Continue(b)
                } else {
                    ControlFlow::Break(a)
                }
            }) else { panic!("sorry") };
            if i > 5 {
                println!("{i} {a:?}");
                break a;
            }
            cycle_len += 1;
            if cycle_len > indices.len() {
                panic!("sorry");
            }
        };

        let a = heights[heights.len()-1];
        let b = heights[heights.len()-value.len()-1];
        let height_per_cycle = a-b;// iterations * self.0.len();
        let blocks_per_cycle = value.len();

        dbg!(height_per_cycle, blocks_per_cycle, height_per_cycle%blocks_per_cycle);

        total_block_falls -= self.0.len() * 60;

        let in_cycle = total_block_falls / blocks_per_cycle;
        let out_of_cycle = total_block_falls % blocks_per_cycle;

        let extra_stack_height = dbg!(in_cycle * height_per_cycle);

        for piece in 0..out_of_cycle {
            // current offset from the left of our falling piece
            let mut x = 2;
            let mut y = stack_height + 3;
            let (mut bits, width, height) = PIECES[piece % 5];

            loop {
                // move horizontally
                {
                    let jet_left = self.0[i] == b'<';
                    i += 1;
                    i %= self.0.len();

                    let (x1, new_bits) = if jet_left && x > 0 {
                        (x - 1, bits.map(|x| x << 1))
                    } else if !jet_left && x + width < 7 {
                        (x + 1, bits.map(|x| x >> 1))
                    } else {
                        (x, bits)
                    };
                    let chunk = u32::from_ne_bytes(bitset[y..y + 4].try_into().unwrap());
                    let mask = u32::from_ne_bytes(new_bits);

                    if chunk & mask == 0 {
                        (x, bits) = (x1, new_bits);
                    }
                }

                // attempt to move down vertically
                {
                    if y > 0 {
                        let chunk = u32::from_ne_bytes(bitset[y - 1..y + 3].try_into().unwrap());
                        let mask = u32::from_ne_bytes(bits);
                        if chunk & mask == 0 {
                            y -= 1;
                            continue;
                        }
                    }

                    for (c, m) in bitset[y..y + 4].iter_mut().zip(bits) {
                        *c |= m;
                    }
                    stack_height = usize::max(stack_height, y + height as usize);
                    if bitset.len() < stack_height + 7 {
                        bitset.resize(stack_height + 7, 0)
                    }
                    break;
                }
            }
        }

        dbg!(stack_height + extra_stack_height)
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
        assert_eq!(output.part_two(), 1514285714288);
    }
}
