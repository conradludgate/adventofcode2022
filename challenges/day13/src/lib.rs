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

impl EntrySlice {
    fn cmp_lt(a: &u8, b: &u8) -> cmp::Ordering {
        if a < b {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }

    // Compare EntrySlice with Entry::Value(other)
    fn cmp_value(&self, other: &u8) -> cmp::Ordering {
        let mut slice = &self.0;
        loop {
            break match slice {
                [] => cmp::Ordering::Less,
                [Entry::Value(e)] => e.cmp(other),
                [Entry::Value(e), ..] => Self::cmp_lt(e, other),
                [Entry::List(i), ..] => {
                    slice = &slice[1..(*i as usize) + 1];
                    continue;
                }
            };
        }
    }

    // Equivalent to (self.cmp_value(2), self.cmp_value(6)), but maybe faster
    fn cmp_value26(&self) -> (cmp::Ordering, cmp::Ordering) {
        let mut slice = &self.0;
        loop {
            break match slice {
                [] => (cmp::Ordering::Less, cmp::Ordering::Less),
                [Entry::Value(e)] => (e.cmp(&2), e.cmp(&6)),
                [Entry::Value(e), ..] => (Self::cmp_lt(e, &2), Self::cmp_lt(e, &6)),
                [Entry::List(i), ..] => {
                    slice = &slice[1..(*i as usize) + 1];
                    continue;
                }
            };
        }
    }
}

impl cmp::Ord for EntrySlice {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let mut list1;
        let mut list2;
        let mut head1;
        let mut head2;
        let mut tail1 = &self.0;
        let mut tail2 = &other.0;

        loop {
            ((head1, tail1), (head2, tail2)) = match (tail1.split_first(), tail2.split_first()) {
                (None, None) => return cmp::Ordering::Equal,
                (None, Some(_)) => return cmp::Ordering::Less,
                (Some(_), None) => return cmp::Ordering::Greater,
                (Some(l), Some(r)) => (l, r),
            };

            // match the heads of the list. If the head is itself a list, slice up accordingly
            let cmp = match (head1, head2) {
                (Entry::Value(a), Entry::Value(b)) => a.cmp(b),
                (Entry::List(o1), Entry::List(o2)) => {
                    (list1, tail1) = tail1.split_at(*o1 as usize);
                    (list2, tail2) = tail2.split_at(*o2 as usize);
                    EntrySlice::cmp(EntrySlice::wrap_ref(list1), EntrySlice::wrap_ref(list2))
                }
                (Entry::List(o1), Entry::Value(e2)) => {
                    (list1, tail1) = tail1.split_at(*o1 as usize);
                    EntrySlice::wrap_ref(list1).cmp_value(e2)
                }
                (Entry::Value(e1), Entry::List(o2)) => {
                    (list2, tail2) = tail2.split_at(*o2 as usize);
                    EntrySlice::wrap_ref(list2).cmp_value(e1).reverse()
                }
            };

            if cmp.is_ne() {
                break cmp;
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, usize);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut sum = 0;
        let mut x = 1;
        let mut y = 2;

        let mut arena = Vec::with_capacity(512);

        let mut input = input.as_bytes();

        // let two = EntrySlice::wrap_ref(&[Entry::Value(2)]);
        // let six = EntrySlice::wrap_ref(&[Entry::Value(6)]);

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

            // construct our entryslice helpers
            let left = EntrySlice::wrap_ref(&arena[left]);
            let right = EntrySlice::wrap_ref(&arena[right]);

            if left < right {
                sum += i;
            }

            let (right2, right6) = right.cmp_value26();
            let (left2, left6) = left.cmp_value26();

            x += (left2.is_lt() as usize) + (right2.is_lt() as usize);
            y += (left6.is_lt() as usize) + (right6.is_lt() as usize);

            arena.clear();
        }

        Ok(("", Self(sum, x * y)))
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
