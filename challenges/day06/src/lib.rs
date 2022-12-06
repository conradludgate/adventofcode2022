#![feature(array_windows)]

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Day06(&'static [u8]);

impl ChallengeParser for Day06 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let data = &input.as_bytes()[..input.rfind('\n').unwrap_or(input.len())];
        Ok(("", Self(data)))
    }
}

impl Challenge for Day06 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        for (i, [a, b, c, d]) in self.0.array_windows().copied().enumerate() {
            if a == b || a == c || a == d || b == c || b == d || c == d {
                continue;
            }
            return i + 4;
        }
        0
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        'outer: for (i, mut window) in self.0.array_windows::<14>().copied().enumerate() {
            window.sort();
            for i in 1..14 {
                if window[i - 1] == window[i] {
                    continue 'outer;
                }
            }
            return i + 14;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day06;
    use aoc::{Challenge, Parser};

    #[test]
    fn part_one() {
        assert_eq!(Day06(b"bvwbjplbgvbhsrlpgdmjqwftvncz").part_one(), 5);
        assert_eq!(Day06(b"nppdvjthqldpwncqszvftbrmjlhg").part_one(), 6);
        assert_eq!(Day06(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").part_one(), 10);
        assert_eq!(Day06(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").part_one(), 11);
    }

    #[test]
    fn part_two() {
        assert_eq!(Day06(b"bvwbjplbgvbhsrlpgdmjqwftvncz").part_two(), 23);
        assert_eq!(Day06(b"nppdvjthqldpwncqszvftbrmjlhg").part_two(), 23);
        assert_eq!(Day06(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").part_two(), 29);
        assert_eq!(Day06(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").part_two(), 26);
    }
}
