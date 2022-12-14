use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::prelude::brent;

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
            (i, stack_height) = self.drop_block(piece % 5, i, stack_height, &mut bitset)
        }

        stack_height
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let mut bitset = Vec::<u8>::with_capacity(self.0.len() * 360);
        bitset.resize(3 + 4, 0);

        // magic number that is enough for my input
        const MAX: usize = 35;
        let (len, s, mu) = brent((0, 0, 0, vec![0; 7]), |(p, i, sh, mut bs)| {
            let (i, mut sh) = self.drop_block(p, i, sh, &mut bs);
            if sh > MAX * 2 {
                sh -= MAX;
                bs.drain(..MAX);
            }
            ((p + 1) % 5, i, sh, bs)
        });

        let sh = s.2;
        let height_per_cycle = (0..len)
            .fold(s, |(p, i, sh, mut bs), _| {
                let (i, sh) = self.drop_block(p, i, sh, &mut bs);
                ((p + 1) % 5, i, sh, bs)
            })
            .2
            - sh;

        let total_block_falls = 1_000_000_000_000_usize - mu; // trillion
        let in_cycle = total_block_falls / len;
        let out_of_cycle = total_block_falls % len;
        let extra_stack_height = in_cycle * height_per_cycle;

        let initial =
            (0..mu + out_of_cycle).fold((0, 0, 0, vec![0; 7]), |(p, i, sh, mut bs), _| {
                let (i, sh) = self.drop_block(p, i, sh, &mut bs);
                ((p + 1) % 5, i, sh, bs)
            });

        initial.2 + extra_stack_height
    }
}

impl Solution {
    fn drop_block(
        &self,
        piece: usize,
        mut i: usize,
        stack_height: usize,
        bitset: &mut Vec<u8>,
    ) -> (usize, usize) {
        // current offset from the left of our falling piece
        let mut x = 2;
        let mut y = stack_height + 3;
        let (mut bits, width, height) = PIECES[piece];

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
                let stack_height = usize::max(stack_height, y + height as usize);
                if bitset.len() < stack_height + 7 {
                    bitset.resize(stack_height + 7, 0)
                }
                break (i, stack_height);
            }
        }
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
