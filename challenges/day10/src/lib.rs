use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayString;
use nom::IResult;
use phf::{phf_map, Map};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(i32, ArrayString<8>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut x = 1;
        let mut cycle: usize = 1;
        let mut output = [0u32; 8];
        let mut total = 0;
        for line in input.lines() {
            let (cycles, newx) = if let Some(n) = line.strip_prefix("addx ") {
                (2, x + n.parse::<i32>().unwrap())
            } else {
                (1, x)
            };

            for _ in 0..cycles {
                let row = (cycle - 1) / 40;
                let col = (cycle - 1) % 40;

                // if center of screen, track signal strength
                if col + 1 == 20 {
                    total += x * cycle as i32;
                }

                let sprite = x - 1..=x + 1;
                if sprite.contains(&(col as i32)) {
                    let row = row % 6;
                    let chr = col / 5;
                    let col = col % 5;

                    output[chr] |= 1 << (row + col * 6);
                }

                cycle += 1;
            }

            x = newx;
        }

        let mut s = ArrayString::<8>::new();

        for char in output {
            static INT_MAP: Map<u32, char> = phf_map! {
                0b111110001001001001111110_u32 => 'A',
                0b011010100101100101111111_u32 => 'B',
                0b010010100001100001011110_u32 => 'C',
                0b011110100001100001111111_u32 => 'D',
                0b100001100101100101111111_u32 => 'E',
                0b000001000101000101111111_u32 => 'F',
                0b111010101001100001011110_u32 => 'G',
                0b111111000100000100111111_u32 => 'H',
                0b000000100001111111100001_u32 => 'I',
                0b011111100001100000010000_u32 => 'J',
                0b100001011010000100111111_u32 => 'K',
                0b100000100000100000111111_u32 => 'L',
                0b111111000110000110111111_u32 => 'M',
                0b111111011000000110111111_u32 => 'N',
                0b011110100001100001011110_u32 => 'O',
                0b000110001001001001111111_u32 => 'P',
                0b011110100001110001011110_u32 => 'Q',
                0b100110011001001001111111_u32 => 'R',
                0b010010101001100101010010_u32 => 'S',
                0b000000000001111111000001_u32 => 'T',
                0b011111100000100000011111_u32 => 'U',
                0b001111110000110000001111_u32 => 'V',
                0b111111011000011000111111_u32 => 'W',
                0b110011001100001100110011_u32 => 'X',
                0b000000000111111000000111_u32 => 'Y',
                0b100011100101101001110001_u32 => 'Z',
            };
            s.push(INT_MAP[&char]);
        }

        Ok(("", Self(total, s)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = i32;
    fn part_one(self) -> Self::Output1 {
        self.0
    }

    type Output2 = ArrayString<8>;
    fn part_two(self) -> Self::Output2 {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::Parser;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.0, 17020);
        assert_eq!(output.1.as_str(), "RLEZFLGE");
    }
}
