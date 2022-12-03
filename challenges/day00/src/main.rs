use aoc::Parser;
use day00::Day00;

fn main() {
    let day = Day00::parse(include_str!("../input.txt")).unwrap().1;
    // aoc::check(day);
    aoc::run(day);
}
