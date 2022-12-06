#![feature(slice_as_chunks, portable_simd)]

use std::simd::{u8x16, Mask, Simd, SimdPartialEq, SimdPartialOrd};

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayString;
use nom::{bytes::complete::tag, character::streaming::line_ending, sequence::tuple, IResult, Parser};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Instruction {
    count: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        tuple((
            tag("move "),
            number::<u8>,
            tag(" from "),
            number::<u8>,
            tag(" to "),
            number::<u8>,
        ))
        .map(|(_, count, _, from, _, to)| Instruction {
            count,
            from: from - 1,
            to: to - 1,
        })
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day05 {
    data: &'static str,
    stack_count: usize,
    data_index_offsets: [usize; 9],
    instructions: Vec<Instruction>,
}

impl ChallengeParser for Day05 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let line_length = input.find('\n').unwrap() + 1;
        let block_length = input.find("\n\n").unwrap() + 1;
        let stack_count = line_length / 4;

        let (data, input) = input.split_at(block_length - line_length);
        let (_, input) = input.split_at(line_length + 1);

        // calculate lengths of each stack based on the data
        let mut data_index_offsets = [0; 9];
        for line in data.as_bytes().chunks(stack_count * 4) {
            for (stack, [_, b, _, _]) in line.as_chunks().0.iter().enumerate() {
                if *b == b' ' {
                    data_index_offsets[stack] += 1;
                }
            }
        }

        let (input, instructions) = Instruction::parse.separated_list1(line_ending).parse(input)?;
        Ok((
            input,
            Self {
                data,
                data_index_offsets,
                stack_count,
                instructions,
            },
        ))
    }
}

impl Day05 {
    fn solve_inner(&mut self, reverse: bool) -> (u8x16, u8x16) {
        // the value of stacks starts off representing the final state of our stacks.
        // as we run through out instructions backwards, we encode which stack each index is currently in
        let mut stacks = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        // offsets encodes where this same value is in the stack from the top (0 is the top)
        let mut offsets = u8x16::default();

        // walk backwards
        for inst in self.instructions.drain(..).rev() {
            let count = u8x16::splat(inst.count);
            let to = u8x16::splat(inst.to);
            let from = u8x16::splat(inst.from);

            let should_move_mask =
                offsets.simd_lt(count).to_int().cast::<u8>() & stacks.simd_eq(to).to_int().cast::<u8>();

            let mut slice = [0; 16];
            slice[inst.from as usize] = inst.count;
            slice[inst.to as usize] = 0u8.wrapping_sub(inst.count);

            offsets += u8x16::gather_or_default(&slice, stacks.cast());

            if reverse {
                offsets ^= should_move_mask;
            } else {
                offsets += should_move_mask & count;
            }

            stacks = should_move_mask & from | !should_move_mask & stacks;
        }

        (stacks, offsets)
    }

    fn answer(self, stacks: u8x16, offsets: u8x16) -> ArrayString<16> {
        let Self {
            data,
            data_index_offsets,
            stack_count,
            ..
        } = self;

        let mut mask = Simd::default();
        mask[..stack_count].fill(-1);
        let mask = Mask::from_int(mask);

        let index = Simd::gather_select(&data_index_offsets, mask, stacks.cast(), Default::default()) + offsets.cast();
        let index = index * Simd::splat(stack_count * 4) + stacks.cast::<usize>() * Simd::splat(4) + Simd::splat(1);

        let output = u8x16::gather_select(data.as_bytes(), mask, index, Default::default());

        let mut output = ArrayString::from_byte_string(output.as_array()).unwrap();
        output.truncate(stack_count);
        output
    }
}

impl Challenge for Day05 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = ArrayString<16>;
    fn part_one(mut self) -> Self::Output1 {
        let (stacks, offsets) = self.solve_inner(true);
        self.answer(stacks, offsets)
    }

    type Output2 = ArrayString<16>;
    fn part_two(mut self) -> Self::Output2 {
        let (stacks, offsets) = self.solve_inner(false);
        self.answer(stacks, offsets)
    }
}

#[cfg(test)]
mod tests {
    use super::Day05;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn parse() {
        let output = Day05::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day05::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().as_str(), "CMZ");
    }

    #[test]
    fn part_two() {
        let output = Day05::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().as_str(), "MCD");
    }
}
