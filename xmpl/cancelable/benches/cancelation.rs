//! Criterion benchmarks for task submission, registry contention, and cancellation latency.
//!
//! TODO Reviewed ??-??-2026.

use std::hint::black_box;
use std::thread;

use cancelable::CancelTokenSource;
use cancelable::Task;
use cancelable::Worker;
use criterion::BatchSize;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;

/// Registers and drops tokens to measure the cost of the shared registry.
fn bench_token_registration(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_registration");
    for count in [1_u64, 100, 1_000] {
        group.throughput(Throughput::Elements(count));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let source = CancelTokenSource::new();
            b.iter(|| {
                // Each token is registered, then dropped (unregistered) immediately.
                for id in 0..count {
                    black_box(source.make_token(id).expect("ids are unique"));
                }
            });
        });
    }
    group.finish();
}

/// Measures how long `cancel_all` takes with many registered tasks, and how fast a token notices.
fn bench_cancellation_signal(c: &mut Criterion) {
    let mut group = c.benchmark_group("cancellation_signal");
    for count in [1_u64, 100, 1_000] {
        group.bench_with_input(
            BenchmarkId::new("cancel_all", count),
            &count,
            |b, &count| {
                b.iter_batched(
                    || {
                        let source = CancelTokenSource::new();
                        let tokens: Vec<_> = (0..count)
                            .map(|id| source.make_token(id).expect("ids are unique"))
                            .collect();
                        (source, tokens)
                    },
                    |(source, tokens)| {
                        source.cancel_all().expect("registry is healthy");
                        black_box(tokens)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    let source = CancelTokenSource::new();
    let token = source.make_token(0).expect("first id is unique");
    group.bench_function("check_not_cancelled", |b| {
        b.iter(|| black_box(token.check()));
    });
    source.cancel_task(0).expect("task is active");
    group.bench_function("check_cancelled", |b| {
        b.iter(|| black_box(token.check()));
    });
    group.finish();
}

/// Measures registry contention when several threads register and drop tokens concurrently.
fn bench_registry_contention(c: &mut Criterion) {
    const TOKENS_PER_THREAD: u64 = 200;
    let mut group = c.benchmark_group("registry_contention");
    for threads in [1_u64, 2, 4, 8] {
        group.throughput(Throughput::Elements(threads * TOKENS_PER_THREAD));
        group.bench_with_input(
            BenchmarkId::from_parameter(threads),
            &threads,
            |b, &threads| {
                let source = CancelTokenSource::new();
                b.iter(|| {
                    thread::scope(|scope| {
                        for thread_index in 0..threads {
                            let source = source.clone();
                            scope.spawn(move || {
                                let base = thread_index * TOKENS_PER_THREAD;
                                for id in base..base + TOKENS_PER_THREAD {
                                    black_box(source.make_token(id).expect("ids are unique"));
                                }
                            });
                        }
                    });
                });
            },
        );
    }
    group.finish();
}

/// Measures filling the worker queue and the time until a cancelled worker has fully stopped.
fn bench_worker_submission_and_cancel(c: &mut Criterion) {
    // Matches the worker's queue capacity.
    const QUEUED: u64 = 5;
    let mut group = c.benchmark_group("worker");
    group.sample_size(10);
    group.throughput(Throughput::Elements(QUEUED));
    group.bench_function("submit_then_cancel_and_stop", |b| {
        b.iter(|| {
            let mut worker = Worker::new();
            for _ in 0..QUEUED {
                worker.start(Task::new()).expect("queue has room");
            }
            // `die` cancels every task and waits for the worker thread to stop.
            worker.die().expect("worker stops cleanly");
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_token_registration,
    bench_cancellation_signal,
    bench_registry_contention,
    bench_worker_submission_and_cancel
);
criterion_main!(benches);
