#![feature(get_many_mut)]

use std::{array, mem::ManuallyDrop};

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::{ArrayString, ArrayVec};
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
    stacks: usize,
    instructions: ArrayVec<Instruction, 512>,
}

impl ChallengeParser for Day05 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let line_length = input.find('\n').unwrap() + 1;
        let block_length = input.find("\n\n").unwrap() + 1;
        let stacks = line_length / 4;

        let (data, input) = input.split_at(block_length - line_length);
        let (_, input) = input.split_at(line_length + 1);

        let (input, instructions) = Instruction::parse.separated_list1(line_ending).parse(input)?;
        Ok((
            input,
            Self {
                data,
                stacks,
                instructions,
            },
        ))
    }
}

impl Day05 {
    #[inline(always)]
    fn solve(self, reverse: bool) -> ArrayString<9> {
        let Self {
            data,
            stacks,
            instructions,
        } = self;

        // manually drop - ArrayVec is bad at eliminating dropbounds so I will enforce it here :)
        let instructions = ManuallyDrop::new(instructions);

        // the value of state is (X, Y) where X is the stack this value is in
        // and Y is the height (offset) in the stack
        let mut states: [_; 9] = array::from_fn(|i| (i as u8, 128));
        // current 'length' of the stack - not accurate but is relatively correct
        let mut lengths = [129_u8; 9];

        // walk backwards
        for i in instructions.iter().rev().copied() {
            // for all of our states
            for state in &mut states {
                let offset = lengths[i.to as usize].wrapping_sub(state.1);
                // if the state is in the resulting stack and is count from the top
                if state.0 == i.to && offset <= i.count {
                    // calculate the new offset from the top of the new stack
                    let offset = if reverse { offset - 1 } else { i.count - offset };
                    let height = lengths[i.from as usize].wrapping_add(offset);
                    *state = (i.from, height);
                }
            }
            // adjust stack lengths
            lengths[i.from as usize] = lengths[i.from as usize].wrapping_add(i.count);
            lengths[i.to as usize] = lengths[i.to as usize].wrapping_sub(i.count);
        }

        // calculate lengths of each stack based on the data
        let mut data_index_offsets = [0; 9];
        for line in data.as_bytes().chunks(stacks * 4) {
            for (stack, length) in data_index_offsets.iter_mut().take(stacks).enumerate() {
                if line[stack * 4 + 1] == b' ' {
                    *length += 1;
                }
            }
        }

        // produce string based on states resulting position
        let mut output = ArrayString::new();
        for state in states.into_iter().take(stacks) {
            let stack = state.0 as usize;
            let offset = lengths[stack] - state.1;
            let index = data_index_offsets[stack] + offset as usize - 1;
            output.push(data.as_bytes()[index * stacks * 4 + stack * 4 + 1] as char);
        }
        output
    }
}

impl Challenge for Day05 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = ArrayString<9>;
    fn part_one(self) -> Self::Output1 {
        self.solve(true)
    }

    type Output2 = ArrayString<9>;
    fn part_two(self) -> Self::Output2 {
        self.solve(false)
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
