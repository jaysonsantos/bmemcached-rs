#[macro_use]
extern crate criterion;
extern crate bmemcached;

use std::sync::Arc;

use criterion::Criterion;

use bmemcached::MemcachedClient;

fn criterion_benchmark(c: &mut Criterion) {
    let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211"], 10).unwrap());
    let cli = client.clone();
    c.bench_function("set/get/delete", move |b| {
        b.iter(|| {
            let key = "benchmark test";
            cli.set(key, "abc", 10_000).unwrap();
            let returned_value: String = cli.get(key).unwrap();
            assert_eq!(returned_value, "abc");
            cli.delete(key).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
