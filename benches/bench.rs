use criterion::{criterion_group, criterion_main, Criterion};
use sinteflake::sinteflake::SINTEFlake;

fn sinteflake_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("init");

    group.bench_function("init_10", |b| {
        b.iter(|| {
            let mut instance = SINTEFlake::new().unwrap();
            for _ in 0..10 {
                instance.next_id().unwrap();
            }
        });
    });

    group.bench_function("init_100", |b| {
        b.iter(|| {
            let mut instance = SINTEFlake::new().unwrap();
            for _ in 0..100 {
                instance.next_id().unwrap();
            }
        });
    });

    group.bench_function("init_1000", |b| {
        b.iter(|| {
            let mut instance = SINTEFlake::new().unwrap();
            for _ in 0..1000 {
                instance.next_id().unwrap();
            }
        });
    });

    group.bench_function("init_10000", |b| {
        b.iter(|| {
            let mut instance = SINTEFlake::new().unwrap();
            for _ in 0..10000 {
                instance.next_id().unwrap();
            }
        });
    });

    group.bench_function("init_100000", |b| {
        b.iter(|| {
            let mut instance = SINTEFlake::new().unwrap();
            for _ in 0..100000 {
                instance.next_id().unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, sinteflake_bench);
criterion_main!(benches);
