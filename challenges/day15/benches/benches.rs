use aoc::{Challenge, Parser};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day15::Solution;

pub fn day15(c: &mut Criterion) {
    let mut group = c.benchmark_group(Solution::<4000000>::NAME);

    let input = include_str!("../input.txt");
    let challenge = Solution::<4000000>::parse(input).unwrap().1;

    group.bench_function("parse", |b| {
        b.iter(|| Solution::<4000000>::parse(black_box(input)))
    });
    group.bench_function("part1", |b| {
        b.iter_batched(
            || challenge.clone(),
            Challenge::part_one,
            BatchSize::SmallInput,
        )
    });
    group.bench_function("part2", |b| {
        b.iter_batched(
            || challenge.clone(),
            Challenge::part_two,
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, day15);
criterion_main!(benches);
