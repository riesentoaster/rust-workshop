use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use challenge::{aoc_2021_06_part2, aoc_2021_06_part2_generate_str, aoc_2021_06_part2_naive};

fn bench_aoc_2021_06_part2(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_aoc_2021_06_part2");
    for len in [0, 1, 3, 10, 20, 50, 1000].iter() {
        let input = aoc_2021_06_part2_generate_str(*len);
        group.bench_with_input(BenchmarkId::new("naive", len), &input, |b, i| {
            b.iter(|| aoc_2021_06_part2_naive(i))
        });
        group.bench_with_input(BenchmarkId::new("lookup", len), &input, |b, i| {
            b.iter(|| aoc_2021_06_part2(i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_aoc_2021_06_part2);
criterion_main!(benches);
