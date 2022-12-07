#![feature(portable_simd)]

use std::simd::{u32x8, u8x8, SimdPartialEq, SimdPartialOrd, SimdUint};

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Day07(Vec<u32>);

impl ChallengeParser for Day07 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        // this is the stack of the current nested directory sizes
        let mut stack = ArrayVec::<u32, 10>::new();

        // this is a store of all final directory sizes
        let mut tree = Vec::<u32>::with_capacity(166);

        // size of the current directory
        let mut current_size = 0;

        for line in input.as_bytes().split(|&b| b == b'\n') {
            match line {
                // on `cd ..`, push the final size to the tree
                // and update the current_size value with the previously saved value
                b"$ cd .." => {
                    let size = stack.pop().unwrap();
                    tree.push(current_size);
                    current_size += size;
                }
                // on `cd foo`, save the current dir size in the stack
                // and reset it to 0 for the sub directory
                x if x.starts_with(b"$ cd ") => {
                    stack.push(current_size);
                    current_size = 0;
                }
                // irrelevant to the algorithm
                b"$ ls" | b"" => {}
                x if x.starts_with(b"dir ") => {}
                // record file size
                x => {
                    let mut number = u8x8::default();
                    let len = x.len().min(8);
                    number.as_mut_array()[..len].copy_from_slice(&x[..len]);

                    let number = (number - u8x8::splat(b'0')).cast();
                    let number_mask = number.simd_lt(u32x8::splat(10)).to_int().cast();

                    let pows =
                        u32x8::from_array([10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100, 10, 1]) & number_mask;

                    let pow = 100_000_000 - pows.reduce_sum() * 9;
                    let size = (pows * number).reduce_sum() / pow;

                    current_size += size;
                }
            }
        }

        // final `cd ..`s
        while let Some(size) = stack.pop() {
            tree.push(current_size);
            current_size += size;
        }

        Ok(("", Self(tree)))
    }
}

impl Challenge for Day07 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = u32;
    fn part_one(self) -> Self::Output1 {
        self.0.into_iter().filter(|x| *x <= 100000).sum()
    }

    type Output2 = u32;
    fn part_two(self) -> Self::Output2 {
        let minimum = self.0.last().unwrap() - 40_000_000;
        self.0.into_iter().filter(|&v| v > minimum).min().unwrap()
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
