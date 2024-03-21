////////////////////////////////////////////////////////////////////////////////

extern crate recursion_lib;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

////////////////////////////////////////////////////////////////////////////////

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Exponents");
    for i in -10..10 {
        group.bench_with_input(BenchmarkId::new("Recursive", i), &i, |b, i| {
            b.iter(|| recursion_lib::expo::recursive(black_box(5.25), *i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), &i, |b, i| {
            b.iter(|| recursion_lib::expo::iterative(black_box(5.25), *i))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Int to binary");
    for i in -10..10 {
        group.bench_with_input(BenchmarkId::new("Recursive", i), &i, |b, i| {
            b.iter(|| recursion_lib::int_to_bin::recursive(*i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), &i, |b, i| {
            b.iter(|| recursion_lib::int_to_bin::iterative(*i))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("LCD");
    group.bench_with_input(BenchmarkId::new("Recursive", 1), &1, |b, i| {
        b.iter(|| {
            recursion_lib::lcd::recursive(black_box(recursion_lib::lcd::Expr::new(1, 20, 22)))
        })
    });
    group.bench_with_input(BenchmarkId::new("Iterative", 1), &1, |b, i| {
        b.iter(|| {
            recursion_lib::lcd::iterative(black_box(recursion_lib::lcd::Expr::new(1, 20, 22)))
        })
    });

    group.finish();
}

////////////////////////////////////////////////////////////////////////////////

criterion_group!(benches, benchmark);
criterion_main!(benches);

////////////////////////////////////////////////////////////////////////////////
