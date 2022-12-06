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
        let mut counter = [0u8; 32];
        for i in 0..14 {
            counter[(self.0[i] - b'a') as usize] += 1;
        }

        let mut i = 14;
        loop {
            let counter2: [u8; 10] = std::array::from_fn(|i| counter[i] * counter[i]);
            let sum = counter2.into_iter().sum::<u8>();
            if sum == 10 {
                return i;
            }

            counter[(self.0[i - 14] - b'a') as usize] -= 1;
            counter[(self.0[i] - b'a') as usize] += 1;

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day06;
    use aoc::Challenge;

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
