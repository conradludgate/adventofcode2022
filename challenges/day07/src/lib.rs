use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
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

impl Day07 {
    fn build_dir_tree(self) -> Vec<usize> {
        // this is the stack of the current nested directory sizes
        let mut stack = ArrayVec::<usize, 10>::new();

        // this is a store of all final directory sizes
        let mut tree = Vec::<usize>::with_capacity(166);

        // size of the current directory
        let mut current_size = 0;

        for line in self.0 {
            match line {
                // on `cd ..`, push the final size to the tree
                // and update the current_size value with the previously saved value
                Line::Command(Command::Cd("..")) => {
                    let size = stack.pop().unwrap();
                    tree.push(current_size);
                    current_size += size;
                }
                // on `cd foo`, save the current dir size in the stack
                // and reset it to 0 for the sub directory
                Line::Command(Command::Cd(_)) => {
                    stack.push(current_size);
                    current_size = 0;
                }
                // irrelevant to the algorithm
                Line::Command(Command::Ls) | Line::Entry(Entry { meta: Meta::Dir, .. }) => {}
                // record file size
                Line::Entry(Entry {
                    meta: Meta::Size(x), ..
                }) => current_size += x,
            }
        }

        // final `cd ..`s
        while let Some(size) = stack.pop() {
            tree.push(current_size);
            current_size += size;
        }

        tree
    }
}

impl Challenge for Day07 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.build_dir_tree().into_iter().filter(|x| *x <= 100000).sum()
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let sizes = self.build_dir_tree();

        let minimum = sizes.last().unwrap() - 40_000_000;
        sizes.into_iter().filter(|&v| v > minimum).min().unwrap()
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
7214296 k
";

    #[test]
    fn parse() {
        let output = Day07::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day07::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 95437);
    }

    #[test]
    fn part_two() {
        let output = Day07::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 24933642);
    }
}
