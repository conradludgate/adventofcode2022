use aoc::Parser;
use day07::Day07;

fn main() {
    let day = Day07::parse(include_str!("../input.txt")).unwrap().1;
    // aoc::check(day);
    aoc::run(day);
}
