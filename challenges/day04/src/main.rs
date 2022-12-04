use aoc::Parser;
use day04::Day04;

fn main() {
    let day = Day04::parse(include_str!("../input.txt")).unwrap().1;
    // aoc::check(day);
    aoc::run(day);
}
