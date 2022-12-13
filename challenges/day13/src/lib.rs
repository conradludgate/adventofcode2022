use std::{cmp, fmt};

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;
use typed_arena::Arena;

#[derive(Clone, Copy)]
enum Entry<'a> {
    List(&'a [Entry<'a>]),
    Value(u8),
}

impl fmt::Debug for Entry<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(list) => f.debug_list().entries(list.iter()).finish(),
            Self::Value(e) => e.fmt(f),
        }
    }
}

impl<'a> Entry<'a> {
    fn parse(arena: &'a Arena<Entry<'a>>, input: &'static [u8]) -> (&'static [u8], Self) {
        let (mut first, mut input) = input.split_first().unwrap();
        if *first == b'[' {
            let mut list = ArrayVec::<Entry, 16>::new();

            // skip empty lists
            if *input.first().unwrap() != b']' {
                loop {
                    let (i, e) = Entry::parse(arena, input);
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

            (input, Entry::List(arena.alloc_extend(list)))
        } else {
            let mut n = *first - b'0';
            while let Some(b'0'..=b'9') = input.first() {
                (first, input) = input.split_first().unwrap();
                n *= 10;
                n += *first - b'0';
            }
            (input, Entry::Value(n))
        }
    }
}

impl cmp::PartialEq for Entry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}
impl cmp::Eq for Entry<'_> {}

impl cmp::PartialOrd for Entry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Entry<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Entry::Value(e1), Entry::Value(e2)) => e1.cmp(e2),
            (Entry::List(l1), Entry::List(l2)) => l1.cmp(l2),
            (Entry::List(l1), Entry::Value(e2)) => (*l1).cmp(&[Entry::Value(*e2)]),
            (Entry::Value(e1), Entry::List(l2)) => [Entry::Value(*e1)].as_slice().cmp(l2),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut sum = 0;

        let arena = Arena::new();

        let two = Entry::parse(&arena, b"[[2]]").1;
        let six = Entry::parse(&arena, b"[[6]]").1;

        let mut lists = vec![two, six];

        for (i, pair) in input.split("\n\n").enumerate() {
            let i = i + 1;

            let (left, right) = pair.split_once('\n').unwrap();

            let left1 = Entry::parse(&arena, left.as_bytes()).1;
            let right1 = Entry::parse(&arena, right.as_bytes()).1;

            if left1 < right1 {
                sum += i;
                lists.push(left1);
                lists.push(right1);
            } else {
                lists.push(right1);
                lists.push(left1);
            }
        }

        lists.sort();

        let mut iter = lists.into_iter();
        let right = iter.rposition(|p| p == six).unwrap();
        let left = iter.position(|p| p == two).unwrap();

        Ok(("", Self(sum, (left + 1) * (right + 1))))
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

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
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
        assert_eq!(output.part_two(), 140);
    }
}
