use arc_mutex_perf::mutex_not_send::MutexOnlySync;
use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

async fn fastrand_raw(n: u64) -> f64 {
    let mut r = fastrand::Rng::new();
    let mut next = 0.0;
    for _ in 0..n {
        next = r.f64();
    }
    next
}
async fn fastrand_rc(n: u64) -> f64 {
    let r = Rc::new(RefCell::new(fastrand::Rng::new()));
    let mut next = 0.0;
    for _ in 0..n {
        next = r.borrow_mut().f64();
    }
    next
}
async fn fastrand_arc(n: u64) -> f64 {
    let r = Arc::new(Mutex::new(fastrand::Rng::new()));
    let mut next = 0.0;
    for _ in 0..n {
        next = r.lock().unwrap().f64();
    }
    next
}
async fn fastrand_arc_tokio(n: u64) -> f64 {
    let r = Arc::new(tokio::sync::Mutex::new(fastrand::Rng::new()));
    let mut next = 0.0;
    for _ in 0..n {
        next = r.lock().await.f64();
    }
    next
}
async fn fastrand_arc_parking_lot(n: u64) -> f64 {
    let r = Arc::new(
        parking_lot::lock_api::Mutex::<parking_lot::RawMutex, _>::new(fastrand::Rng::new()),
    );
    let mut next = 0.0;
    for _ in 0..n {
        next = r.lock().f64();
    }
    next
}

async fn normal_rand(n: u64) -> f64 {
    let mut r = rand::thread_rng();
    let mut next = 0.0;
    for _ in 0..n {
        next = r.gen();
    }
    next
}

async fn normal_rand_with_await(n: u64) -> f64 {
    let mut r = rand::thread_rng();
    let mut next = 0.0;
    //let future = future::ready(());
    for _ in 0..n {
        tokio::task::yield_now().await;
        next = r.gen();
    }
    next
}

async fn primitive_lock(n: u64) -> f64 {
    let mut r = rand::thread_rng();
    let mut next = 0.0;
    let primitive_lock = Rc::new(MutexOnlySync::new());
    for _ in 0..n {
        primitive_lock.lock().await;
        next = r.gen();
        primitive_lock.unlock();
    }
    next
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random number generation");
    let iterations = 1000000;
    group.bench_function("rand", |b| {
        b.to_async(FuturesExecutor).iter(|| normal_rand(iterations))
    });
    group.bench_function("rand_fut", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| normal_rand_with_await(iterations))
    });
    group.bench_function("fast", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| fastrand_raw(iterations))
    });
    group.bench_function("rc", |b| {
        b.to_async(FuturesExecutor).iter(|| fastrand_rc(iterations))
    });
    group.bench_function("arc", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| fastrand_arc(iterations))
    });
    group.bench_function("arc_tokio", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| fastrand_arc_tokio(iterations))
    });
    group.bench_function("arc_parking", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| fastrand_arc_parking_lot(iterations))
    });
    group.bench_function("primitive_lock", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| primitive_lock(iterations))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
