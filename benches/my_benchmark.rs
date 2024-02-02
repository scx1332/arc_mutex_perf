use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
fn fastrand_raw(n: u64) -> f64 {
    let mut r = fastrand::Rng::new();
    let mut next = 0.0;
    for _ in 0..n {
        next = r.f64();
    }
    next
}
fn fastrand_rc(n: u64) -> f64 {
    let r = Rc::new(RefCell::new(fastrand::Rng::new()));
    let mut next = 0.0;
    for _ in 0..n {
        next = r.borrow_mut().f64();
    }
    next
}
fn fastrand_arc(n: u64) -> f64 {
    let r = Arc::new(Mutex::new(fastrand::Rng::new()));
    let mut next = 0.0;
    for _ in 0..n {
        next = r.lock().unwrap().f64();
    }
    next
}
fn normal_rand(n: u64) -> f64 {
    let mut r = rand::thread_rng();
    let mut next = 0.0;
    for _ in 0..n {
        next = r.gen();
    }
    next
}

fn criterion_benchmark(c: &mut Criterion) {
    let iterations = 1000000;
    c.bench_function("normal_rand", |b| b.iter(|| normal_rand(iterations)));
    c.bench_function("fastrand_raw", |b| b.iter(|| fastrand_raw(iterations)));
    c.bench_function("fastrand_rc", |b| b.iter(|| fastrand_rc(iterations)));
    c.bench_function("fastrand_arc", |b| b.iter(|| fastrand_arc(iterations)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);