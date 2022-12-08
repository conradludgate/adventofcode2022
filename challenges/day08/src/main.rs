use aoc::Parser;
use day08::Day08;

fn main() {
    let day = Day08::parse(include_str!("../input.txt")).unwrap().1;
    aoc::check(day);
    // aoc::run(day);
}
