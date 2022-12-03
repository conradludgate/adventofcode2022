use aoc::Parser;
use day03::Day03;

fn main() {
    let day = Day03::parse(include_str!("../input.txt")).unwrap().1;
    aoc::check(day);
    // aoc::run(day);
}
