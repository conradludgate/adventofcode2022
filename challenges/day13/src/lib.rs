use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Clone)]
enum Entry {
    List(Vec<Entry>),
    Value(u8),
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(list) => f.debug_list().entries(list).finish(),
            Self::Value(e) => e.fmt(f),
        }
    }
}

impl Entry {
    fn parse(input: &'static [u8]) -> (&'static [u8], Self) {
        let (mut first, mut input) = input.split_first().unwrap();
        if *first == b'[' {
            let mut list = Vec::new();

            // skip empty lists
            if *input.first().unwrap() != b']' {
                loop {
                    let (i, e) = Entry::parse(input);
                    list.push(e);

                    // check for `,` or `]`
                    (first, input) = i.split_first().unwrap();
                    if *first == b']' {
                        break;
                    }
                }
            } else {
                (_, input) = input.split_first().unwrap();
            }

            (input, Entry::List(list))
        } else {
            let mut n = *first - b'0';
            while input.first().unwrap().is_ascii_digit() {
                (first, input) = input.split_first().unwrap();
                n *= 10;
                n += *first - b'0';
            }
            (input, Entry::Value(n))
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entry::Value(e1), Entry::Value(e2)) => e1.eq(e2),
            (Entry::List(l1), Entry::List(l2)) => l1.eq(l2),
            (Entry::List(l1), Entry::Value(e2)) => l1.as_slice().eq(&[Entry::Value(*e2)]),
            (Entry::Value(e1), Entry::List(l2)) => [Entry::Value(*e1)].as_slice().eq(l2.as_slice()),
        }
    }
}
impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Entry::Value(e1), Entry::Value(e2)) => e1.cmp(e2),
            (Entry::List(l1), Entry::List(l2)) => l1.cmp(l2),
            (Entry::List(l1), Entry::Value(e2)) => l1.as_slice().cmp(&[Entry::Value(*e2)]),
            (Entry::Value(e1), Entry::List(l2)) => [Entry::Value(*e1)].as_slice().cmp(l2.as_slice()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut sum = 0;
        for (i, pair) in input.split("\n\n").enumerate() {
            let i = i + 1;

            let (left, right) = pair.split_once('\n').unwrap();


            let left1 = Entry::parse(left.as_bytes()).1;
            let right1 = Entry::parse(right.as_bytes()).1;

            println!("== Pair {i} ==: {left1:?} vs {right1:?}");

            match left1.cmp(&right1) {
                std::cmp::Ordering::Less => sum += i,
                std::cmp::Ordering::Equal => {
                    dbg!(i, left1, right1);
                }
                std::cmp::Ordering::Greater => {}
            }

            // dbg!(left1);
            // dbg!(right1);

            // let mut left = left.as_bytes();
            // let mut right = right.as_bytes();
            // let mut fake_list_stack = (0u16, 0u16);
            // sum += loop {
            //     let Some(l) = left.first().copied() else { break i };
            //     let Some(r) = right.first().copied() else { break 0 };

            //     if fake_list_stack.0 > 0 || fake_list_stack.1 > 0 {
            //         println!("==============> {fake_list_stack:?}");
            //         println!("{}", std::str::from_utf8(left).unwrap());
            //         println!("{}", std::str::from_utf8(right).unwrap());
            //     }

            //     if l != r {
            //         // left list drained first
            //         if l == b']' {
            //             if let Some(s) = fake_list_stack.1.checked_sub(1) {
            //                 fake_list_stack.1 = s;
            //                 left = left.split_first().unwrap().1;
            //                 continue;
            //             } else {
            //                 break i;
            //             }
            //         }
            //         // right list drained first
            //         if r == b']' {
            //             if let Some(s) = fake_list_stack.0.checked_sub(1) {
            //                 fake_list_stack.0 = s;
            //                 right = right.split_first().unwrap().1;
            //                 continue;
            //             } else {
            //                 break i;
            //             }
            //         }

            //         // If exactly one value is an integer, convert the integer to a list
            //         // which contains that integer as its only value, then retry the
            //         // comparison. For example, if comparing [0,0,0] and 2, convert the right
            //         // value to [2] (a list containing 2); the result is then found by
            //         // instead comparing [0,0,0] and [2].

            //         // possible cases:
            //         // []    vs [x] => we will next compare `]` to `x`
            //         //                 this will always exit
            //         // [y]   vs [x] => we will next compare `y` to `x`.
            //         //                 if y != x, we will exit
            //         //                 if y == x, we will compare `]` to `,` or `]`
            //         // [y,z] vs [x] => we will next compare `y` to `x`.
            //         //                 if y != x, we will exit
            //         //                 if y == x, we will compare `,` to `,` or `]`.

            //         if l == b'[' {
            //             fake_list_stack.1 += 1;
            //             // consume only the [
            //             left = left.split_first().unwrap().1;
            //             continue;
            //         }
            //         if r == b'[' {
            //             fake_list_stack.0 += 1;
            //             // consume only the [
            //             right = right.split_first().unwrap().1;
            //             continue;
            //         }

            //         if l < r {
            //             break i;
            //         } else {
            //             break 0;
            //         }
            //     }

            //     match (fake_list_stack.0.checked_sub(1), fake_list_stack.1.checked_sub(1)) {
            //         (Some(l), Some(r)) => fake_list_stack = (l, r),
            //         (Some(_), None) => break 0,
            //         (None, Some(_)) => break i,
            //         (None, None) => {
            //             left = left.split_first().unwrap().1;
            //             right = right.split_first().unwrap().1;
            //         }
            //     }
            // };
            // println!("sum = {sum}");
        }

        Ok(("", Self(sum)))
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
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "[[],[9,[[7,10,8],2,4],2],[3,5,3,10]]
[[],[1,6],[1,8]]
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 13);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
