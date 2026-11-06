//! Criterion benchmarks for task submission, channel contention, and cancellation latency in
//! `cancelable`.
//!
//! Measures:
//! - Task submission throughput and cancellation token registration across multiple threads.
//! - Channel contention under concurrent task requests and queue saturation.
//! - Cancellation latency for targeted task cancellation and broadcast `cancel_all` requests.

use std::hint::black_box;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::thread;

use cancelable::CancelTokenSource;
use cancelable::Task;
use cancelable::Worker;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

/// Benchmarks task creation and submission throughput into the worker queue.
fn benchmark_task_submission_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("task_submission");

    group.bench_function("single_thread_submission_and_drain", |b| {
        b.iter(|| {
            let mut worker = Worker::new();
            for _ in 0..black_box(5) {
                let task = Task::new();
                let _ = worker.start(task);
            }
            let _ = worker.die();
        });
    });

    group.bench_function("token_generation_throughput", |b| {
        let source = CancelTokenSource::new();
        let counter = AtomicU64::new(1);
        b.iter(|| {
            let id = counter.fetch_add(1, Ordering::Relaxed);
            let token = source.make_token(black_box(id)).expect("makes token");
            black_box(token);
        });
    });

    group.finish();
}

/// Spawns worker tasks in a separate helper function to avoid excessive nesting.
fn submit_worker_tasks(worker: &Worker) {
    for _ in 0..5 {
        let _ = worker.start(Task::new());
    }
}

/// Benchmarks channel contention and queue saturation under concurrent threads attempting to submit
/// tasks.
fn benchmark_channel_contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel_contention");

    group.bench_function("concurrent_submissions", |b| {
        b.iter(|| {
            let worker = Arc::new(Worker::new());
            let mut handles = Vec::with_capacity(4);

            for _ in 0..4 {
                let worker_clone = Arc::clone(&worker);
                handles.push(thread::spawn(move || {
                    submit_worker_tasks(&worker_clone);
                }));
            }

            for handle in handles {
                let _ = handle.join();
            }

            if let Ok(mut worker) = Arc::try_unwrap(worker) {
                let _ = worker.die();
            }
        });
    });

    group.finish();
}

/// Benchmarks cancellation latency: token checking, targeted task cancellation, and broadcasting
/// cancellation.
fn benchmark_cancellation_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("cancellation_latency");

    group.bench_function("token_check_uncancelled", |b| {
        let source = CancelTokenSource::new();
        let token = source.make_token(42).expect("token created");
        b.iter(|| {
            let check_outcome = black_box(&token).check();
            let _ = black_box(check_outcome);
        });
    });

    group.bench_function("targeted_cancellation_latency", |b| {
        let source = CancelTokenSource::new();
        let counter = AtomicU64::new(100);
        b.iter(|| {
            let id = counter.fetch_add(1, Ordering::Relaxed);
            let _token = source.make_token(id).expect("token created");
            let cancel_outcome = source.cancel_task(black_box(id));
            let _ = black_box(cancel_outcome);
        });
    });

    group.bench_function("broadcast_cancellation_latency", |b| {
        b.iter(|| {
            let source = CancelTokenSource::new();
            for i in 0..black_box(20) {
                let _ = source.make_token(i);
            }
            let broadcast_outcome = source.cancel_all();
            let _ = black_box(broadcast_outcome);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_task_submission_throughput,
    benchmark_channel_contention,
    benchmark_cancellation_latency
);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    /// Runs the cancelable benchmark entry point under test harness.
    #[test]
    fn test_cancelable_bench_entry_point() {
        let mut criterion = criterion::Criterion::default().configure_from_args();
        super::benchmark_task_submission_throughput(&mut criterion);
        super::benchmark_channel_contention(&mut criterion);
        super::benchmark_cancellation_latency(&mut criterion);
    }
}
