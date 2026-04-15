use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use toto_engine::backend::MockBackend;
use toto_engine::exec::ExecCtx;

fn bench_sleep(c: &mut Criterion) {
    let cases: &[f64] = &[0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0];
    let mut group = c.benchmark_group("sleep");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(150);

    for &ms in cases {
        group.bench_with_input(BenchmarkId::new("ms", ms), &ms, |b, &ms| {
            b.iter_custom(|iters| {
                let (_tx, rx) = mpsc::channel::<()>();
                let mut backend = MockBackend::new();
                let ctx = ExecCtx::new(&mut backend, &rx);
                let d = Duration::from_secs_f64(ms / 1000.0);
                let t0 = Instant::now();
                for _ in 0..iters {
                    ctx.sleep(d);
                }
                t0.elapsed()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sleep);
criterion_main!(benches);
