use std::{cmp, fmt};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Entry {
    /// entries up to this index ahead is a list
    List(u16),
    /// a raw value
    Value(u8),
}

impl fmt::Debug for EntrySlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        let mut slice = self.0;

        loop {
            match slice {
                [Entry::Value(e), rest @ ..] => {
                    list.entry(e);
                    slice = rest
                }
                [Entry::List(o), ..] => {
                    let (entries, rest) = slice.split_at(*o as usize);
                    list.entry(&EntrySlice(&entries[1..]));
                    slice = rest
                }
                [] => return list.finish(),
            }
        }
    }
}

impl Entry {
    fn parse(arena: &mut Vec<Entry>, input: &'static [u8]) -> &'static [u8] {
        let (mut first, mut input) = input.split_first().unwrap();
        if *first == b'[' {
            let prefix_index = arena.len();
            arena.push(Entry::List(0));

            // skip empty lists
            if *input.first().unwrap() != b']' {
                loop {
                    let i = Entry::parse(arena, input);

                    // check for `,` or `]`
                    (first, input) = i.split_first().unwrap();
                    if *first == b']' {
                        break;
                    }
                }
            } else {
                (_, input) = input.split_first().unwrap();
            }

            let offset = (arena.len() - prefix_index) as u16;
            arena[prefix_index] = Entry::List(offset);
            input
        } else {
            let mut n = *first - b'0';
            while let Some(b'0'..=b'9') = input.first() {
                (first, input) = input.split_first().unwrap();
                n *= 10;
                n += *first - b'0';
            }
            arena.push(Entry::Value(n));
            input
        }
    }
}

struct EntrySlice<'a>(&'a [Entry]);

impl cmp::PartialEq for EntrySlice<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}
impl cmp::Eq for EntrySlice<'_> {}

impl cmp::PartialOrd for EntrySlice<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for EntrySlice<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.0, other.0) {
            // compare single values
            ([Entry::Value(e1), rest1 @ ..], [Entry::Value(e2), rest2 @ ..]) => e1
                .cmp(e2)
                .then_with(|| EntrySlice::cmp(&EntrySlice(rest1), &EntrySlice(rest2))),

            // compare list against list
            (l1 @ [Entry::List(o1), ..], l2 @ [Entry::List(o2), ..]) => {
                let (l1, rest1) = l1.split_at(*o1 as usize);
                let (l2, rest2) = l2.split_at(*o2 as usize);
                EntrySlice::cmp(&EntrySlice(&l1[1..]), &EntrySlice(&l2[1..]))
                    .then_with(|| EntrySlice::cmp(&EntrySlice(rest1), &EntrySlice(rest2)))
            }

            // compare list with value
            (l1 @ [Entry::List(i), ..], [e2 @ Entry::Value(_), rest2 @ ..]) => {
                let (l1, rest1) = l1.split_at(*i as usize);
                let l2 = &[Entry::List(2), *e2]; // construct value as list
                EntrySlice::cmp(&EntrySlice(l1), &EntrySlice(l2))
                    .then_with(|| EntrySlice::cmp(&EntrySlice(rest1), &EntrySlice(rest2)))
            }

            // compare value with list
            ([e1 @ Entry::Value(_), rest1 @ ..], l2 @ [Entry::List(i), ..]) => {
                let l1 = &[Entry::List(2), *e1]; // construct value as list
                let (l2, rest2) = l2.split_at(*i as usize);
                EntrySlice::cmp(&EntrySlice(l1), &EntrySlice(l2))
                    .then_with(|| EntrySlice::cmp(&EntrySlice(rest1), &EntrySlice(rest2)))
            }

            // edge cases
            ([], []) => cmp::Ordering::Equal,
            ([], [..]) => cmp::Ordering::Less,
            ([..], []) => cmp::Ordering::Greater,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut sum = 0;

        let mut arena = Vec::new();
        let mut ranges = Vec::new();

        let mut input = input.as_bytes();

        for i in 1.. {
            if input.len() < 2 {
                break;
            }
            if i > 1 {
                input = &input[2..]; // trim newlines
            }

            let left = arena.len();
            input = Entry::parse(&mut arena, input);

            input = &input[1..]; // trim newline

            let right = arena.len();
            input = Entry::parse(&mut arena, input);

            // determine ranges in arena
            let left = left + 1..right;
            let right = right + 1..arena.len();

            // save the ranges
            ranges.push(left.clone());
            ranges.push(right.clone());

            // construct our entryslice helpers
            let left = EntrySlice(&arena[left]);
            let right = EntrySlice(&arena[right]);

            println!("{left:?} <=> {right:?}");

            if left < right {
                sum += i;
            }
        }

        let by_key = |x: &std::ops::Range<usize>| EntrySlice(&arena[x.clone()]);
        ranges.sort_unstable_by_key(by_key);

        // we remove the list head from the original input lines, so this matches
        let two = [Entry::List(2), Entry::Value(2)];
        let six = [Entry::List(2), Entry::Value(6)];

        let (Ok(x) | Err(x)) = ranges.binary_search_by_key(&EntrySlice(&two), by_key);
        let (Ok(y) | Err(y)) = ranges.binary_search_by_key(&EntrySlice(&six), by_key);

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
