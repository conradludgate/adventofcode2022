#![feature(split_array)]

use aoc::{Challenge, Parser as ChallengeParser};
use fxhash::FxHashMap;
use nom::IResult;
use poly::Poly;

use crate::rational::Rational;

mod poly;
mod rational;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    human: u16,
    op: Op,
    lhs: Poly,
    rhs: Poly,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

enum Expr {
    Op(Op, [u8; 4], [u8; 4]),
    Val(u16),
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut monkeys = FxHashMap::with_capacity_and_hasher(input.len() / 8, Default::default());

        let mut input = input.as_bytes();
        while input.len() > 6 {
            // assumed: all monkey names are 4 chars
            let name;
            (name, input) = input.split_array_ref();
            let name = *name;

            let op = match input.get(7) {
                Some(b'*') => Some(Op::Mul),
                Some(b'/') => Some(Op::Div),
                Some(b'+') => Some(Op::Add),
                Some(b'-') => Some(Op::Sub),
                _ => None,
            };
            let res = match op {
                Some(op) => {
                    let chunk: &[u8; 14];
                    (chunk, input) = input.split_array_ref();
                    let [_, _, a, b, c, d, _, _, _, e, f, g, h, _] = *chunk;
                    let lhs = [a, b, c, d];
                    let rhs = [e, f, g, h];
                    Expr::Op(op, lhs, rhs)
                }
                None => {
                    // assumed: input numbers are at most 4 digits to fit in u16
                    let nl = input.iter().position(|b| *b == b'\n').unwrap();
                    let n;
                    (n, input) = input.split_at(nl + 1);
                    let mut v = 0;
                    for i in &n[2..nl] {
                        v *= 10;
                        v += (*i - b'0') as u16;
                    }
                    Expr::Val(v)
                }
            };
            monkeys.insert(name, res);
        }

        let Expr::Val(human) = monkeys[b"humn"] else { panic!("humn wasn't an integer value") };
        let Expr::Op(op, lhs, rhs) = monkeys[b"root"] else { panic!("root wasn't an operation") };

        let lhs = build_poly(lhs, &monkeys);
        let rhs = build_poly(rhs, &monkeys);

        Ok((
            "",
            Self {
                human,
                op,
                lhs,
                rhs,
            },
        ))
    }
}

fn build_poly(op: [u8; 4], equation: &FxHashMap<[u8; 4], Expr>) -> Poly {
    if op == *b"humn" {
        Poly::x()
    } else {
        match equation[&op] {
            Expr::Op(op, lhs, rhs) => {
                let lhs = build_poly(lhs, equation);
                let rhs = build_poly(rhs, equation);
                Poly::apply(op, lhs, rhs)
            }
            Expr::Val(x) => Poly::from(Rational::from(x)),
        }
    }
}

impl Poly {
    fn apply(op: Op, lhs: Self, rhs: Self) -> Self {
        match op {
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
        }
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = i64;
    fn part_one(self) -> Self::Output1 {
        Poly::apply(self.op, self.lhs, self.rhs)
            .eval(Rational::from(self.human))
            .val()
            .unwrap()
    }

    type Output2 = i64;
    fn part_two(self) -> Self::Output2 {
        Poly::apply(Op::Sub, self.lhs, self.rhs)
            .solve()
            .val()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 152);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 301);
    }
}
