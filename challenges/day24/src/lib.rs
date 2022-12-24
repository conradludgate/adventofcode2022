use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::prelude::astar;

const N: u8 = b'^';
const E: u8 = b'>';
const S: u8 = b'v';
const W: u8 = b'<';

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Input(&'static [u8], usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        // let mut state = Vec::with_capacity(input.len());

        let row_len = input.bytes().position(|b| b == b'\n').unwrap() + 1;
        let i = Input(input.as_bytes(), row_len);
        let column_len = input.len() / row_len;

        let x = i.solve(
            State {
                minute: 0,
                x: 1,
                y: 0,
            },
            (row_len - 3, column_len - 1),
        );
        let y = i.solve(
            State {
                minute: x,
                x: row_len - 3,
                y: column_len - 1,
            },
            (1, 0),
        );
        let z = i.solve(
            State {
                minute: x + y,
                x: 1,
                y: 0,
            },
            (row_len - 3, column_len - 1),
        );

        Ok(("", Self(x, x + y + z)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.0
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.1
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    minute: usize,
    x: usize,
    y: usize,
}

impl Input {
    fn solve(&self, start: State, goal: (usize, usize)) -> usize {
        let Self(state, row_len) = *self;
        let column_len = state.len() / row_len;

        astar(
            &start,
            |s| {
                let min = s.minute + 1;
                let xy = [
                    (Some(s.x - 1), Some(s.y)),      // left
                    (Some(s.x + 1), Some(s.y)),      // right
                    (Some(s.x), s.y.checked_sub(1)), // up
                    (Some(s.x), s.y.checked_add(1)), // down
                    (Some(s.x), Some(s.y)),          // wait
                ]
                .into_iter()
                .filter_map(|(x, y)| x.zip(y))
                .filter(move |&(x, y)| {
                    // boundary conditions
                    if x == 0 || x + 3 > row_len {
                        return false;
                    }
                    if y == 0 {
                        return x == 1;
                    }
                    if y + 1 == column_len {
                        return x + 3 == row_len;
                    }
                    if y >= column_len {
                        return false;
                    }

                    let rl = row_len - 3;
                    let cl = column_len - 2;

                    let east_index = (x - 1 + rl - (min % rl)) % rl;
                    let west_index = (x - 1 + min) % rl;

                    let south_index = (y - 1 + cl - (min % cl)) % cl;
                    let north_index = (y - 1 + min) % cl;

                    state[y * row_len + east_index + 1] != E
                        && state[y * row_len + west_index + 1] != W
                        && state[x + north_index * row_len + row_len] != N
                        && state[x + south_index * row_len + row_len] != S
                });

                let min = s.minute + 1;
                xy.map(move |(x, y)| (State { minute: min, x, y }, 1))
            },
            |s| (goal.0).abs_diff(s.x) + (goal.1).abs_diff(s.y),
            |s| s.x == goal.0 && s.y == goal.1,
        )
        .unwrap()
        .1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 18);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 54);
    }
}
