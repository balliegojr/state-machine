use criterion::{criterion_group, criterion_main, Criterion};

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("State Machine Comparison");
    group.bench_function("enum", |b| {
        b.iter(state_machine::internal_enum::run_full_state_machine)
    });

    group.bench_function("dyn trait", |b| {
        b.iter(state_machine::dyn_trait::run_full_state_machine)
    });

    group.bench_function("compose", |b| {
        b.iter(state_machine::compose_trait::run_full_state_machine)
    });

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
