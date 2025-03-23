use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lr_percolation::realize;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn realize_l1(n: usize) -> () {
    let l = f32::sqrt(n as f32) as usize;
    let alpha = 1.0;
    let beta = 0.03;
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    realize(lr_percolation::Norm::L1, l, alpha, beta, &mut rng);
}

fn scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling l1");
    [8usize, 12, 16, 20, 24, 28, 32]
        .iter()
        .map(|i| i * i)
        .for_each(|n| {
            group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
                b.iter(|| realize_l1(n))
            });
        });
    group.finish()
}

criterion_group!(benches, scaling);
criterion_main!(benches);
