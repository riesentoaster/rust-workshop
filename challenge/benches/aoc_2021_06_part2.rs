use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use challenge::aoc_2021_06_part2::{
    aoc_2021_06_part2, aoc_2021_06_part2_generate_str, aoc_2021_06_part2_naive,
    aoc_2021_06_part2_parse, aoc_2021_06_part2_parse_manual, aoc_2021_06_part2_slow_parse,
};

const VALID_LENGTHS: [usize; 6] = [1, 3, 10, 20, 50, 1000];

fn bench_aoc_2021_06_part2_impls(c: &mut Criterion) {
    for len in VALID_LENGTHS.iter() {
        let mut group = c.benchmark_group(format!("bench_aoc_2021_06_part2_impls — {}", len));
        let input = aoc_2021_06_part2_generate_str(*len);
        group.bench_with_input(BenchmarkId::new("naive", len), &input, |b, i| {
            b.iter(|| aoc_2021_06_part2_naive(i))
        });
        group.bench_with_input(BenchmarkId::new("lookup", len), &input, |b, i| {
            b.iter(|| aoc_2021_06_part2(i))
        });
        group.bench_with_input(
            BenchmarkId::new("lookup — slow parse", len),
            &input,
            |b, i| b.iter(|| aoc_2021_06_part2_slow_parse(i)),
        );
        group.finish();
    }
}

fn bench_aoc_2021_06_part2_parse(c: &mut Criterion) {
    for len in VALID_LENGTHS.iter() {
        let mut group = c.benchmark_group(format!("bench_aoc_2021_06_part2_parse — {}", len));
        let input = aoc_2021_06_part2_generate_str(*len);
        group.bench_with_input(BenchmarkId::new("default", len), &input, |b, i| {
            b.iter(|| aoc_2021_06_part2_parse(i))
        });
        group.bench_with_input(BenchmarkId::new("manual", len), &input, |b, i| {
            b.iter(|| aoc_2021_06_part2_parse_manual(i))
        });
        group.finish();
    }
}

criterion_group!(
    benches,
    bench_aoc_2021_06_part2_impls,
    bench_aoc_2021_06_part2_parse
);
criterion_main!(benches);
