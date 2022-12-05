#![feature(slice_as_chunks)]

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
    fn solve(&mut self, reverse: bool) -> ([u8; 16], [u8; 16]) {
        // the value of stacks starts off representing the final state of our stacks.
        // as we run through out instructions backwards, we encode which stack each index is currently in
        let mut stacks: [u8; 16] = std::array::from_fn(|i| i as u8);
        // offsets encodes where this same value is in the stack from the top (0 is the top)
        let mut offsets = [0; 16];

        // walk backwards
        for inst in self.instructions.drain(..).rev() {
            let should_move_mask: [u8; 16] =
                std::array::from_fn(|i| 0u8.wrapping_sub((inst.to == stacks[i] && offsets[i] < inst.count) as u8));

            offsets = std::array::from_fn(|i| {
                offsets[i]
                    .wrapping_add(inst.count * (stacks[i] == inst.from) as u8)
                    .wrapping_sub(inst.count * (stacks[i] == inst.to) as u8)
            });

            if reverse {
                offsets = std::array::from_fn(|i| offsets[i] ^ should_move_mask[i]);
            } else {
                offsets = std::array::from_fn(|i| offsets[i].wrapping_add(inst.count & should_move_mask[i]));
            }

            stacks = std::array::from_fn(|i| {
                let mask = should_move_mask[i];
                (mask & inst.from) | (!mask & stacks[i])
            });
        }
        (stacks, offsets)
    }

    fn answer(self, (stacks, offsets): ([u8; 16], [u8; 16])) -> ArrayString<16> {
        let Self {
            data,
            data_index_offsets,
            stack_count,
            ..
        } = self;

        // produce string based on states resulting position
        let mut output = ArrayString::new();
        for i in 0..stack_count {
            let stack = stacks[i] as usize;
            let index = data_index_offsets[stack] + offsets[i] as usize;
            output.push(data.as_bytes()[index * stack_count * 4 + stack * 4 + 1] as char);
        }
        output
    }
}

impl Challenge for Day05 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = ArrayString<16>;
    fn part_one(mut self) -> Self::Output1 {
        let states = self.solve(true);
        self.answer(states)
    }

    type Output2 = ArrayString<16>;
    fn part_two(mut self) -> Self::Output2 {
        let states = self.solve(false);
        self.answer(states)
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
