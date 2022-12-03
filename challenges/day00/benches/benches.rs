use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day00::Day00;

pub fn day00(c: &mut Criterion) {
    let mut group = c.benchmark_group(Day00::NAME);

    let input = include_str!("../input.txt");
    let challenge = Day00::parse(input).unwrap().1;

    group.bench_function("parse", |b| b.iter(|| Day00::parse(black_box(input))));
    group.bench_function("part1", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_one, BatchSize::SmallInput)
    });
    group.bench_function("part2", |b| {
        b.iter_batched(|| challenge.clone(), Challenge::part_two, BatchSize::SmallInput)
    });

    group.finish();
}

criterion_group!(benches, day00);
criterion_main!(benches);
