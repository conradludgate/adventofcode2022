#![feature(portable_simd)]

use std::simd::{u32x8, u8x8, SimdPartialOrd, SimdUint};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<u32>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        // this is the stack of the current nested directory sizes
        let mut stack = [0; 16];
        let mut stack_len = 0;

        // this is a store of all final directory sizes
        let mut tree = Vec::<u32>::with_capacity(166);

        // size of the current directory
        let mut current_size = 0;

        for line in input.as_bytes().split(|&b| b == b'\n') {
            if line.len() < 4 {
                continue;
            }
            const CD: u32 = u32::from_ne_bytes(*b"$ cd");
            const LS: u32 = u32::from_ne_bytes(*b"$ ls");
            const DIR: u32 = u32::from_ne_bytes(*b"dir ");
            let prefix = u32::from_ne_bytes(<[u8; 4]>::try_from(&line[..4]).unwrap());
            match prefix {
                // on `cd ..`, push the final size to the tree
                // and update the current_size value with the previously saved value
                CD => {
                    if line.get(5).copied() == Some(b'.') {
                        stack_len -= 1;
                        let size = stack[stack_len & 0xf];
                        tree.push(current_size);
                        current_size += size;
                    } else {
                        stack[stack_len & 0xf] = current_size;
                        stack_len += 1;
                        current_size = 0;
                    }
                }
                // irrelevant to the algorithm
                LS | DIR => {}
                // record file size
                _ => {
                    let mut number = u8x8::default();
                    let len = line.len().clamp(0, 8);
                    number.as_mut_array()[..len].copy_from_slice(&line[..len]);

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
        while let Some(s) = stack_len.checked_sub(1) {
            tree.push(current_size);
            current_size += stack[s & 0xf];
            stack_len = s;
        }

        Ok(("", Self(tree)))
    }
}

impl Challenge for Solution {
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
    use super::Solution;
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
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 95437);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 24933642);
    }
}
