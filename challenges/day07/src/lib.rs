use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    sequence::tuple,
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Meta {
    Dir,
    Size(usize),
}

impl Meta {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((tag("dir").map(|_| Self::Dir), number.map(Self::Size))).parse(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Entry {
    meta: Meta,
    name: &'static str,
}

impl Entry {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        tuple((Meta::parse, tag(" "), take_until("\n")))
            .map(|(meta, _, name)| Self { meta, name })
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Command {
    Ls,
    Cd(&'static str),
}

impl Command {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((
            tag("ls").map(|_| Self::Ls),
            take_until("\n").preceded_by(tag("cd ")).map(Self::Cd),
        ))
        .preceded_by(tag("$ "))
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Line {
    Command(Command),
    Entry(Entry),
}

impl Line {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((Command::parse.map(Self::Command), Entry::parse.map(Self::Entry))).parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Day07(Vec<Line>);

impl ChallengeParser for Day07 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Line::parse.lines().map(Self).parse(input)
    }
}

impl Challenge for Day07 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        todo!()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day07;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse() {
        let output = Day07::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day07::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 0);
    }

    #[test]
    fn part_two() {
        let output = Day07::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
