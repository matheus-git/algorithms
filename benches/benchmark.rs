use algorithms::rod_cutting::{rod_cutting, rod_cutting_naive};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_dummy(c: &mut Criterion) {
    let prices: Vec<i32> = (1..=40).map(|x| x * 2).collect();
    let length = 40;

    let mut group = c.benchmark_group("Rod Cutting");

    group.bench_function("rod_dynamic", |b| {
        b.iter(|| {
            rod_cutting(&prices, length)
        })
    });

    group.bench_function("rod_naive", |b| {
        b.iter(|| {
            rod_cutting_naive(&prices, length)
        })
    });

    group.finish();
}


criterion_group!(benches, bench_dummy);
criterion_main!(benches);

