use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_complex::Complex;
use rand::{thread_rng, Rng};

fn bench_sum(c: &mut Criterion) {
    let mut rng = thread_rng();
    let v: Vec<Complex<f32>> = (0..12800).map(|_| rng.gen()).collect();

    c.bench_function("sum_simd", |b| {
        b.iter(|| black_box(&v).iter().sum::<Complex<f32>>())
    });
    c.bench_function("sum_scalar", |b| {
        b.iter(|| black_box(&v).iter().fold(Complex::new(0.0, 0.0), |acc, x| acc + x))
    });
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
