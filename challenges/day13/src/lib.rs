use std::{cmp, fmt, ops::Range};

use aoc::{Challenge, Parser as ChallengeParser};
use bytemuck::TransparentWrapper;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Entry {
    /// entries up to this index ahead is a list
    List(u8),
    /// a raw value
    Value(u8),
}

impl Entry {
    fn parse(arena: &mut Vec<Entry>, input: &'static [u8]) -> &'static [u8] {
        let (mut first, mut input) = input.split_first().unwrap();
        if *first == b'[' {
            let prefix_index = arena.len();
            let range;
            arena.push(Entry::List(0));
            (input, range) = Entry::parse_list(arena, input);
            arena[prefix_index] = Entry::List(range.len() as u8);
        } else {
            let mut n = *first - b'0';
            while let Some(b'0'..=b'9') = input.first() {
                (first, input) = input.split_first().unwrap();
                n *= 10;
                n += *first - b'0';
            }
            arena.push(Entry::Value(n));
        }
        input
    }

    fn parse_list(
        arena: &mut Vec<Entry>,
        mut input: &'static [u8],
    ) -> (&'static [u8], Range<usize>) {
        let start = arena.len();
        // skip empty lists
        if *input.first().unwrap() != b']' {
            loop {
                let i = Entry::parse(arena, input);

                // check for `,` or `]`
                let next;
                (next, input) = i.split_first().unwrap();
                if *next == b']' {
                    break;
                }
            }
        } else {
            (_, input) = input.split_first().unwrap();
        }

        (input, start..arena.len())
    }
}

#[derive(TransparentWrapper)]
#[repr(transparent)]
struct EntrySlice([Entry]);

impl fmt::Debug for EntrySlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        let mut slice = &self.0;

        loop {
            match slice {
                [Entry::Value(e), rest @ ..] => {
                    list.entry(e);
                    slice = rest
                }
                [Entry::List(o), rest @ ..] => {
                    let (entries, rest) = rest.split_at(*o as usize);
                    list.entry(&EntrySlice::wrap_ref(entries));
                    slice = rest
                }
                [] => return list.finish(),
            }
        }
    }
}

impl cmp::PartialEq for EntrySlice {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}
impl cmp::Eq for EntrySlice {}

impl cmp::PartialOrd for EntrySlice {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for EntrySlice {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let ((head1, mut tail1), (head2, mut tail2)) =
            match (self.0.split_first(), other.0.split_first()) {
                (None, None) => return cmp::Ordering::Equal,
                (None, Some(_)) => return cmp::Ordering::Less,
                (Some(_), None) => return cmp::Ordering::Greater,
                (Some(l), Some(r)) => (l, r),
            };

        let list1;
        let list2;

        // match the heads of the list. If the head is itself a list, slice up accordingly
        let cmp = match (head1, head2) {
            (Entry::Value(a), Entry::Value(b)) => a.cmp(b),
            (Entry::List(o1), Entry::List(o2)) => {
                (list1, tail1) = tail1.split_at(*o1 as usize);
                (list2, tail2) = tail2.split_at(*o2 as usize);
                EntrySlice::cmp(EntrySlice::wrap_ref(list1), EntrySlice::wrap_ref(list2))
            }
            (Entry::List(o1), e2 @ Entry::Value(_)) => {
                (list1, tail1) = tail1.split_at(*o1 as usize);
                let list2 = &[*e2];
                EntrySlice::cmp(EntrySlice::wrap_ref(list1), EntrySlice::wrap_ref(list2))
            }
            (e1 @ Entry::Value(_), Entry::List(o2)) => {
                let list1 = &[*e1];
                (list2, tail2) = tail2.split_at(*o2 as usize);
                EntrySlice::cmp(EntrySlice::wrap_ref(list1), EntrySlice::wrap_ref(list2))
            }
        };

        // finally, continue comparing the tails
        cmp.then_with(|| Self::cmp(Self::wrap_ref(tail1), Self::wrap_ref(tail2)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut sum = 0;

        let mut arena = Vec::with_capacity(input.len());
        let mut ranges = Vec::with_capacity(input.len() / 50);

        let mut input = input.as_bytes();

        for i in 1.. {
            if input.is_empty() {
                break;
            }
            if i > 1 {
                input = &input[1..]; // trim newline
            }
            input = &input[1..]; // trim `[`

            let left;
            let right;

            (input, left) = Entry::parse_list(&mut arena, input);
            input = &input[2..]; // trim `\n[`
            (input, right) = Entry::parse_list(&mut arena, input);
            input = &input[1..]; // trim `\n`

            // save the ranges
            ranges.push(left.clone());
            ranges.push(right.clone());

            // construct our entryslice helpers
            let left = EntrySlice::wrap_ref(&arena[left]);
            let right = EntrySlice::wrap_ref(&arena[right]);

            // println!("bar {left:?} <=> {right:?}");

            if left < right {
                sum += i;
            }
        }

        let by_key = |x: &std::ops::Range<usize>| EntrySlice::wrap_ref(&arena[x.clone()]);
        ranges.sort_unstable_by_key(by_key);

        let two = [Entry::Value(2)];
        let six = [Entry::Value(6)];

        let (Ok(x) | Err(x)) = ranges.binary_search_by_key(&EntrySlice::wrap_ref(&two), by_key);
        let (Ok(y) | Err(y)) = ranges.binary_search_by_key(&EntrySlice::wrap_ref(&six), by_key);

        Ok(("", Self(sum, (x + 1) * (y + 2))))
        // Ok(("", Self(0, 0)))
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
