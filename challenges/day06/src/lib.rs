#![feature(portable_simd)]
use std::simd::{u8x32, SimdUint};

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

impl Day06 {
    #[inline(always)]
    fn solve(self, n: usize) -> usize {
        let mut counter = u8x32::default();
        for i in 0..n {
            counter.as_mut_array()[self.0[i] as usize & 0x1f] += 1;
        }

        let mut i = n;
        loop {
            // a clever way to assert that all the counters are 0 or 1 (ie no duplicates)
            // i*i = 0 if i == 0, = 1 if i == 1, > i if i > 1.
            // This means that if all i are 0 or 1, the sum remains the same.
            // If there is an i > 1, the sum will increase.
            // We maintain the regular sum to be = n, so we test that the sum is still n
            // after squaring.
            //
            // Being aware of overflows: the most we can expect is 14 of a single value
            // 14*14 is 196 which does not overflow u8.
            let sum = (counter * counter).reduce_sum();
            if sum == n as u8 {
                return i;
            }

            counter.as_mut_array()[self.0[i - n] as usize & 0x1f] -= 1;
            counter.as_mut_array()[self.0[i] as usize & 0x1f] += 1;

            i += 1;
        }
    }
}

impl Challenge for Day06 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.solve(4)
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.solve(14)
    }
}

#[cfg(test)]
mod tests {
    use super::Day06;
    use aoc::{Challenge, Parser};

    #[test]
    fn part_one() {
        assert_eq!(Day06::parse("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap().1.part_one(), 5);
        assert_eq!(Day06::parse("nppdvjthqldpwncqszvftbrmjlhg").unwrap().1.part_one(), 6);
        assert_eq!(
            Day06::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap().1.part_one(),
            10
        );
        assert_eq!(
            Day06::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap().1.part_one(),
            11
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(Day06::parse("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap().1.part_two(), 23);
        assert_eq!(Day06::parse("nppdvjthqldpwncqszvftbrmjlhg").unwrap().1.part_two(), 23);
        assert_eq!(
            Day06::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap().1.part_two(),
            29
        );
        assert_eq!(
            Day06::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap().1.part_two(),
            26
        );
    }
}
