use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day08::Day08;

pub fn day08(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day08::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day08::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day08::parse(black_box(input))));
    group.bench_function("part1", |b| b.iter(|| challenge.part_one()));
    group.bench_function("part2", |b| b.iter(|| challenge.part_two()));

    group.finish();
}

criterion_group!(benches, day08);
criterion_main!(benches);
