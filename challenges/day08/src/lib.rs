use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Day08 {
    heights: &'static [u8],
    stride: usize,
}

impl ChallengeParser for Day08 {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let line = input.as_bytes().iter().position(|&b| b == b'\n').unwrap_or(input.len());
        let stride = line + 1;

        Ok((
            "",
            Self {
                heights: input.as_bytes(),
                stride,
            },
        ))
    }
}

impl Challenge for Day08 {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let width = self.stride - 1;
        let height = self.heights.len() / self.stride;

        let mut set = vec![0; self.heights.len()];

        // left-to-right
        for j in 1..height - 1 {
            let mut max = self.heights[j * self.stride];
            set[j * self.stride] = 1;
            for i in 1..width - 1 {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
            let mut max = self.heights[j * self.stride + self.stride - 2];
            set[j * self.stride + self.stride - 2] = 1;
            for i in (1..width - 1).rev() {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
        }

        // top-to-bottom
        for i in 1..width - 1 {
            let mut max = self.heights[i];
            set[i] = 1;
            for j in 1..height - 1 {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
            let mut max = self.heights[(height - 1) * self.stride + i];
            set[(height - 1) * self.stride + i] = 1;
            for j in (1..height - 1).rev() {
                let b = self.heights[j * self.stride + i];
                if b > max {
                    max = b;
                    set[j * self.stride + i] = 1;
                }
            }
        }

        set.into_iter().sum::<usize>() + 4 /* corners */
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        // let width = self.stride - 1;
        let height = self.heights.len() / self.stride;

        let mut set = vec![1; self.heights.len()];
        // let mut columns = vec![0; self.heights.len()];

        // for (i, b) in self.heights.iter().copied().enumerate() {
        //     if b != b'\n' {
        //         let x = i % self.stride;
        //         let y = i / self.stride;
        //         columns[y + x * (height + 1)] = b;
        //     }
        // }

        // left-to-right
        for v in b'0'..=b'9' {
            // last is the index of the most recent entry that is >= v
            let mut last = 0;
            let mut lastb = v;
            let mut i = 0;
            loop {
                let dist = i - last;
                if last >= self.heights.len() || i >= self.heights.len() {
                    unsafe { std::hint::unreachable_unchecked() }
                }

                let b = self.heights[i];
                if b == b'\n' {
                    // if the last tracked was v, we should update that value
                    if lastb == v {
                        set[last] *= dist - 1;
                    }
                    i += 1;
                    if i >= self.heights.len() {
                        break
                    }
                    last = i;
                    lastb = self.heights[i];
                    continue;
                }

                if b >= v {
                    // if this byte is v, we should track the runlength since last
                    if b == v {
                        set[i] *= dist;
                    }
                    // if the last tracked was v, we should update that value
                    if lastb == v {
                        set[last] *= dist;
                    }
                    last = i;
                    lastb = b;
                }
                i += 1;
            }
        }
        // for line in columns.chunks(height+1) {
        //     println!("{:?}", &line)
        // }
        // for line in set.chunks(self.stride) {
        //     println!("{:?}", &line)
        // }

        // for v in (b'0'..=b'9').rev() {
        //     // last is the index of the most recent entry that is >= v
        //     let mut last = 0;
        //     let mut last_set = 0;
        //     for (i, b) in columns.iter().copied().enumerate() {
        //         if b == 0 {
        //             // if the last tracked was v, we should update that value
        //             if columns[last] == v {
        //                 set[last_set] *= i - last - 1;
        //             }
        //             last = i + 1;
        //             continue;
        //         }

        //         if b >= v {
        //             let x = i % (height + 1);
        //             let y = i / (height + 1);
        //             let set_i = y + x * self.stride;

        //             // if this byte is v, we should track the runlength since last
        //             if b == v {
        //                 set[set_i] *= i - last;
        //             }
        //             // if the last tracked was v, we should update that value
        //             if columns[last] == v {
        //                 set[last_set] *= i - last;
        //             }
        //             last = i;
        //             last_set = set_i;
        //         }
        //     }
        // }
        // for line in set.chunks(self.stride) {
        //     println!("{:?}", &line)
        // }

        // top-to-bottom
        for v in b'0'..=b'9' {
            // last is the index of the most recent entry that is >= v
            let mut last = 0;
            let mut i = 0;
            loop {
                if i >= self.heights.len() {
                    // if the last tracked was v, we should update that value
                    if self.heights[last] == v {
                        set[last] *= (i - last) / self.stride - 1;
                    }
                    i %= self.heights.len();
                    i += 1;
                    last = i;
                }
                let b = self.heights[i];

                if b == b'\n' {
                    break;
                }

                if b >= v {
                    // if this byte is v, we should track the runlength since last
                    if b == v {
                        set[i] *= (i - last) / self.stride;
                    }
                    // if the last tracked was v, we should update that value
                    if self.heights[last] == v {
                        set[last] *= (i - last) / self.stride;
                    }
                    last = i;
                }

                i += self.stride;
            }
        }

        // for line in set.chunks(self.stride) {
        //     println!("{:?}", &line[..width])
        // }

        set.into_iter().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Day08;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn parse() {
        let output = Day08::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Day08::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 21);
    }

    #[test]
    fn part_two() {
        let output = Day08::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 8);
    }
}
