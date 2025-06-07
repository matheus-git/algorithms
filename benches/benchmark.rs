use algorithms::rod_cutting::{rod_cutting, rod_cutting_naive};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_dummy(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rod Cutting");

    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20, 24, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80];
    let length = 20;

    group.bench_function("rod_dynamic", |b| b.iter(|| {
        rod_cutting(&prices, length)
    }));

    group.bench_function("rod_naive", |b| b.iter(|| {
        rod_cutting_naive(&prices, length)
    }));

    group.finish();
}

criterion_group!(benches, bench_dummy);
criterion_main!(benches);

