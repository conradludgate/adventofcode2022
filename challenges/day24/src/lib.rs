use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::prelude::{astar, bfs, dijkstra};

const N: u8 = 1;
const E: u8 = 2;
const S: u8 = 4;
const W: u8 = 8;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<u8>, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut state = Vec::with_capacity(input.len());

        let line_len = input.bytes().position(|b| b == b'\n').unwrap() + 1;
        for chunk in input.as_bytes()[line_len..input.len() - line_len].chunks(line_len) {
            for v in &chunk[1..line_len - 2] {
                let v = match v {
                    b'^' => N,
                    b'>' => E,
                    b'v' => S,
                    b'<' => W,
                    _ => 0,
                };
                state.push(v);
            }
        }

        Ok(("", Self(state, line_len - 3)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let Self(ref state, row_len) = self;
        let column_len = state.len() / row_len;

        self.solve(
            State {
                minute: 0,
                x: 0,
                y: 0,
            },
            (row_len - 1, column_len + 1),
        )
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        let Self(ref state, row_len) = self;
        let column_len = state.len() / row_len;

        let x = self.solve(
            State {
                minute: 0,
                x: 0,
                y: 0,
            },
            (row_len - 1, column_len + 1),
        );
        let y = self.solve(
            State {
                minute: x,
                x: row_len - 1,
                y: column_len + 1,
            },
            (0, 0),
        );
        let z = self.solve(
            State {
                minute: x + y,
                x: 0,
                y: 0,
            },
            (row_len - 1, column_len + 1),
        );
        x + y + z
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    minute: usize,
    x: usize,
    y: usize,
}

impl Solution {
    fn solve(&self, start: State, goal: (usize, usize)) -> usize {
        let Self(ref state, row_len) = *self;
        let column_len = state.len() / row_len;

        astar(
            &start,
            |s| {
                let min = s.minute + 1;
                let state = state.as_slice();
                let xy = [
                    (s.x.checked_sub(1), Some(s.y)), // left
                    (s.x.checked_add(1), Some(s.y)), // right
                    (Some(s.x), s.y.checked_sub(1)), // up
                    (Some(s.x), s.y.checked_add(1)), // down
                    (Some(s.x), Some(s.y)),          // wait
                ]
                .into_iter()
                .filter_map(|(x, y)| x.zip(y))
                .filter(move |&(x, y)| {
                    if x >= row_len {
                        return false;
                    }
                    if y == 0 {
                        return x == 0;
                    }
                    let y = y - 1;
                    if y == column_len {
                        return x + 1 == row_len;
                    }
                    if y > column_len {
                        return false;
                    }

                    let east_index = (x + row_len - (min % row_len)) % row_len;
                    let west_index = (x + min) % row_len;

                    let south_index = (y + column_len - (min % column_len)) % column_len;
                    let north_index = (y + min) % column_len;

                    state[y * row_len + east_index] != E
                        && state[y * row_len + west_index] != W
                        && state[x + north_index * row_len] != N
                        && state[x + south_index * row_len] != S
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
