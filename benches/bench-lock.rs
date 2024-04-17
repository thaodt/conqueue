use conqueue::{ConcurrentQueue, LockQueue};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_concurrent_queue(c: &mut Criterion) {
    let queue = ConcurrentQueue::new();
    c.bench_function("concurrent_queue", |b| {
        b.iter(|| {
            queue.enqueue(black_box(1000000));
            queue.dequeue();
        })
    });
}

fn bench_lock_queue(c: &mut Criterion) {
    let queue = LockQueue::new();
    c.bench_function("lock_queue", |b| {
        b.iter(|| {
            queue.enqueue(black_box(1000000));
            queue.dequeue();
        })
    });
}

/*fn multiple_bench(c: &mut Criterion) {
    let inputs = [100, 1000, 10000, 100000, 1000000];
    let mut group = c.benchmark_group("Multiple");
    let queue = ConcurrentQueue::new();
    let queue1 = LockQueue::new();

    for i in inputs {
        group.bench_with_input(BenchmarkId::new("concurrent queue", i), &i, |b, &i| {
            b.iter(|| {
                queue.enqueue(black_box(i));
                queue.dequeue();
            })
        });

        group.bench_with_input(BenchmarkId::new("lock-based queue", i), &i, |b, &i| {
            b.iter(|| {
                queue1.enqueue(black_box(i));
                queue1.dequeue();
            })
        });
    }
    group.finish();
}*/

criterion_group!(
    benches,
    bench_concurrent_queue,
    bench_lock_queue,
    //multiple_bench
);
criterion_main!(benches);
