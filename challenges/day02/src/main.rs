use aoc::Parser;
use day02::Day02;

fn main() {
    let day = Day02::parse(include_str!("../input.txt")).unwrap().1;
    aoc::check(day);
    // aoc::run(day);
}
