use criterion::{black_box, criterion_group, criterion_main, Criterion};
use id_pool::IdPool;

pub fn request(c: &mut Criterion) {
    let mut pool = IdPool::new();
    c.bench_function("request", |b| b.iter(|| black_box(pool.request_id())));
}

pub fn request_return(c: &mut Criterion) {
    let mut pool = IdPool::new();
    c.bench_function("request_return", |b| {
        b.iter(|| {
            black_box({
                pool.request_id();
                pool.return_id(0).unwrap();
            })
        })
    });
}

// pub fn random(c: &mut Criterion) {
//     c.bench_function("random", |b| b.iter(|| fibonacci(black_box(20))));
// }

criterion_group!(benches, request, request_return);
criterion_main!(benches);
