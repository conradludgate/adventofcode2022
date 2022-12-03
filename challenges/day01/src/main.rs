use aoc::Parser;
use day01::Day01;

fn main() {
    let day = Day01::parse(include_str!("../input.txt")).unwrap().1;
    aoc::check(day);
    // aoc::run(day);
}
