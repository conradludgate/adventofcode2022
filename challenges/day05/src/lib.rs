#![feature(get_many_mut)]

use std::mem::ManuallyDrop;

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::{ArrayString, ArrayVec};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::anychar, streaming::line_ending},
    sequence::tuple,
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Crate(u8);

impl Crate {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((
            tag("   ").map(|_| Crate(0)),
            tuple((tag("["), anychar, tag("]"))).map(|(_, c, _)| Crate(c as u8)),
        ))
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Stack(ArrayVec<Crate, 56>);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Instruction {
    count: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        tuple((tag("move "), number, tag(" from "), number, tag(" to "), number))
            .map(|(_, count, _, from, _, to)| Instruction { count, from, to })
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day05 {
    stacks: ArrayVec<Stack, 9>,
    instructions: ArrayVec<Instruction, 512>,
}

impl ChallengeParser for Day05 {
    fn parse(mut input: &'static str) -> IResult<&'static str, Self> {
        let lines: ArrayVec<ArrayVec<Crate, 9>, 8>;
        (input, lines) = Crate::parse
            .separated_list1(tag(" "))
            .terminate_list1(line_ending)
            .parse(input)?;

        let mut stacks = ArrayVec::<Stack, 9>::new();
        for (i, line) in lines.into_iter().enumerate() {
            for (j, krate) in line.into_iter().enumerate() {
                if i == 0 {
                    let stack = ArrayVec::new();
                    stacks.push(Stack(stack));
                }
                stacks[j].0.insert(0, krate);
            }
        }

        for stack in &mut stacks {
            while stack.0.last() == Some(&Crate(0)) {
                stack.0.pop();
            }
        }

        // skip empty lines
        input = &input[4 * stacks.len() + 1..];

        let (input, instructions) = Instruction::parse.separated_list1(line_ending).parse(input)?;
        Ok((input, Self { stacks, instructions }))
    }
}

impl Day05 {
    #[inline(always)]
    fn solve(self, reverse: bool) -> ArrayString<9> {
        let Self { stacks, instructions } = self;
        let mut stacks = ManuallyDrop::new(stacks);
        let instructions = ManuallyDrop::new(instructions);

        for i in &*instructions {
            // let (Stack(from), Stack(to)) = slice_dual_mut(&mut stacks, i.from as usize - 1, i.to as usize - 1);
            let [Stack(from), Stack(to)] = stacks.get_many_mut([i.from as usize - 1, i.to as usize - 1]).unwrap();

            if reverse {
                for _ in 0..i.count {
                    to.push(from.pop().unwrap())
                }
            } else {
                let partition = from.len() - i.count as usize;
                to.extend(from.drain(partition..));
            }
        }

        let mut output = ArrayString::new();
        for Stack(stack) in &*stacks {
            output.push(stack.last().unwrap().0 as char);
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
