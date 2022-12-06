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
        if self.0.len() < n {
            return 0;
        }

        let mut counter = [0u8; 32];
        for i in 0..n {
            unsafe {
                *counter.get_unchecked_mut(self.0[i] as usize & 0b0001_1111) += 1;
            }
        }

        let mut i = n;
        loop {
            let counter2: [u8; 32] = std::array::from_fn(|i| counter[i] * counter[i]);
            let sum = counter2.into_iter().sum::<u8>();
            if sum == n as u8 {
                return i;
            }

            unsafe {
                *counter.get_unchecked_mut(*self.0.get_unchecked(i - n) as usize & 0b0001_1111) -= 1;
            }
            unsafe {
                *counter.get_unchecked_mut(*self.0.get_unchecked(i) as usize & 0b0001_1111) += 1;
            }

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
