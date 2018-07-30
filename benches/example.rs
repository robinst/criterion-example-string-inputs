#[macro_use]
extern crate criterion;

use criterion::{Bencher, Criterion};

fn example(b: &mut Bencher, s: &str) {
    b.iter(|| s.len());
}

fn example_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "fun",
        |b, s| example(b, s),
        vec![
            "foo",
            "bar"
        ]
    );
}

criterion_group!(benches, example_benchmark);
criterion_main!(benches);
