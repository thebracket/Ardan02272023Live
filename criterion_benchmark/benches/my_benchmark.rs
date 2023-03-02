use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    c.bench_function("is_prime",
        |b| b.iter(|| {
            black_box(is_prime(black_box(500_000)))
        })
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
