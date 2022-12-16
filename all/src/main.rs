use std::time::{Duration, Instant};

use aoc::Parser;

static DAY01: &str = include_str!("../../challenges/day01/input.txt");
static DAY02: &str = include_str!("../../challenges/day02/input.txt");
static DAY03: &str = include_str!("../../challenges/day03/input.txt");
static DAY04: &str = include_str!("../../challenges/day04/input.txt");
static DAY05: &str = include_str!("../../challenges/day05/input.txt");
static DAY06: &str = include_str!("../../challenges/day06/input.txt");
static DAY07: &str = include_str!("../../challenges/day07/input.txt");
static DAY08: &str = include_str!("../../challenges/day08/input.txt");
static DAY09: &str = include_str!("../../challenges/day09/input.txt");
static DAY10: &str = include_str!("../../challenges/day10/input.txt");
static DAY11: &str = include_str!("../../challenges/day11/input.txt");
static DAY12: &str = include_str!("../../challenges/day12/input.txt");
static DAY13: &str = include_str!("../../challenges/day13/input.txt");
static DAY14: &str = include_str!("../../challenges/day14/input.txt");
static DAY15: &str = include_str!("../../challenges/day15/input.txt");
static DAY16: &str = include_str!("../../challenges/day16/input.txt");
// static DAY17: &str = include_str!("../../challenges/day17/input.txt");
// static DAY18: &str = include_str!("../../challenges/day18/input.txt");
// static DAY19: &str = include_str!("../../challenges/day19/input.txt");
// static DAY20: &str = include_str!("../../challenges/day20/input.txt");
// static DAY21: &str = include_str!("../../challenges/day21/input.txt");
// static DAY22: &str = include_str!("../../challenges/day22/input.txt");
// static DAY23: &str = include_str!("../../challenges/day23/input.txt");
// static DAY24: &str = include_str!("../../challenges/day24/input.txt");
// static DAY25: &str = include_str!("../../challenges/day25/input.txt");

fn main() {
    let start = Instant::now();
    let mut results = Vec::with_capacity(25);
    results.push(check::<day01::Solution>(DAY01));
    results.push(check::<day02::Solution>(DAY02));
    results.push(check::<day03::Solution>(DAY03));
    results.push(check::<day04::Solution>(DAY04));
    results.push(check::<day05::Solution>(DAY05));
    results.push(check::<day06::Solution>(DAY06));
    results.push(check::<day07::Solution>(DAY07));
    results.push(check::<day08::Solution>(DAY08));
    results.push(check::<day09::Solution>(DAY09));
    results.push(check::<day10::Solution>(DAY10));
    results.push(check::<day11::Solution>(DAY11));
    results.push(check::<day12::Solution>(DAY12));
    results.push(check::<day13::Solution>(DAY13));
    results.push(check::<day14::Solution>(DAY14));
    results.push(check::<day15::Solution<4000000>>(DAY15));
    results.push(check::<day16::Solution>(DAY16));
    // results.push(check::<day17::Solution>(DAY17));
    // results.push(check::<day18::Solution>(DAY18));
    // results.push(check::<day19::Solution>(DAY19));
    // results.push(check::<day20::Solution>(DAY20));
    // results.push(check::<day21::Solution>(DAY21));
    // results.push(check::<day22::Solution>(DAY22));
    // results.push(check::<day23::Solution>(DAY23));
    // results.push(check::<day24::Solution>(DAY24));
    // results.push(check::<day25::Solution>(DAY25));

    println!("Running {} days took {:?}", results.len(), start.elapsed());
    println!("{results:#?}");
}

fn check<C: Parser + Clone>(input: &'static str) -> (String, String, Duration) {
    let start = Instant::now();
    let challenge = C::parse(input).unwrap().1;
    let p1 = challenge.clone().part_one();
    let p2 = challenge.part_two();
    let took = start.elapsed();

    (p1.to_string(), p2.to_string(), took)
}
