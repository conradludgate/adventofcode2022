use aoc::Parser;
use day05::Day05;

fn main() {
    let day = Day05::parse(include_str!("../input.txt")).unwrap().1;
    // aoc::check(day);
    aoc::run(day);
}
