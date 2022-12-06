use aoc::Parser;
use day06::Day06;

fn main() {
    let day = Day06::parse(include_str!("../input.txt")).unwrap().1;
    // aoc::check(day);
    aoc::run(day);
}
